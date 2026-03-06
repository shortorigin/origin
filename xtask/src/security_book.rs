use std::fs;
use std::path::{Component, Path, PathBuf};

use regex::Regex;
use serde::Deserialize;

use crate::{ensure_command_available, run_compile_plan, run_plan, workspace_root, CommandPlan};

const BOOK_RELATIVE_ROOT: &str = "docs/security-rust/book";
const EXAMPLE_PACKAGES: &[&str] = &[
    "security-example-wasm-sandboxing",
    "security-example-network-security",
    "security-example-distributed-security",
    "security-example-infrastructure-hardening",
];
const NEXTEST_PACKAGES: &[&str] = &[
    "secure-patterns",
    "exploit-mitigation",
    "runtime-security",
    "security-instrumentation",
    "runtime-security-labs",
];

#[derive(Debug, Deserialize)]
struct VerificationConfig {
    #[serde(default)]
    chapters: Vec<ChapterVerification>,
}

#[derive(Debug, Deserialize)]
struct ChapterVerification {
    page: String,
    #[serde(default)]
    commands: Vec<String>,
    #[serde(default)]
    artifacts: Vec<String>,
    #[serde(default)]
    examples: Vec<String>,
    #[serde(default)]
    research: Vec<String>,
}

pub fn run(args: Vec<String>) -> Result<(), String> {
    let (subcommand, passthrough) = args
        .split_first()
        .ok_or_else(|| "expected `security-book <build|serve|test>`".to_string())?;

    match subcommand.as_str() {
        "build" => build_book(passthrough),
        "serve" => preview_book(passthrough),
        "test" => test_book(),
        other => Err(format!("unsupported security-book subcommand `{other}`")),
    }
}

pub fn build_plan(book_root: &Path, passthrough: &[String]) -> CommandPlan {
    CommandPlan::new("mdbook", book_root)
        .arg("build")
        .args(passthrough.to_vec())
}

pub fn preview_plan(book_root: &Path, passthrough: &[String]) -> CommandPlan {
    CommandPlan::new("mdbook", book_root)
        .arg("serve")
        .args(passthrough.to_vec())
}

pub fn build_book(passthrough: &[String]) -> Result<(), String> {
    ensure_mdbook()?;
    let book_root = book_root()?;
    run_plan(&build_plan(&book_root, passthrough))
}

pub fn preview_book(passthrough: &[String]) -> Result<(), String> {
    ensure_mdbook()?;
    let book_root = book_root()?;
    run_plan(&preview_plan(&book_root, passthrough))
}

pub fn test_book() -> Result<(), String> {
    let workspace_root = workspace_root()?;
    let book_root = workspace_root.join(BOOK_RELATIVE_ROOT);
    test_book_internal(&workspace_root, &book_root)
}

fn test_book_internal(workspace_root: &Path, book_root: &Path) -> Result<(), String> {
    ensure_mdbook()?;
    ensure_cargo_nextest()?;

    run_plan(&build_plan(book_root, &[]))?;
    validate_markdown_links(&book_root.join("src"))?;
    validate_chapter_references(workspace_root, book_root)?;

    cargo_nextest(workspace_root)?;
    cargo_check_examples(workspace_root)?;
    cargo_targeted_runtime_checks(workspace_root)?;
    Ok(())
}

fn ensure_mdbook() -> Result<(), String> {
    ensure_command_available(
        "mdbook",
        &["--version"],
        "mdBook is required. Install it with `cargo install mdbook --locked`.",
    )
}

fn ensure_cargo_nextest() -> Result<(), String> {
    ensure_command_available(
        "cargo",
        &["nextest", "--version"],
        "cargo-nextest is required. Install it with `cargo install cargo-nextest --locked`.",
    )
}

fn cargo_nextest(workspace_root: &Path) -> Result<(), String> {
    let mut plan = CommandPlan::new("cargo", workspace_root)
        .arg("nextest")
        .arg("run");
    for package in NEXTEST_PACKAGES {
        plan.push_arg("-p");
        plan.push_arg(*package);
    }
    run_compile_plan(plan)
}

fn cargo_check_examples(workspace_root: &Path) -> Result<(), String> {
    let mut plan = CommandPlan::new("cargo", workspace_root).arg("check");
    for package in EXAMPLE_PACKAGES {
        plan.push_arg("-p");
        plan.push_arg(*package);
    }
    run_compile_plan(plan)
}

fn cargo_targeted_runtime_checks(workspace_root: &Path) -> Result<(), String> {
    run_compile_plan(CommandPlan::new("cargo", workspace_root).args([
        "test",
        "-p",
        "strategy-sandbox",
        "-p",
        "wasmcloud-smoke-tests",
        "-p",
        "surrealdb-access",
    ]))
}

fn validate_markdown_links(book_src: &Path) -> Result<(), String> {
    let mut markdown_files = Vec::new();
    collect_markdown_files(book_src, &mut markdown_files)?;
    let link_pattern =
        Regex::new(r"!?\[[^\]]+\]\(([^)\s]+)\)").map_err(|error| error.to_string())?;

    for file in markdown_files {
        let content = fs::read_to_string(&file)
            .map_err(|error| format!("failed to read {}: {error}", file.display()))?;
        let mut fenced_code = false;
        for line in content.lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("```") {
                fenced_code = !fenced_code;
                continue;
            }
            if fenced_code {
                continue;
            }

            for capture in link_pattern.captures_iter(line) {
                let Some(target_match) = capture.get(1) else {
                    continue;
                };
                let target = target_match.as_str();
                if should_skip_link(target) {
                    continue;
                }
                let target_path = strip_anchor(target);
                if target_path.is_empty() {
                    continue;
                }

                let resolved = normalize_path(&file.parent().unwrap_or(book_src).join(target_path));
                if !resolved.exists() {
                    return Err(format!(
                        "broken markdown link `{target}` referenced from {}",
                        file.display()
                    ));
                }
            }
        }
    }

    Ok(())
}

fn validate_chapter_references(workspace_root: &Path, book_root: &Path) -> Result<(), String> {
    let manifest_path = book_root.join("verification.toml");
    let raw = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("failed to read {}: {error}", manifest_path.display()))?;
    let config: VerificationConfig =
        toml::from_str(&raw).map_err(|error| format!("invalid verification.toml: {error}"))?;

    for chapter in config.chapters {
        let page_path = book_root.join(&chapter.page);
        if !page_path.exists() {
            return Err(format!(
                "verification entry points to a missing page: {}",
                page_path.display()
            ));
        }
        let content = fs::read_to_string(&page_path)
            .map_err(|error| format!("failed to read {}: {error}", page_path.display()))?;

        for command in chapter.commands {
            if command.trim().is_empty() || !content.contains(&command) {
                return Err(format!(
                    "page {} is missing the documented command `{command}`",
                    page_path.display()
                ));
            }
        }

        for path in chapter
            .artifacts
            .iter()
            .chain(chapter.examples.iter())
            .chain(chapter.research.iter())
        {
            let repo_path = workspace_root.join(path);
            if !repo_path.exists() {
                return Err(format!(
                    "page {} references a missing artifact `{path}`",
                    page_path.display()
                ));
            }
            if !content.contains(path) {
                return Err(format!(
                    "page {} does not mention the required path `{path}`",
                    page_path.display()
                ));
            }
        }
    }

    Ok(())
}

fn collect_markdown_files(root: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to read directory {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_files(&path, output)?;
        } else if path.extension().is_some_and(|extension| extension == "md") {
            output.push(path);
        }
    }
    Ok(())
}

fn should_skip_link(target: &str) -> bool {
    target.starts_with('#')
        || target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with("mailto:")
        || target.starts_with("data:")
}

fn strip_anchor(target: &str) -> &str {
    target.split('#').next().unwrap_or(target)
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}

fn book_root() -> Result<PathBuf, String> {
    Ok(workspace_root()?.join(BOOK_RELATIVE_ROOT))
}

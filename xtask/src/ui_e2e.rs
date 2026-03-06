use std::path::{Path, PathBuf};

use clap::ValueEnum;

use crate::{run_plan, workspace_root, CommandPlan};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum UiE2eScene {
    ShellDefault,
    SettingsAppearance,
    SettingsAccessibility,
    ShellHighContrast,
    TerminalDefault,
}

impl UiE2eScene {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::ShellDefault => "shell-default",
            Self::SettingsAppearance => "settings-appearance",
            Self::SettingsAccessibility => "settings-accessibility",
            Self::ShellHighContrast => "shell-high-contrast",
            Self::TerminalDefault => "terminal-default",
        }
    }
}

#[must_use]
pub fn e2e_workspace_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join("ui/e2e")
}

#[must_use]
pub fn npm_ci_plan(workspace_root: &Path) -> CommandPlan {
    CommandPlan::new("npm", e2e_workspace_dir(workspace_root)).arg("ci")
}

#[must_use]
pub fn browser_install_plan(workspace_root: &Path, ci: bool) -> CommandPlan {
    let mut plan = CommandPlan::new("npm", e2e_workspace_dir(workspace_root)).args([
        "exec",
        "playwright",
        "install",
    ]);
    if ci {
        plan.push_arg("--with-deps");
    }
    plan.arg("chromium")
}

#[must_use]
pub fn playwright_test_plan(workspace_root: &Path, scene: Option<UiE2eScene>) -> CommandPlan {
    let mut plan = CommandPlan::new("npm", e2e_workspace_dir(workspace_root)).args([
        "exec",
        "playwright",
        "test",
    ]);
    if let Some(scene) = scene {
        plan.set_env("SHORT_ORIGIN_E2E_SCENE", scene.id());
    }
    plan
}

pub fn run(scene: Option<UiE2eScene>) -> Result<(), String> {
    let root = workspace_root()?;
    run_plan(&npm_ci_plan(&root))?;
    run_plan(&browser_install_plan(
        &root,
        std::env::var_os("CI").is_some(),
    ))?;
    run_plan(&playwright_test_plan(&root, scene))
}

#[cfg(test)]
mod tests {
    use super::{browser_install_plan, npm_ci_plan, playwright_test_plan, UiE2eScene};
    use std::path::Path;

    #[test]
    fn browser_install_plan_uses_ci_deps_flag() {
        let ci = browser_install_plan(Path::new("/repo"), true);
        let local = browser_install_plan(Path::new("/repo"), false);
        assert_eq!(
            ci.display(),
            "npm exec playwright install --with-deps chromium"
        );
        assert_eq!(local.display(), "npm exec playwright install chromium");
    }

    #[test]
    fn npm_ci_plan_installs_ui_e2e_dependencies() {
        let plan = npm_ci_plan(Path::new("/repo"));
        assert_eq!(plan.display(), "npm ci");
    }

    #[test]
    fn playwright_test_plan_sets_scene_environment() {
        let plan = playwright_test_plan(Path::new("/repo"), Some(UiE2eScene::TerminalDefault));
        assert_eq!(plan.display(), "npm exec playwright test");
        assert_eq!(
            plan.env.get("SHORT_ORIGIN_E2E_SCENE").map(String::as_str),
            Some("terminal-default")
        );
    }
}

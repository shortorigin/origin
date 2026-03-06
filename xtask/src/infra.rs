use std::path::{Path, PathBuf};

use clap::ValueEnum;

use crate::{run_plan, workspace_root, CommandPlan};

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum InfraStack {
    Dev,
    Stage,
    Prod,
}

impl InfraStack {
    #[must_use]
    pub const fn script_suffix(self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Stage => "stage",
            Self::Prod => "prod",
        }
    }
}

#[must_use]
pub fn pulumi_workspace_dir(workspace_root: &Path) -> PathBuf {
    workspace_root.join("infrastructure/pulumi")
}

#[must_use]
pub fn npm_ci_plan(workspace_root: &Path) -> CommandPlan {
    CommandPlan::new("npm", pulumi_workspace_dir(workspace_root)).arg("ci")
}

#[must_use]
pub fn verify_plans(workspace_root: &Path) -> Vec<CommandPlan> {
    let pulumi_dir = pulumi_workspace_dir(workspace_root);
    vec![
        npm_ci_plan(workspace_root),
        CommandPlan::new("npm", &pulumi_dir).arg("test"),
        CommandPlan::new("npm", pulumi_dir).args(["--workspace", "live", "run", "lint"]),
    ]
}

#[must_use]
pub fn preview_plan(workspace_root: &Path, stack: InfraStack) -> CommandPlan {
    CommandPlan::new("npm", pulumi_workspace_dir(workspace_root)).args([
        "--workspace",
        "live",
        "run",
        &format!("preview:{}", stack.script_suffix()),
    ])
}

pub fn verify() -> Result<(), String> {
    let root = workspace_root()?;
    for plan in verify_plans(&root) {
        run_plan(&plan)?;
    }
    Ok(())
}

pub fn preview(stack: InfraStack) -> Result<(), String> {
    let root = workspace_root()?;
    run_plan(&npm_ci_plan(&root))?;
    run_plan(&preview_plan(&root, stack))
}

#[cfg(test)]
mod tests {
    use super::{preview_plan, verify_plans, InfraStack};
    use std::path::Path;

    #[test]
    fn verify_plans_cover_ci_commands() {
        let plans = verify_plans(Path::new("/repo"));
        let displays = plans
            .into_iter()
            .map(|plan| plan.display())
            .collect::<Vec<_>>();
        assert_eq!(
            displays,
            vec![
                "npm ci".to_string(),
                "npm test".to_string(),
                "npm --workspace live run lint".to_string(),
            ]
        );
    }

    #[test]
    fn preview_plan_targets_selected_stack() {
        let plan = preview_plan(Path::new("/repo"), InfraStack::Stage);
        assert_eq!(plan.display(), "npm --workspace live run preview:stage");
    }
}

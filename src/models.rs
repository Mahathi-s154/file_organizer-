use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MoveAction {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub category: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionStatus {
    Planned,
    Moved,
    CopiedAcrossFilesystems,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionReport {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub category: String,
    pub status: ActionStatus,
}

impl ActionReport {
    pub fn from_action(action: &MoveAction, status: ActionStatus) -> Self {
        Self {
            source: action.source.clone(),
            destination: action.destination.clone(),
            category: action.category.clone(),
            status,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct RunSummary {
    pub planned: usize,
    pub moved: usize,
    pub copied_across_filesystems: usize,
}

impl RunSummary {
    pub fn from_actions(actions: &[ActionReport]) -> Self {
        let mut summary = Self::default();

        for action in actions {
            match action.status {
                ActionStatus::Planned => {
                    summary.planned += 1;
                }
                ActionStatus::Moved => {
                    summary.moved += 1;
                }
                ActionStatus::CopiedAcrossFilesystems => {
                    summary.moved += 1;
                    summary.copied_across_filesystems += 1;
                }
            }
        }

        summary
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RunReport {
    pub target_dir: PathBuf,
    pub dry_run: bool,
    pub recursive: bool,
    pub action_count: usize,
    pub summary: RunSummary,
    pub actions: Vec<ActionReport>,
}

impl RunReport {
    pub fn new(
        target_dir: PathBuf,
        dry_run: bool,
        recursive: bool,
        actions: Vec<ActionReport>,
    ) -> Self {
        let summary = RunSummary::from_actions(&actions);

        Self {
            target_dir,
            dry_run,
            recursive,
            action_count: actions.len(),
            summary,
            actions,
        }
    }
}

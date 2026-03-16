use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use colored::Colorize;
use walkdir::WalkDir;

use crate::cli::Args;
use crate::models::MoveAction;

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(OsStr::to_str)
        .is_some_and(|name| name.starts_with('.'))
}

fn extension_category(path: &Path) -> String {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.to_lowercase())
        .unwrap_or_else(|| "misc".to_string())
}

fn resolve_collision(dest: &Path, claimed: &HashSet<PathBuf>) -> PathBuf {
    if !dest.exists() && !claimed.contains(dest) {
        return dest.to_path_buf();
    }

    let stem = dest.file_stem().unwrap_or_default().to_string_lossy();
    let ext = dest
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let parent = dest.parent().unwrap_or_else(|| Path::new(""));

    let mut counter = 1u32;
    loop {
        let candidate = parent.join(format!("{stem} ({counter}){ext}"));
        if !candidate.exists() && !claimed.contains(&candidate) {
            return candidate;
        }
        counter += 1;
    }
}

pub fn build_plan(args: &Args) -> anyhow::Result<Vec<MoveAction>> {
    let target = &args.target_dir;
    let max_depth = if args.recursive { usize::MAX } else { 1 };

    let walker = WalkDir::new(target)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !is_hidden(e.path()));

    let mut actions = Vec::new();
    let mut claimed: HashSet<PathBuf> = HashSet::new();

    for entry in walker {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path == target || entry.file_type().is_dir() {
            continue;
        }

        let category = extension_category(path);

        // Skip files already sitting in their correct category subfolder.
        if path.parent() == Some(target.join(&category).as_path()) {
            continue;
        }

        let file_name = path.file_name().context("File entry has no file name")?;
        let desired = target.join(&category).join(file_name);
        let destination = resolve_collision(&desired, &claimed);
        claimed.insert(destination.clone());

        actions.push(MoveAction {
            source: path.to_path_buf(),
            destination,
        });
    }

    Ok(actions)
}

pub fn execute_plan(plan: Vec<MoveAction>, dry_run: bool) -> anyhow::Result<()> {
    for action in &plan {
        if dry_run {
            println!(
                "{}",
                format!(
                    "[DRY RUN] Would move: {} -> {}",
                    action.source.display(),
                    action.destination.display()
                )
                .yellow()
            );
        } else {
            let dest_parent = action
                .destination
                .parent()
                .context("Destination has no parent directory")?;

            fs::create_dir_all(dest_parent)
                .with_context(|| format!("Failed to create directory {}", dest_parent.display()))?;

            fs::rename(&action.source, &action.destination).with_context(|| {
                format!(
                    "Failed to move {} -> {}",
                    action.source.display(),
                    action.destination.display()
                )
            })?;

            println!(
                "{}",
                format!(
                    "Moved: {} -> {}",
                    action.source.display(),
                    action.destination.display()
                )
                .green()
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_args(dir: &Path, recursive: bool) -> Args {
        Args {
            target_dir: dir.to_path_buf(),
            dry_run: false,
            recursive,
        }
    }

    #[test]
    fn test_build_plan_basic() -> anyhow::Result<()> {
        let tmp = TempDir::new()?;
        let base = tmp.path();

        fs::write(base.join("test.txt"), "hello")?;
        fs::write(base.join("image.png"), "img")?;
        fs::write(base.join("noext"), "data")?;

        let args = make_args(base, false);
        let mut plan = build_plan(&args)?;
        plan.sort_by(|a, b| a.source.cmp(&b.source));

        assert_eq!(plan.len(), 3);

        let expected: Vec<(String, String)> = plan
            .iter()
            .map(|a| {
                let src = a.source.file_name().unwrap().to_string_lossy().to_string();
                let dst_dir = a
                    .destination
                    .parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                (src, dst_dir)
            })
            .collect();

        assert!(expected.contains(&("image.png".into(), "png".into())));
        assert!(expected.contains(&("test.txt".into(), "txt".into())));
        assert!(expected.contains(&("noext".into(), "misc".into())));

        Ok(())
    }

    #[test]
    fn test_build_plan_skips_hidden() -> anyhow::Result<()> {
        let tmp = TempDir::new()?;
        let base = tmp.path();

        fs::write(base.join(".hidden"), "secret")?;
        fs::write(base.join("visible.txt"), "hi")?;

        let args = make_args(base, false);
        let plan = build_plan(&args)?;

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].source.file_name().unwrap().to_string_lossy(),
            "visible.txt"
        );

        Ok(())
    }

    #[test]
    fn test_build_plan_non_recursive() -> anyhow::Result<()> {
        let tmp = TempDir::new()?;
        let base = tmp.path();

        let subdir = base.join("subdir");
        fs::create_dir(&subdir)?;
        fs::write(subdir.join("deep.txt"), "deep")?;
        fs::write(base.join("top.txt"), "top")?;

        let args = make_args(base, false);
        let plan = build_plan(&args)?;

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].source.file_name().unwrap().to_string_lossy(),
            "top.txt"
        );

        Ok(())
    }

    #[test]
    fn test_build_plan_recursive() -> anyhow::Result<()> {
        let tmp = TempDir::new()?;
        let base = tmp.path();

        let subdir = base.join("subdir");
        fs::create_dir(&subdir)?;
        fs::write(subdir.join("deep.txt"), "deep")?;
        fs::write(base.join("top.txt"), "top")?;

        let args = make_args(base, true);
        let plan = build_plan(&args)?;

        assert_eq!(plan.len(), 2);

        Ok(())
    }

    #[test]
    fn test_build_plan_skips_already_organized() -> anyhow::Result<()> {
        let tmp = TempDir::new()?;
        let base = tmp.path();

        let txt_dir = base.join("txt");
        fs::create_dir(&txt_dir)?;
        fs::write(txt_dir.join("already.txt"), "already organized")?;
        fs::write(base.join("new.txt"), "new")?;

        let args = make_args(base, true);
        let plan = build_plan(&args)?;

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].source.file_name().unwrap().to_string_lossy(),
            "new.txt"
        );

        Ok(())
    }

    #[test]
    fn test_collision_on_disk() -> anyhow::Result<()> {
        let tmp = TempDir::new()?;
        let base = tmp.path();

        // Pre-create the destination so it collides on disk.
        let txt_dir = base.join("txt");
        fs::create_dir(&txt_dir)?;
        fs::write(txt_dir.join("report.txt"), "old")?;

        fs::write(base.join("report.txt"), "new")?;

        let args = make_args(base, false);
        let plan = build_plan(&args)?;

        assert_eq!(plan.len(), 1);
        assert_eq!(
            plan[0].destination.file_name().unwrap().to_string_lossy(),
            "report (1).txt"
        );

        Ok(())
    }

    #[test]
    fn test_collision_within_plan() -> anyhow::Result<()> {
        let tmp = TempDir::new()?;
        let base = tmp.path();

        // Two files with the same name in different subdirectories,
        // both destined for txt/.
        let sub_a = base.join("a");
        let sub_b = base.join("b");
        fs::create_dir(&sub_a)?;
        fs::create_dir(&sub_b)?;
        fs::write(sub_a.join("notes.txt"), "a")?;
        fs::write(sub_b.join("notes.txt"), "b")?;

        let args = make_args(base, true);
        let plan = build_plan(&args)?;

        assert_eq!(plan.len(), 2);

        let dest_names: Vec<String> = plan
            .iter()
            .map(|a| {
                a.destination
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
            })
            .collect();

        assert!(dest_names.contains(&"notes.txt".to_string()));
        assert!(dest_names.contains(&"notes (1).txt".to_string()));

        Ok(())
    }
}

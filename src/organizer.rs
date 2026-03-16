use std::ffi::OsStr;
use std::path::Path;

use anyhow::Context;
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

pub fn build_plan(args: &Args) -> anyhow::Result<Vec<MoveAction>> {
    let target = &args.target_dir;
    let max_depth = if args.recursive { usize::MAX } else { 1 };

    let walker = WalkDir::new(target)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|e| e.depth() == 0 || !is_hidden(e.path()));

    let mut actions = Vec::new();

    for entry in walker {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path == target || entry.file_type().is_dir() {
            continue;
        }

        let category = extension_category(path);

        let file_name = path.file_name().context("File entry has no file name")?;

        let destination = target.join(&category).join(file_name);

        // Skip files already sitting in their correct category subfolder.
        if path.parent() == Some(target.join(&category).as_path()) {
            continue;
        }

        actions.push(MoveAction {
            source: path.to_path_buf(),
            destination,
        });
    }

    Ok(actions)
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
}

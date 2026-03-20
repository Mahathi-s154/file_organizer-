use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use tempfile::TempDir;

#[test]
fn dry_run_json_reports_planned_moves() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = TempDir::new()?;
    let base = tmp.path();
    fs::write(base.join("notes.txt"), "hello")?;

    let output = Command::cargo_bin("file_organizer")?
        .args([
            "--target-dir",
            &base.to_string_lossy(),
            "--dry-run",
            "--format",
            "json",
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let report: Value = serde_json::from_slice(&output)?;
    let destination = report["actions"][0]["destination"]
        .as_str()
        .expect("destination should be a string");

    assert_eq!(report["dry_run"], true);
    assert_eq!(report["action_count"], 1);
    assert_eq!(report["summary"]["planned"], 1);
    assert_eq!(report["summary"]["moved"], 0);
    assert_eq!(report["actions"][0]["status"], "planned");
    assert!(
        destination.ends_with("/txt/notes.txt") || destination.ends_with("\\txt\\notes.txt"),
        "unexpected destination: {destination}"
    );

    Ok(())
}

#[test]
fn json_execution_moves_file_and_reports_result() -> Result<(), Box<dyn std::error::Error>> {
    let tmp = TempDir::new()?;
    let base = tmp.path();
    fs::write(base.join("photo.png"), "img")?;

    let output = Command::cargo_bin("file_organizer")?
        .args(["--target-dir", &base.to_string_lossy(), "--format", "json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let report: Value = serde_json::from_slice(&output)?;

    assert_eq!(report["dry_run"], false);
    assert_eq!(report["action_count"], 1);
    assert_eq!(report["summary"]["moved"], 1);
    assert_eq!(report["summary"]["copied_across_filesystems"], 0);
    assert_eq!(report["actions"][0]["status"], "moved");
    assert!(base.join("png").join("photo.png").exists());
    assert!(!base.join("photo.png").exists());

    Ok(())
}

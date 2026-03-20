# Roadmap

`file_organizer` started as a focused extension-based sorter. The next step is to evolve it into a safer and more configurable file organization engine without losing the simplicity of the current CLI.

## Current Baseline

- Organize files into extension-named folders
- Dry-run previews
- Recursive scanning
- Hidden-file skipping
- Collision-safe renaming
- npm packaging and release automation

## Near-Term Priorities

- Rule and config system using a project-level config file
- Cleaner separation between scanning, planning, execution, and reporting
- Stronger cross-platform handling for hidden files, symlinks, and filesystem boundaries
- JSON reporting suitable for scripts and editor integrations
- PR-focused CI for formatting, linting, and tests

## Medium-Term Goals

- Undo journals for reverting a completed run
- Preserve-tree and per-directory organization modes
- Include and exclude filters
- Larger integration test coverage and benchmark suites

## Stretch Ideas

- Watch mode for automatically organizing a directory over time
- Terminal preview UI for inspecting a plan before applying it

## Non-Goals For Now

- GUI-first workflows
- Cloud or remote storage integrations
- Large plugin systems before the core planner is stable

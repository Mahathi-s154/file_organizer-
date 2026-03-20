# Design Notes

## Project Direction

The project is moving from a single-purpose extension sorter toward a safe, scriptable file organization engine. That means correctness and predictable behavior matter more than adding a large number of flags quickly.

## Current Pipeline

1. Scan the target directory tree.
2. Classify each file into a category.
3. Build a collision-safe move plan.
4. Execute the plan or preview it.
5. Report the outcome in text or JSON form.

## Core Invariants

- Dry runs must never mutate the filesystem.
- The planner must never intentionally overwrite an existing file.
- Collision resolution must be deterministic within a run.
- Files already in their correct destination should be skipped.
- Execution should degrade gracefully when a rename crosses filesystems.

## Existing Modules

- `src/cli.rs`: argument parsing and output selection
- `src/organizer.rs`: scanning, planning, execution, and tests
- `src/models.rs`: plan and report data models

## Target Architecture

The current codebase is intentionally small, but the longer-term shape should become more explicit:

- `scanner`: directory walking and hidden-file policy
- `classifier`: extension, MIME, or rule-based categorization
- `planner`: target path generation and collision resolution
- `executor`: filesystem mutations and recovery paths
- `reporter`: human-readable and machine-readable output
- `journal`: future undo and audit support

## Next High-Signal Improvements

- Introduce config-backed rules without coupling them to CLI parsing.
- Add richer reporting and integration tests around the binary.
- Expand cross-platform behavior coverage in CI.

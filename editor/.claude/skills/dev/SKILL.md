---
name: dev
description: Pick and Complete Next Task
---
1. Pick up a ticket to work on
    * If one is provided either with a ticket ID or description, locate that ticket.
    * If no ticket is provided, choose one by running `nbd next`
2. Implement the changes across all necessary files
3. Validate changes
    * Run `cargo fmt` to ensure code looks good
    * Run `cargo clippy` to ensure it follows best practices
    * Run `cargo check` to verify compilation
    * Run `cargo test` if tests pass
5. Mark the task as complete
6. Update PLANNING.md to reflect the status of the project and what was done in this working session.
7. If any follow-up work needs to be done, add it to PLANNING.md and create additional nbd tickets
8. Commit the changes including `.nbd/tickets/*` with a descriptive message referencing the task ID

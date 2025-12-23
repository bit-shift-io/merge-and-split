---
description: Process tasks from the .agent/tasks directory
---

This workflow defines the lifecycle for managing and implementing tasks located in the `.agent/tasks` directory.

### Process Steps

1. **Plan Generation**:
   - Review any non-reviewed tasks in `.agent/tasks`.
   - Take the first incomplete task file (e.g., `task 1.txt`).
   - Create a corresponding plan file: `.agent/tasks/[task-name].plan-for-review.md`.
   - This plan should outline the implementation strategy for the task.
   - Repeat until all tasks have a `.plan-for-review.md`.

2. **User Review**:
   - Wait for the user to approve the plan.
   - Approval is indicated by renaming `.plan-for-review.md` to `.plan-ok.md`.

3. **Implementation**:
   - Once a task has a `.plan-ok.md` file, implement the task according to the plan.

4. **Walkthrough**:
   - Upon completion, create a walkthrough file summarizing the changes.
   - Path: `.agent/tasks/[task-name].walkthrough.md`.

6. **Code Style Requirement**:
   - `mod.rs` files must ONLY contain `pub mod` lines.
   - All other code (structs, enums, impl blocks, logic) MUST be moved to separate module files.

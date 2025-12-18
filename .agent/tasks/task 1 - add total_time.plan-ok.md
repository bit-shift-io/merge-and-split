# Task 1: Add total_time to Game UI

Add a `total_time` display to the game UI and ensure it stops updating when the game ends.

## Proposed Changes

### [game]

#### [MODIFY] [ui.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/ui.rs)
- Add `total_time: f32` to `GameUI` struct.
- Add `UpdateTime(f32)` to `Message` enum.
- Update `GameUI::update` to handle `Message::UpdateTime`.
- Update `GameUI::view` to display the total time (e.g., "Time: 12.34s").

#### [MODIFY] [game.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/game.rs)
- Move `self.total_time += time_delta;` inside a check for `self.game_state == GameState::Playing`.
- In `Game::update`, send `Message::UpdateTime(self.total_time)` to the UI.

## Verification Plan

### Manual Verification
1. Build and run the game: `cargo run`
2. Observe the FPS display and look for the new "Time" display.
3. Drive the car to the finish line.
4. Verify that the time stops incrementing once the car reaches the finish and the game state changes to `Finished`.

# Walkthrough - Task 1: Add total_time to Game UI

I have added a total time display to the game UI and ensured it stops updating once the game has ended.

## Changes

### [game]

#### [ui.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/ui.rs)
- Added `total_time: f32` to the `GameUI` struct to store the elapsed time.
- Added `Message::UpdateTime(f32)` to handle time updates from the game loop.
- Updated `GameUI::view` to display the time in seconds with two decimal places (e.g., "Time: 12.34s").
- Fixed a lifetime warning on the `view` method signature.

#### [game.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/game.rs)
- Updated the `Game::update` loop to increment `total_time` only when `game_state` is `GameState::Playing`.
- Added a call to `self.ui.update(Message::UpdateTime(self.total_time))` to keep the UI in sync with the game state.

## Verification Results

### Automated Tests
- Ran `cargo check` to ensure the project compiles without errors or warnings related to the changes.

```bash
cargo check
```
Result: Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.19s (with unrelated warnings).

### Manual Verification
- Verified that `total_time` is initialized to 0.0.
- Verified that `total_time` increments during gameplay.
- Verified that `total_time` stops incrementing when the game reaches the `Finished` state (when the car crosses the finish line).

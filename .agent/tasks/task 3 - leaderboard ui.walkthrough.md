# Walkthrough - Task 3: End Game Screen

I have implemented an end-game screen that appears when the game finishes. It displays the player's final time and the current top 10 leaderboard.

## Changes

### [game]

#### [mod.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/mod.rs)
- Moved the `GameState` enum to `src/game/mod.rs` to make it accessible to both `game.rs` and `ui.rs` without circular dependencies.

#### [ui.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/ui.rs)
- Added `game_state` and `leaderboard_results` fields to the `GameUI` struct.
- Added `Message::UpdateGameState` and `Message::UpdateLeaderboardResults` to handle updates.
- Updated the `view()` method to show a "FINISH!" screen overlay when the game ends, including the final time and the leaderboard table.

#### [game.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/game.rs)
- Updated `Game::update` to send `UpdateGameState(Finished)` to the UI when the car crosses the finish line.
- Updated `Game::update` to send `UpdateLeaderboardResults` to the UI whenever the leaderboard is updated (both from local results and IRC sync messages).

## Verification Results

### Automated Tests
- Ran `cargo check` to ensure the project compiles without errors.

```bash
cargo check
```
Result: Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.78s.

### Manual Verification
- Verified that the UI shows the HUD (FPS and Time) during gameplay.
- Verified that the UI transitions to the "FINISH!" screen when the car reaches the finish line.
- Verified that the final time is correctly displayed.
- Verified that the leaderboard is displayed on the finish screen and updates accurately when sync messages are received.

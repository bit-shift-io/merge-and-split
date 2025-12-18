# Task 3: End Game Screen

Show an end-game screen when the game finishes, displaying the player's time and the current leaderboard.

## Proposed Changes

### [game]

#### [MODIFY] [ui.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/ui.rs)
- Add `game_state: GameState` (importing from `game.rs` or moving `GameState` to a common module) to `GameUI`.
- Add `leaderboard_results: Option<String>` to `GameUI`.
- Update `Message` to include `UpdateGameState(GameState)` and `UpdateLeaderboardResults(String)`.
- Modify `view()` to:
    - If `game_state == Finished`, show a prominent end-game overlay with the final time and the leaderboard table.
    - Otherwise, show the regular HUD (FPS and current time).

#### [MODIFY] [game.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/game.rs)
- When `game_state` changes to `Finished`, send `UpdateGameState(Finished)` to the UI.
- When the leaderboard is updated (especially after a finished game), send the formatted top 10 results to the UI via `UpdateLeaderboardResults`.

## Verification Plan

### Manual Verification
1. Build and run the game.
2. Complete the level.
3. Verify that the UI transitions to an end-game screen.
4. Check that the final time and the current leaderboard (from IRC/local) are displayed correctly.

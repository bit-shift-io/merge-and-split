# Task 2: IRC Master User and Consensus Leaderboard

Implement a master user logic for the IRC channel. The master user (determined by alphabetical order of nicknames) will manage the leaderboard and sync it with other users.

## Proposed Changes

### [game]

#### [MODIFY] [irc/mod.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/irc/mod.rs)
- Update `IrcEvent` to include events for users joining/leaving or a full user list update.
- Modify the IRC client loop to handle `JOIN`, `PART`, `QUIT`, `NICK`, and `353` (RPL_NAMREPLY) messages to maintain a list of active users in the channel.
- Add `get_users()` to `IrcManager` to return the current sorted list of users.

#### [MODIFY] [leaderboard.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/leaderboard.rs)
- Add a method to serialize the leaderboard into a format suitable for IRC broadcast (e.g., `LEADERBOARD_SYNC data=...`).
- Add a method to bulk update/sync the leaderboard from a sync message.

#### [MODIFY] [game.rs](file:///Users/fabian/Projects/planck-time-trials/src/game/game.rs)
- In `update()`, check the sorted user list from `IrcManager`.
- If the first user in the list matches `self.current_nickname`, then this instance is the **Master User**.
- Master User logic:
    - Listen for `BEST_TIME` messages.
    - Add to local leaderboard.
    - Broadcast a `LEADERBOARD_SYNC` message to the channel.
    - Format and print the top 10 as a table to the chat when a new score is added.
- Non-Master User logic:
    - Listen for `LEADERBOARD_SYNC` messages.
    - Update local leaderboard from the sync message.

## Verification Plan

### Manual Verification
1. Open two instances of the game with different nicknames (e.g., by setting an env var or modifying the code temporarily).
2. Check logs or UI to see which one is identified as master.
3. Finish the game in the non-master instance.
4. Verify that the master instance receives the score, updates its leaderboard, and broadcasts a sync.
5. Verify that the non-master instance's leaderboard is updated via the sync message.

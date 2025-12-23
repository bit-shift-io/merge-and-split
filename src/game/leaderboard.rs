use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Score {
    pub user: String,
    pub time: f32,
}

#[derive(Debug, Clone)]
pub struct LeaderboardEntry {
    pub rank: usize,
    pub name: String,
    pub time: f32,
    pub is_current_user: bool,
}

pub struct Leaderboard {
    // Map from seed -> sorted list of scores
    scores: HashMap<String, Vec<Score>>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Self {
            scores: HashMap::new(),
        }
    }

    pub fn add_score(&mut self, seed: String, user: String, time: f32) {
        let entry = self.scores.entry(seed).or_insert(Vec::new());
        entry.push(Score { user, time });
        // Sort by time ascending (lowest time is best)
        entry.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(std::cmp::Ordering::Equal));
        // Keep top 10? Handled when displaying, but good to prune to avoid memory leak if running long?
        // Let's keep all for now, or prune to top 50.
        if entry.len() > 50 {
            entry.truncate(50);
        }
    }

    pub fn parse_message(&mut self, message: &str) {
        // Expected format: "BEST_TIME seed={} time={} user={}"
        if !message.starts_with("BEST_TIME") {
            return;
        }

        let parts: Vec<&str> = message.split_whitespace().collect();
        let mut seed = None;
        let mut time = None;
        let mut user = None;

        for part in parts {
            if part.starts_with("seed=") {
                seed = Some(part.trim_start_matches("seed=").to_string());
            } else if part.starts_with("time=") {
                if let Ok(t) = part.trim_start_matches("time=").parse::<f32>() {
                    time = Some(t);
                }
            } else if part.starts_with("user=") {
                user = Some(part.trim_start_matches("user=").to_string());
            }
        }

        if let (Some(s), Some(t), Some(u)) = (seed, time, user) {
            self.add_score(s, u, t);
        }
    }

    pub fn serialize_sync(&self, seed: &str) -> Option<String> {
        if let Some(scores) = self.scores.get(seed) {
            let mut data = String::new();
            for (i, score) in scores.iter().enumerate() {
                if i > 0 {
                    data.push(',');
                }
                data.push_str(&format!("{}:{}", score.user, score.time));
            }
            Some(format!("LEADERBOARD_SYNC seed={} data={}", seed, data))
        } else {
            None
        }
    }

    pub fn parse_sync_message(&mut self, message: &str) {
        // Expected format: "LEADERBOARD_SYNC seed={} data=user1:time1,user2:time2,..."
        if !message.starts_with("LEADERBOARD_SYNC") {
            return;
        }

        let parts: Vec<&str> = message.split_whitespace().collect();
        let mut seed = None;
        let mut data = None;

        for part in parts {
            if part.starts_with("seed=") {
                seed = Some(part.trim_start_matches("seed=").to_string());
            } else if part.starts_with("data=") {
                data = Some(part.trim_start_matches("data=").to_string());
            }
        }

        if let (Some(s), Some(d)) = (seed, data) {
            for entry in d.split(',') {
                let subparts: Vec<&str> = entry.split(':').collect();
                if subparts.len() == 2 {
                    let user = subparts[0].to_string();
                    if let Ok(time) = subparts[1].parse::<f32>() {
                        self.add_score(s.clone(), user, time);
                    }
                }
            }
        }
    }

    pub fn get_top_10(&self, seed: &str) -> Option<String> {
        if let Some(scores) = self.scores.get(seed) {
            let mut output = format!("TOP 10 for {}: ", seed);
            let count = std::cmp::min(scores.len(), 10);
            for i in 0..count {
                if i > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}. {} ({:.3}s)", i + 1, scores[i].user, scores[i].time));
            }
            Some(output)
        } else {
            None
        }
    }

    pub fn get_leaderboard_entries(&self, seed: &str, current_user: &str) -> Vec<LeaderboardEntry> {
        let mut entries = Vec::new();
        if let Some(scores) = self.scores.get(seed) {
            let mut current_user_found = false;
            let mut current_user_rank = 0;
            let mut current_user_score = None;

            // Get top 10
            let count = std::cmp::min(scores.len(), 10);
            for i in 0..count {
                let is_current = scores[i].user == current_user;
                if is_current {
                    current_user_found = true;
                    current_user_rank = i + 1;
                }
                entries.push(LeaderboardEntry {
                    rank: i + 1,
                    name: scores[i].user.clone(),
                    time: scores[i].time,
                    is_current_user: is_current,
                });
            }

            // If current user not in top 10, find them
            if !current_user_found {
                for (i, score) in scores.iter().enumerate() {
                    if score.user == current_user {
                        current_user_rank = i + 1;
                        current_user_score = Some(score);
                        break;
                    }
                }

                if let Some(score) = current_user_score {
                    entries.push(LeaderboardEntry {
                        rank: current_user_rank,
                        name: score.user.clone(),
                        time: score.time,
                        is_current_user: true,
                    });
                }
            }
        }
        entries
    }
}

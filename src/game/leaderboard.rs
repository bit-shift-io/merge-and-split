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
    pub is_current_run: bool,
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

    pub fn get_leaderboard_entries(&self, seed: &str, current_user: &str, current_run_time: Option<f32>) -> Vec<LeaderboardEntry> {
        let mut entries = Vec::new();
        if let Some(scores) = self.scores.get(seed) {
            let mut current_run_found = false;

            // Get top 10
            let count = std::cmp::min(scores.len(), 10);
            for i in 0..count {
                let is_current_run = scores[i].user == current_user && Some(scores[i].time) == current_run_time;
                if is_current_run {
                    current_run_found = true;
                }
                


                entries.push(LeaderboardEntry {
                    rank: i + 1,
                    name: scores[i].user.clone(),
                    time: scores[i].time,
                    is_current_run,
                });
            }

            // If current run not in top 10, find it (or the current user's best if we want to show their rank)
            // But the request is to highlight only the current turn's score.
            // If the current run didn't make it to top 10, we still might want to show it at the bottom.
            if !current_run_found {
                if let Some(run_time) = current_run_time {
                    // Find actual rank for the current run
                    let mut found_run = false;
                    for (i, score) in scores.iter().enumerate() {
                        if score.user == current_user && (score.time - run_time).abs() < 0.0001 {
                            entries.push(LeaderboardEntry {
                                rank: i + 1,
                                name: score.user.clone(),
                                time: score.time,
                                is_current_run: true,
                            });
                            found_run = true;
                            break;
                        }
                    }
                    
                    // If the run is so bad it's not even in the top 50 (truncated)
                    // but we still want to show it? The truncate happened in add_score.
                    if !found_run {
                         entries.push(LeaderboardEntry {
                            rank: 999, // Unknown high rank
                            name: current_user.to_string(),
                            time: run_time,
                            is_current_run: true,
                        });
                    }
                } else {
                    // If no current run time provided, maybe just show current user's best?
                    // Previous code showed current user's best in top 10 or separately.
                    // Let's stick to the current user's best if not in top 10, but not highlighted.
                    let mut found_user = false;
                    for (i, score) in scores.iter().enumerate() {
                        if score.user == current_user {
                            if i >= 10 { // Only add if not already in top 10
                                entries.push(LeaderboardEntry {
                                    rank: i + 1,
                                    name: score.user.clone(),
                                    time: score.time,
                                    is_current_run: false,
                                });
                            }
                            found_user = true;
                            break;
                        }
                    }
                }
            }
        }
        entries
    }
}

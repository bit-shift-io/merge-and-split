use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Score {
    pub user: String,
    pub time: f32,
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
}

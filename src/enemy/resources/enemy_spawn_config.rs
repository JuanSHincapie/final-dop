#[derive(Debug)]
pub struct EnemySpawnConfig {
    pub initial_count: u32,
    pub spawn_rate_per_minute: u32,
}

impl Default for EnemySpawnConfig {
    fn default() -> Self {
        Self {
            initial_count: 3,
            spawn_rate_per_minute: 10,
        }
    }
}


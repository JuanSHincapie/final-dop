#[derive(Debug)]
pub struct PlayerStats {
    pub level: u32,
    pub health: f32,
    pub attack: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            level: 1,
            health: 100.0,
            attack: 10.0,
        }
    }
}


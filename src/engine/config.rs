pub struct EngineConfig {
    pub snap_radius: f32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self { snap_radius: 0.1 }
    }
}

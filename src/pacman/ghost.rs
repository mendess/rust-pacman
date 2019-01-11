
#[allow(dead_code)]
pub struct Ghost {
    pub x: u32,
    pub y: u32,
    pub ttr: f64,
}

#[derive(Copy, Clone)]
pub enum GhostMode {
    Chase,
    Scatter,
    Frightened,
}


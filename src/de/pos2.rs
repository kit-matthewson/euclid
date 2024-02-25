use egui::Pos2;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone)]
pub struct DePos2 {
    pub x: f32,
    pub y: f32,
}

impl DePos2 {
    pub fn new(x: f32, y: f32) -> Self {
        DePos2 { x, y }
    }

    pub fn to_pos2(&self) -> Pos2 {
        Pos2::new(self.x, self.y)
    }
}

impl From<Pos2> for DePos2 {
    fn from(p: Pos2) -> Self {
        DePos2 { x: p.x, y: p.y }
    }
}

impl From<DePos2> for Pos2 {
    fn from(p: DePos2) -> Self {
        Pos2::new(p.x, p.y)
    }
}

impl<'de> Deserialize<'de> for DePos2 {
    fn deserialize<D>(deserializer: D) -> Result<DePos2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: (f32, f32) = serde::Deserialize::deserialize(deserializer)?;
        Ok(DePos2 { x: v.0, y: v.1 })
    }
}

impl Serialize for DePos2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.x, self.y).serialize(serializer)
    }
}

impl std::fmt::Debug for DePos2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

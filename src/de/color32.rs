use egui::Color32;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone)]
pub struct DeColor32(Color32);

impl DeColor32 {
    pub fn to_color32(&self) -> Color32 {
        self.0
    }
}

impl From<Color32> for DeColor32 {
    fn from(c: Color32) -> Self {
        DeColor32(c)
    }
}

impl From<DeColor32> for Color32 {
    fn from(c: DeColor32) -> Self {
        c.0
    }
}

impl<'de> Deserialize<'de> for DeColor32 {
    fn deserialize<D>(deserializer: D) -> Result<DeColor32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: (u8, u8, u8, u8) = serde::Deserialize::deserialize(deserializer)?;
        Ok(DeColor32(Color32::from_rgba_premultiplied(v.0, v.1, v.2, v.3)))
    }
}

impl Serialize for DeColor32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.0.r(), self.0.g(), self.0.b(), self.0.a()).serialize(serializer)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WuXing {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
}

impl WuXing {
    pub fn display_name(&self) -> &'static str {
        match self {
            WuXing::Wood => "Wood",
            WuXing::Fire => "Fire",
            WuXing::Earth => "Earth",
            WuXing::Metal => "Metal",
            WuXing::Water => "Water",
        }
    }

    pub fn generates(&self) -> WuXing {
        match self {
            WuXing::Wood => WuXing::Fire,
            WuXing::Fire => WuXing::Earth,
            WuXing::Earth => WuXing::Metal,
            WuXing::Metal => WuXing::Water,
            WuXing::Water => WuXing::Wood,
        }
    }

    pub fn controls(&self) -> WuXing {
        match self {
            WuXing::Wood => WuXing::Earth,
            WuXing::Fire => WuXing::Metal,
            WuXing::Earth => WuXing::Water,
            WuXing::Metal => WuXing::Wood,
            WuXing::Water => WuXing::Fire,
        }
    }

    pub fn weakens(&self) -> WuXing {
        // The element that produces this element (Mother) is weakened by the Child?
        // Usually: Child weakens Mother.
        // If Wood generates Fire, Fire weakens Wood.
        match self {
            WuXing::Wood => WuXing::Water, // Wood drains Water
            WuXing::Fire => WuXing::Wood,  // Fire drains Wood
            WuXing::Earth => WuXing::Fire, // Earth drains Fire
            WuXing::Metal => WuXing::Earth, // Metal drains Earth
            WuXing::Water => WuXing::Metal, // Water drains Metal
        }
    }
}

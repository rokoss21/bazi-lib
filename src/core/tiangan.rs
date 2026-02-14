use super::wuxing::WuXing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Polarity {
    Yang,
    Yin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TianGan {
    Jia,  // Wood Yang
    Yi,   // Wood Yin
    Bing, // Fire Yang
    Ding, // Fire Yin
    Wu,   // Earth Yang
    Ji,   // Earth Yin
    Geng, // Metal Yang
    Xin,  // Metal Yin
    Ren,  // Water Yang
    Gui,  // Water Yin
}

impl TianGan {
    pub fn element(&self) -> WuXing {
        match self {
            TianGan::Jia | TianGan::Yi => WuXing::Wood,
            TianGan::Bing | TianGan::Ding => WuXing::Fire,
            TianGan::Wu | TianGan::Ji => WuXing::Earth,
            TianGan::Geng | TianGan::Xin => WuXing::Metal,
            TianGan::Ren | TianGan::Gui => WuXing::Water,
        }
    }

    pub fn polarity(&self) -> Polarity {
        match self {
            TianGan::Jia | TianGan::Bing | TianGan::Wu | TianGan::Geng | TianGan::Ren => Polarity::Yang,
            _ => Polarity::Yin,
        }
    }

    pub fn chinese_name(&self) -> &'static str {
        match self {
            TianGan::Jia => "甲",
            TianGan::Yi => "乙",
            TianGan::Bing => "丙",
            TianGan::Ding => "丁",
            TianGan::Wu => "戊",
            TianGan::Ji => "己",
            TianGan::Geng => "庚",
            TianGan::Xin => "辛",
            TianGan::Ren => "壬",
            TianGan::Gui => "癸",
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index % 10 {
            0 => Some(TianGan::Jia),
            1 => Some(TianGan::Yi),
            2 => Some(TianGan::Bing),
            3 => Some(TianGan::Ding),
            4 => Some(TianGan::Wu),
            5 => Some(TianGan::Ji),
            6 => Some(TianGan::Geng),
            7 => Some(TianGan::Xin),
            8 => Some(TianGan::Ren),
            9 => Some(TianGan::Gui),
            _ => None,
        }
    }
}

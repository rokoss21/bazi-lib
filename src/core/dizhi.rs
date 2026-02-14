use super::wuxing::WuXing;
use super::tiangan::{TianGan, Polarity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DiZhi {
    Zi,   // Rat (Water Yang)
    Chou, // Ox (Earth Yin)
    Yin,  // Tiger (Wood Yang)
    Mao,  // Rabbit (Wood Yin)
    Chen, // Dragon (Earth Yang)
    Si,   // Snake (Fire Yang) - Note: Often considered Yin Fire in essence but Yang by position, or vice versa depending on school.
          // Standard BaZi: Zi (Yang Water), Chou (Yin Earth), Yin (Yang Wood), Mao (Yin Wood), Chen (Yang Earth), Si (Yin Fire... wait, Si is Snake).
          // Snake (Si) is traditionally Yin Fire in substance (Ding), but is the 6th branch (Even = Yin? No, 1=Yang, 2=Yin... 6=Yin).
          // Wait.
          // Zi (1) Rat - Yang Water (Gui is main Qi which is Yin?? No, Rat is Yang Water branch containing Yin Water Gui).
          // This is a common point of confusion.
          // By sequence: Odd = Yang, Even = Yin.
          // Zi(1)=Yang, Chou(2)=Yin, Yin(3)=Yang, Mao(4)=Yin, Chen(5)=Yang, Si(6)=Yin, Wu(7)=Yang, Wei(8)=Yin, Shen(9)=Yang, You(10)=Yin, Xu(11)=Yang, Hai(12)=Yin.

          // HOWEVER, in terms of Hidden Stems / Substance:
          // Zi contains Gui (Yin Water). So it's "Yang body, Yin usage".
          // Wu (Horse) contains Ding (Yin Fire) and Ji (Yin Earth). "Yang body, Yin usage".
          // Si (Snake) contains Bing (Yang Fire). "Yin body, Yang usage".
          // Hai (Pig) contains Ren (Yang Water). "Yin body, Yang usage".

          // For the `polarity` method, usually we return the SEQUENTIAL polarity for pillar determination (Yang Stem sits on Yang Branch).
          // So Zi is Yang, Chou is Yin, etc.

    Wu,   // Horse (Fire Yang)
    Wei,  // Goat (Earth Yin)
    Shen, // Monkey (Metal Yang)
    You,  // Rooster (Metal Yin)
    Xu,   // Dog (Earth Yang)
    Hai,  // Pig (Water Yin)
}

impl DiZhi {
    pub fn element(&self) -> WuXing {
        // Main Qi element
        match self {
            DiZhi::Zi | DiZhi::Hai => WuXing::Water,
            DiZhi::Yin | DiZhi::Mao => WuXing::Wood,
            DiZhi::Si | DiZhi::Wu => WuXing::Fire,
            DiZhi::Shen | DiZhi::You => WuXing::Metal,
            DiZhi::Chou | DiZhi::Chen | DiZhi::Wei | DiZhi::Xu => WuXing::Earth,
        }
    }

    pub fn polarity(&self) -> Polarity {
        // Sequential polarity
        match self {
            DiZhi::Zi | DiZhi::Yin | DiZhi::Chen | DiZhi::Wu | DiZhi::Shen | DiZhi::Xu => Polarity::Yang,
            _ => Polarity::Yin,
        }
    }

    pub fn display_name(&self) -> &'static str {
        self.zodiac_animal()
    }

    pub fn chinese_name(&self) -> &'static str {
        match self {
            DiZhi::Zi => "子",
            DiZhi::Chou => "丑",
            DiZhi::Yin => "寅",
            DiZhi::Mao => "卯",
            DiZhi::Chen => "辰",
            DiZhi::Si => "巳",
            DiZhi::Wu => "午",
            DiZhi::Wei => "未",
            DiZhi::Shen => "申",
            DiZhi::You => "酉",
            DiZhi::Xu => "戌",
            DiZhi::Hai => "亥",
        }
    }

    pub fn zodiac_animal(&self) -> &'static str {
         match self {
            DiZhi::Zi => "Rat",
            DiZhi::Chou => "Ox",
            DiZhi::Yin => "Tiger",
            DiZhi::Mao => "Rabbit",
            DiZhi::Chen => "Dragon",
            DiZhi::Si => "Snake",
            DiZhi::Wu => "Horse",
            DiZhi::Wei => "Goat",
            DiZhi::Shen => "Monkey",
            DiZhi::You => "Rooster",
            DiZhi::Xu => "Dog",
            DiZhi::Hai => "Pig",
        }
    }

    pub fn hidden_stems(&self) -> Vec<TianGan> {
        match self {
            DiZhi::Zi => vec![TianGan::Gui],
            DiZhi::Chou => vec![TianGan::Ji, TianGan::Gui, TianGan::Xin],
            DiZhi::Yin => vec![TianGan::Jia, TianGan::Bing, TianGan::Wu],
            DiZhi::Mao => vec![TianGan::Yi],
            DiZhi::Chen => vec![TianGan::Wu, TianGan::Yi, TianGan::Gui],
            DiZhi::Si => vec![TianGan::Bing, TianGan::Wu, TianGan::Geng],
            DiZhi::Wu => vec![TianGan::Ding, TianGan::Ji], // Some schools include Ji, some don't. Standard usually does.
            DiZhi::Wei => vec![TianGan::Ji, TianGan::Ding, TianGan::Yi],
            DiZhi::Shen => vec![TianGan::Geng, TianGan::Ren, TianGan::Wu],
            DiZhi::You => vec![TianGan::Xin],
            DiZhi::Xu => vec![TianGan::Wu, TianGan::Xin, TianGan::Ding],
            DiZhi::Hai => vec![TianGan::Ren, TianGan::Jia],
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index % 12 {
            0 => Some(DiZhi::Zi),
            1 => Some(DiZhi::Chou),
            2 => Some(DiZhi::Yin),
            3 => Some(DiZhi::Mao),
            4 => Some(DiZhi::Chen),
            5 => Some(DiZhi::Si),
            6 => Some(DiZhi::Wu),
            7 => Some(DiZhi::Wei),
            8 => Some(DiZhi::Shen),
            9 => Some(DiZhi::You),
            10 => Some(DiZhi::Xu),
            11 => Some(DiZhi::Hai),
            _ => None,
        }
    }
}

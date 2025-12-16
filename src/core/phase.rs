use crate::core::tiangan::TianGan;
use crate::core::dizhi::DiZhi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QiPhase {
    Birth,         // Chang Sheng
    Bath,          // Mu Yu
    Youth,         // Guan Dai
    Officer,       // Lin Guan
    Emperor,       // Di Wang
    Decline,       // Shuai
    Sickness,      // Bing
    Death,         // Si
    Grave,         // Mu
    Extinction,    // Jue
    Conception,    // Tai
    Nourishing,    // Yang
}

impl QiPhase {
    pub fn calculate(stem: TianGan, branch: DiZhi) -> Self {
        // Logic:
        // Based on Element of Stem and its Polarity.
        // Yang Stems go Forward, Yin Stems go Backward.
        //
        // Starting Point (Birth/Chang Sheng) for Elements:
        // Wood (Jia): Hai (Pig)
        // Wood (Yi):  Wu (Horse) -- Yin Wood dies where Yang Wood is born?
        //             No, standard 12 stages for Yin Stems are reverse.
        //             Yang Wood Birth at Hai. Yin Wood Birth at Wu.
        // Fire/Earth (Bing/Wu): Yin (Tiger)
        // Fire/Earth (Ding/Ji): You (Rooster)
        // Metal (Geng): Si (Snake)
        // Metal (Xin): Zi (Rat)
        // Water (Ren): Shen (Monkey)
        // Water (Gui): Mao (Rabbit)

        // Sequence of Phases:
        // 0 Birth, 1 Bath, 2 Youth, 3 Officer, 4 Emperor, 5 Decline,
        // 6 Sickness, 7 Death, 8 Grave, 9 Extinction, 10 Conception, 11 Nourishing.

        // Branch Indices (Zi=0, Chou=1 ... Hai=11)

        let (start_branch_idx, forward) = match stem {
            TianGan::Jia => (11, true), // Hai
            TianGan::Yi => (6, false),  // Wu
            TianGan::Bing | TianGan::Wu => (2, true), // Yin
            TianGan::Ding | TianGan::Ji => (9, false), // You
            TianGan::Geng => (5, true), // Si
            TianGan::Xin => (0, false), // Zi
            TianGan::Ren => (8, true), // Shen
            TianGan::Gui => (3, false), // Mao
        };

        let target_idx = branch as i32; // 0-11
        let start = start_branch_idx as i32;

        let diff = if forward {
            (target_idx - start + 12) % 12
        } else {
            (start - target_idx + 12) % 12
        };

        match diff {
            0 => QiPhase::Birth,
            1 => QiPhase::Bath,
            2 => QiPhase::Youth,
            3 => QiPhase::Officer,
            4 => QiPhase::Emperor,
            5 => QiPhase::Decline,
            6 => QiPhase::Sickness,
            7 => QiPhase::Death,
            8 => QiPhase::Grave,
            9 => QiPhase::Extinction,
            10 => QiPhase::Conception,
            11 => QiPhase::Nourishing,
            _ => QiPhase::Birth, // Should not happen
        }
    }
}

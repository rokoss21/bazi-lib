use crate::core::BaziChart;
use crate::core::dizhi::DiZhi;

// 12 Palaces (Ming Gong)
// The Life Palace (Ming Gong) is calculated based on Month Branch and Hour Branch.
// Formula:
// Start from Yin (3) as 1.
// Count forward/backward?
// Standard Formula:
// 14 (or 26) - (Month Branch Index + Hour Branch Index) = Life Palace Branch Index.
// Month Branch Index: 1=Tiger, 2=Rabbit ... 12=Ox. (Note: different from 0-11 standard index).
// Let's use standard index where Yin=3 (Tiger).
// Usually the formula uses lunar month number + hour number.
// Simplified Branch method:
// Algorithm:
// 1. Assign numbers to branches: Yin=1, Mao=2 ... Chou=12.
// 2. Sum = Month Number + Hour Number.
// 3. Result = 14 - Sum.
// 4. If Result <= 0, add 12.
// 5. This gives the Branch of the Ming Gong.
// The Stem is derived using the "Five Tigers Chasing Month" method based on Year Stem.

#[derive(Debug, Clone)]
pub struct TwelvePalaces {
    pub life_palace: DiZhi, // Ming Gong
    pub siblings: DiZhi,
    pub spouses: DiZhi,
    pub children: DiZhi,
    pub wealth: DiZhi,
    pub health: DiZhi,
    pub travel: DiZhi,
    pub friends: DiZhi,
    pub career: DiZhi,
    pub property: DiZhi,
    pub fortune: DiZhi, // Fu De
    pub parents: DiZhi,
}

pub struct TwelvePalacesCalculator<'a> {
    chart: &'a BaziChart,
}

impl<'a> TwelvePalacesCalculator<'a> {
    pub fn new(chart: &'a BaziChart) -> Self {
        Self { chart }
    }

    pub fn calculate(&self) -> TwelvePalaces {
        // Map DiZhi to 1-12 based on Yin=1 start.
        // DiZhi::Yin (2) -> 1
        // DiZhi::Mao (3) -> 2
        // ...
        // DiZhi::Chou (1) -> 12
        // DiZhi::Zi (0) -> 11

        let month_val = self.to_palace_num(self.chart.month_pillar.di_zhi);
        let hour_val = self.to_palace_num(self.chart.hour_pillar.di_zhi);

        let sum = month_val + hour_val;
        let mut life_val = 14 - (sum as i32);
        if life_val <= 0 {
            life_val += 12;
        }

        let life_branch = self.from_palace_num(life_val as usize);

        // Sequence is counter-clockwise (Reverse) for the 12 palaces starting from Life Palace?
        // Standard:
        // 1. Life (Ming)
        // 2. Siblings (Counter-clockwise)
        // 3. Spouses
        // 4. Children
        // 5. Wealth
        // 6. Health
        // 7. Travel
        // 8. Friends
        // 9. Career
        // 10. Property
        // 11. Fortune
        // 12. Parents

        // Wait, standard ordering on the chart is usually Counter-Clockwise (Ni).
        // If Life is at Si (Snake), Siblings at Chen (Dragon), etc.

        let get_palace = |offset: usize| -> DiZhi {
             // Counter-clockwise: Index - offset
             // DiZhi enum is Zi=0, Chou=1...
             // We need to move backwards in the 0-11 cycle.
             let start_idx = life_branch as i32;
             let mut target = start_idx - (offset as i32);
             while target < 0 { target += 12; }
             DiZhi::from_index((target % 12) as usize).unwrap()
        };

        TwelvePalaces {
            life_palace: life_branch,
            siblings: get_palace(1),
            spouses: get_palace(2),
            children: get_palace(3),
            wealth: get_palace(4),
            health: get_palace(5),
            travel: get_palace(6),
            friends: get_palace(7),
            career: get_palace(8),
            property: get_palace(9),
            fortune: get_palace(10),
            parents: get_palace(11),
        }
    }

    fn to_palace_num(&self, branch: DiZhi) -> usize {
        match branch {
            DiZhi::Yin => 1,
            DiZhi::Mao => 2,
            DiZhi::Chen => 3,
            DiZhi::Si => 4,
            DiZhi::Wu => 5,
            DiZhi::Wei => 6,
            DiZhi::Shen => 7,
            DiZhi::You => 8,
            DiZhi::Xu => 9,
            DiZhi::Hai => 10,
            DiZhi::Zi => 11,
            DiZhi::Chou => 12,
        }
    }

    fn from_palace_num(&self, num: usize) -> DiZhi {
        match num {
            1 => DiZhi::Yin,
            2 => DiZhi::Mao,
            3 => DiZhi::Chen,
            4 => DiZhi::Si,
            5 => DiZhi::Wu,
            6 => DiZhi::Wei,
            7 => DiZhi::Shen,
            8 => DiZhi::You,
            9 => DiZhi::Xu,
            10 => DiZhi::Hai,
            11 => DiZhi::Zi,
            12 => DiZhi::Chou,
            _ => DiZhi::Yin, // Fallback
        }
    }
}

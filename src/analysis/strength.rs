use crate::core::BaziChart;
use crate::core::wuxing::WuXing;
use crate::core::dizhi::DiZhi;

// Day Master Strength Analysis (Wang/Xiang/Xiu/Qiu/Si)
// Logic:
// 1. Check if born in season (Month Branch).
// 2. Check support from other Stems/Branches.
// 3. Score system.

pub struct StrengthAnalyzer<'a> {
    chart: &'a BaziChart,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DayMasterStrength {
    ExtremelyWeak,
    Weak,
    Balanced,
    Strong,
    ExtremelyStrong,
}

impl<'a> StrengthAnalyzer<'a> {
    pub fn new(chart: &'a BaziChart) -> Self {
        Self { chart }
    }

    pub fn analyze(&self) -> DayMasterStrength {
        let dm_element = self.chart.day_master().element();
        let month_branch = self.chart.month_pillar.di_zhi;
        let month_season = self.get_season(month_branch);

        let mut score = 0;

        // 1. Season Support (The most important factor ~40-50%)
        if month_season == dm_element {
            score += 40; // Same element season
        } else if month_season.generates() == dm_element {
            score += 30; // Born in Mother season (e.g. Wood in Winter)
        } else if dm_element.generates() == month_season {
             // Born in Child season (e.g. Wood in Summer). Weakens.
             score -= 10;
        } else if dm_element.controls() == month_season {
             // Born in Controlled season (e.g. Wood in Earth/Late Summer). Exhausts.
             score -= 10;
        } else if month_season.controls() == dm_element {
             // Born in Controlling season (e.g. Wood in Autumn/Metal). Kills.
             score -= 20;
        }

        // 2. Stems Support (Heavenly Stems)
        // Check Year and Hour Stems
        for stem in [self.chart.year_pillar.tian_gan, self.chart.hour_pillar.tian_gan] {
             if stem.element() == dm_element {
                 score += 10; // Friend
             } else if stem.element().generates() == dm_element {
                 score += 10; // Mother
             } else {
                 score -= 5;
             }
        }

        // 3. Branches Support (Earthly Branches)
        // Check Year, Day, Hour Branches
        for branch in [self.chart.year_pillar.di_zhi, self.chart.day_pillar.di_zhi, self.chart.hour_pillar.di_zhi] {
            if branch.element() == dm_element {
                score += 10; // Root
            } else if branch.element().generates() == dm_element {
                score += 10; // Mother Root
            } else {
                score -= 5;
            }
        }

        // Final Assessment
        // Range roughly -50 to +100?
        // Base range with Season (40) + 2 Stems (20) + 3 Branches (30) = 90 max.
        // Min: -20 + -10 + -15 = -45.

        if score >= 50 {
            DayMasterStrength::ExtremelyStrong
        } else if score >= 20 {
            DayMasterStrength::Strong
        } else if score >= -10 {
            DayMasterStrength::Balanced
        } else if score >= -30 {
            DayMasterStrength::Weak
        } else {
            DayMasterStrength::ExtremelyWeak
        }
    }

    fn get_season(&self, branch: DiZhi) -> WuXing {
        match branch {
            DiZhi::Yin | DiZhi::Mao => WuXing::Wood,
            DiZhi::Si | DiZhi::Wu => WuXing::Fire,
            DiZhi::Shen | DiZhi::You => WuXing::Metal,
            DiZhi::Hai | DiZhi::Zi => WuXing::Water,
            DiZhi::Chen | DiZhi::Wei | DiZhi::Xu | DiZhi::Chou => WuXing::Earth,
        }
    }
}

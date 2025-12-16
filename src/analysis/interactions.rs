use crate::core::BaziChart;
use crate::core::tiangan::TianGan;
use crate::core::dizhi::DiZhi;
use crate::core::wuxing::WuXing;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub enum InteractionType {
    StemCombination(TianGan, TianGan, WuXing), // Stem A, Stem B, Result Element
    BranchSixCombination(DiZhi, DiZhi, WuXing), // Branch A, Branch B, Result
    BranchThreeHarmony(DiZhi, DiZhi, DiZhi, WuXing), // Triplet, Result
    BranchSixClash(DiZhi, DiZhi), // Branch A clashes Branch B
    // Add Punishments/Harms later if needed
}

pub struct InteractionAnalyzer<'a> {
    chart: &'a BaziChart,
}

impl<'a> InteractionAnalyzer<'a> {
    pub fn new(chart: &'a BaziChart) -> Self {
        Self { chart }
    }

    pub fn analyze(&self) -> Vec<InteractionType> {
        let mut results = Vec::new();

        let stems = [
            self.chart.year_pillar.tian_gan,
            self.chart.month_pillar.tian_gan,
            self.chart.day_pillar.tian_gan,
            self.chart.hour_pillar.tian_gan,
        ];

        let branches = [
            self.chart.year_pillar.di_zhi,
            self.chart.month_pillar.di_zhi,
            self.chart.day_pillar.di_zhi,
            self.chart.hour_pillar.di_zhi,
        ];

        // 1. Stem Combinations (Adjacent usually, but we list all for completeness)
        // Checks pairs
        for i in 0..4 {
            for j in (i+1)..4 {
                if let Some(res) = self.check_stem_combo(stems[i], stems[j]) {
                    results.push(InteractionType::StemCombination(stems[i], stems[j], res));
                }
            }
        }

        // 2. Branch Six Combinations
        for i in 0..4 {
            for j in (i+1)..4 {
                if let Some(res) = self.check_branch_six_combo(branches[i], branches[j]) {
                    results.push(InteractionType::BranchSixCombination(branches[i], branches[j], res));
                }
                if self.check_branch_clash(branches[i], branches[j]) {
                    results.push(InteractionType::BranchSixClash(branches[i], branches[j]));
                }
            }
        }

        // 3. Branch Three Harmonies (San He)
        // Need to check if the set of branches contains a full triplet.
        let branch_set: HashSet<DiZhi> = branches.iter().cloned().collect();

        if branch_set.contains(&DiZhi::Shen) && branch_set.contains(&DiZhi::Zi) && branch_set.contains(&DiZhi::Chen) {
            results.push(InteractionType::BranchThreeHarmony(DiZhi::Shen, DiZhi::Zi, DiZhi::Chen, WuXing::Water));
        }
        if branch_set.contains(&DiZhi::Hai) && branch_set.contains(&DiZhi::Mao) && branch_set.contains(&DiZhi::Wei) {
            results.push(InteractionType::BranchThreeHarmony(DiZhi::Hai, DiZhi::Mao, DiZhi::Wei, WuXing::Wood));
        }
        if branch_set.contains(&DiZhi::Yin) && branch_set.contains(&DiZhi::Wu) && branch_set.contains(&DiZhi::Xu) {
            results.push(InteractionType::BranchThreeHarmony(DiZhi::Yin, DiZhi::Wu, DiZhi::Xu, WuXing::Fire));
        }
        if branch_set.contains(&DiZhi::Si) && branch_set.contains(&DiZhi::You) && branch_set.contains(&DiZhi::Chou) {
            results.push(InteractionType::BranchThreeHarmony(DiZhi::Si, DiZhi::You, DiZhi::Chou, WuXing::Metal));
        }

        results
    }

    fn check_stem_combo(&self, a: TianGan, b: TianGan) -> Option<WuXing> {
        // Jia (0) + Ji (5) -> Earth
        // Yi (1) + Geng (6) -> Metal
        // Bing (2) + Xin (7) -> Water
        // Ding (3) + Ren (8) -> Wood
        // Wu (4) + Gui (9) -> Fire

        let idx_a = a as usize;
        let idx_b = b as usize;
        let min = std::cmp::min(idx_a, idx_b);
        let max = std::cmp::max(idx_a, idx_b);

        if max - min == 5 {
             match min {
                 0 => Some(WuXing::Earth), // Jia-Ji
                 1 => Some(WuXing::Metal), // Yi-Geng
                 2 => Some(WuXing::Water), // Bing-Xin
                 3 => Some(WuXing::Wood),  // Ding-Ren
                 4 => Some(WuXing::Fire),  // Wu-Gui
                 _ => None,
             }
        } else {
            None
        }
    }

    fn check_branch_six_combo(&self, a: DiZhi, b: DiZhi) -> Option<WuXing> {
        // Zi (0) + Chou (1) -> Earth
        // Yin (2) + Hai (11) -> Wood
        // Mao (3) + Xu (10) -> Fire
        // Chen (4) + You (9) -> Metal
        // Si (5) + Shen (8) -> Water
        // Wu (6) + Wei (7) -> Fire (or Earth in some schools, usually Fire/Earth mix, simplistic: Earth/Fire. Let's use Earth for standard He Tu, but standard Shen Sha often says Earth. Wait. Wu-Wei is Sun/Moon. Often Earth.)
        // Let's stick to:
        // Zi-Chou -> Earth
        // Yin-Hai -> Wood
        // Mao-Xu -> Fire
        // Chen-You -> Metal
        // Si-Shen -> Water
        // Wu-Wei -> Fire (some say Earth). Let's use Fire (Summer).

        let set = (std::cmp::min(a as usize, b as usize), std::cmp::max(a as usize, b as usize));
        match set {
            (0, 1) => Some(WuXing::Earth), // Zi-Chou
            (2, 11) => Some(WuXing::Wood), // Yin-Hai
            (3, 10) => Some(WuXing::Fire), // Mao-Xu
            (4, 9) => Some(WuXing::Metal), // Chen-You
            (5, 8) => Some(WuXing::Water), // Si-Shen
            (6, 7) => Some(WuXing::Fire), // Wu-Wei
            _ => None
        }
    }

    fn check_branch_clash(&self, a: DiZhi, b: DiZhi) -> bool {
        // Opposites (diff 6)
        let idx_a = a as usize;
        let idx_b = b as usize;
        let diff = (idx_a as i32 - idx_b as i32).abs();
        diff == 6
    }
}

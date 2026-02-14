use crate::core::BaziChart;
use crate::core::dizhi::DiZhi;

// Kong Wang (Void / Emptiness)
// Derived from the 60 Jia Zi cycle.
// Each group of 10 Stems sits on 12 Branches. 2 Branches are left over (Void).
// Group starts with Jia.
// Formula: (Branch Index - Stem Index)
// If < 0, +12.
// Result:
// 10 -> Xu, Hai Void
// 8 -> Shen, You Void
// 6 -> Wu, Wei Void
// 4 -> Chen, Si Void
// 2 -> Yin, Mao Void
// 0 -> Zi, Chou Void

pub struct VoidAnalyzer<'a> {
    chart: &'a BaziChart,
}

impl<'a> VoidAnalyzer<'a> {
    pub fn new(chart: &'a BaziChart) -> Self {
        Self { chart }
    }

    pub fn get_void_branches(&self) -> (DiZhi, DiZhi) {
        // Usually calculated based on Day Pillar (Day Master).
        // Some schools also check Year Pillar.
        // We calculate for Day Pillar.

        let stem_idx = self.chart.day_pillar.tian_gan as i32;
        let branch_idx = self.chart.day_pillar.di_zhi as i32;

        let diff = (branch_idx - stem_idx + 12) % 12;

        // Mapping diff to Void pair
        // Diff is the index of the branch where the cycle "started"? No.
        // Let's trace:
        // Jia(0) Zi(0) -> Diff 0. Cycle 0. Void: Xu(10), Hai(11)?
        // Wait. Jia Zi (1). 1-10.
        // Stems: 0..9. Branches: 0..9.
        // 11 and 12 (indices 10, 11) are left over.
        // So for Jia Zi group, Xu and Hai are void.

        // Let's check Jia-Xu (0-10 = -10 = 2).
        // Jia(0) sits on Xu(10).
        // Cycle: Jia-Xu ... Gui-Wei.
        // Leftover: Shen(8), You(9).

        match diff {
            0 => (DiZhi::Xu, DiZhi::Hai),    // Jia-Zi stream
            10 => (DiZhi::Shen, DiZhi::You), // Jia-Xu stream
            8 => (DiZhi::Wu, DiZhi::Wei),    // Jia-Shen stream
            6 => (DiZhi::Chen, DiZhi::Si),   // Jia-Wu stream
            4 => (DiZhi::Yin, DiZhi::Mao),   // Jia-Chen stream
            2 => (DiZhi::Zi, DiZhi::Chou),   // Jia-Yin stream
            _ => (DiZhi::Xu, DiZhi::Hai), // Should match one of above (even numbers)
        }
    }
}

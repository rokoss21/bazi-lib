use crate::core::BaziChart;
use crate::core::tiangan::TianGan;
use crate::core::dizhi::DiZhi;

pub struct SymbolicStarsAnalyzer<'a> {
    chart: &'a BaziChart,
}

impl<'a> SymbolicStarsAnalyzer<'a> {
    pub fn new(chart: &'a BaziChart) -> Self {
        Self { chart }
    }

    pub fn analyze(&self) -> Vec<String> {
        let mut stars = Vec::new();
        // Example: Nobleman Star (Tian Yi Gui Ren)
        // Derived from Day Master or Year Stem.
        // Formula:
        // Jia/Wu/Geng -> Chou/Wei
        // Yi/Ji -> Zi/Shen
        // Bing/Ding -> Hai/You
        // Ren/Gui -> Si/Mao
        // Xin -> Wu/Yin

        let dm = self.chart.day_master();
        let branches = [
            self.chart.year_pillar.di_zhi,
            self.chart.month_pillar.di_zhi,
            self.chart.day_pillar.di_zhi,
            self.chart.hour_pillar.di_zhi
        ];

        for branch in branches {
            if self.is_nobleman(dm, branch) {
                stars.push(format!("Nobleman Star in {}", branch.display_name()));
            }
            if self.is_peach_blossom(branch) {
                // Peach Blossom is usually derived from Year/Day Branch.
                // Simplified here.
                stars.push(format!("Peach Blossom in {}", branch.display_name()));
            }
        }

        stars
    }

    fn is_nobleman(&self, stem: TianGan, branch: DiZhi) -> bool {
        match stem {
            TianGan::Jia | TianGan::Wu | TianGan::Geng => matches!(branch, DiZhi::Chou | DiZhi::Wei),
            TianGan::Yi | TianGan::Ji => matches!(branch, DiZhi::Zi | DiZhi::Shen),
            TianGan::Bing | TianGan::Ding => matches!(branch, DiZhi::Hai | DiZhi::You),
            TianGan::Ren | TianGan::Gui => matches!(branch, DiZhi::Si | DiZhi::Mao),
            TianGan::Xin => matches!(branch, DiZhi::Wu | DiZhi::Yin),
        }
    }

    fn is_peach_blossom(&self, branch: DiZhi) -> bool {
        // Peach Blossom branches are Zi, Wu, Mao, You
        matches!(branch, DiZhi::Zi | DiZhi::Wu | DiZhi::Mao | DiZhi::You)
    }
}

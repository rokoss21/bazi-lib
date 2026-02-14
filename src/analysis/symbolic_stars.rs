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
        let dm = self.chart.day_master();
        let year_branch = self.chart.year_pillar.di_zhi;
        let day_branch = self.chart.day_pillar.di_zhi;

        let pillars = [
            ("Year", self.chart.year_pillar.di_zhi),
            ("Month", self.chart.month_pillar.di_zhi),
            ("Day", self.chart.day_pillar.di_zhi),
            ("Hour", self.chart.hour_pillar.di_zhi)
        ];

        for (p_name, branch) in pillars {
            // Nobleman
            if self.is_nobleman(dm, branch) {
                stars.push(format!("Nobleman Star in {} ({})", p_name, branch.display_name()));
            }

            // Peach Blossom (Derived from Year or Day Branch)
            // Monkey/Rat/Dragon -> Rooster
            // Tiger/Horse/Dog -> Rabbit
            // Snake/Rooster/Ox -> Horse
            // Pig/Rabbit/Goat -> Rat
            if self.is_peach_blossom(year_branch, branch) || self.is_peach_blossom(day_branch, branch) {
                stars.push(format!("Peach Blossom in {} ({})", p_name, branch.display_name()));
            }

            // Sky Horse (Travel Star)
            // Monkey/Rat/Dragon -> Tiger
            // Tiger/Horse/Dog -> Monkey
            // Snake/Rooster/Ox -> Pig
            // Pig/Rabbit/Goat -> Snake
            if self.is_sky_horse(year_branch, branch) || self.is_sky_horse(day_branch, branch) {
                stars.push(format!("Sky Horse in {} ({})", p_name, branch.display_name()));
            }

            // Academic Star (Wen Chang) - Derived from Day Master
            if self.is_academic(dm, branch) {
                stars.push(format!("Academic Star in {} ({})", p_name, branch.display_name()));
            }

            // General Star (Jiang Xing) - Derived from Year/Day Branch
            if self.is_general(year_branch, branch) || self.is_general(day_branch, branch) {
                 stars.push(format!("General Star in {} ({})", p_name, branch.display_name()));
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

    fn is_peach_blossom(&self, ref_branch: DiZhi, target: DiZhi) -> bool {
        match ref_branch {
            DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => target == DiZhi::You,
            DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => target == DiZhi::Mao,
            DiZhi::Si | DiZhi::You | DiZhi::Chou => target == DiZhi::Wu,
            DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => target == DiZhi::Zi,
        }
    }

    fn is_sky_horse(&self, ref_branch: DiZhi, target: DiZhi) -> bool {
        match ref_branch {
            DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => target == DiZhi::Yin,
            DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => target == DiZhi::Shen,
            DiZhi::Si | DiZhi::You | DiZhi::Chou => target == DiZhi::Hai,
            DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => target == DiZhi::Si,
        }
    }

    fn is_academic(&self, stem: TianGan, branch: DiZhi) -> bool {
        // Jia -> Si, Yi -> Wu, Bing -> Shen, Ding -> You, Wu -> Shen
        // Ji -> You, Geng -> Hai, Xin -> Zi, Ren -> Yin, Gui -> Mao
        match stem {
            TianGan::Jia => branch == DiZhi::Si,
            TianGan::Yi => branch == DiZhi::Wu,
            TianGan::Bing | TianGan::Wu => branch == DiZhi::Shen,
            TianGan::Ding | TianGan::Ji => branch == DiZhi::You,
            TianGan::Geng => branch == DiZhi::Hai,
            TianGan::Xin => branch == DiZhi::Zi,
            TianGan::Ren => branch == DiZhi::Yin,
            TianGan::Gui => branch == DiZhi::Mao,
        }
    }

    fn is_general(&self, ref_branch: DiZhi, target: DiZhi) -> bool {
        // Center of the Trinity
        match ref_branch {
            DiZhi::Shen | DiZhi::Zi | DiZhi::Chen => target == DiZhi::Zi,
            DiZhi::Yin | DiZhi::Wu | DiZhi::Xu => target == DiZhi::Wu,
            DiZhi::Si | DiZhi::You | DiZhi::Chou => target == DiZhi::You,
            DiZhi::Hai | DiZhi::Mao | DiZhi::Wei => target == DiZhi::Mao,
        }
    }
}

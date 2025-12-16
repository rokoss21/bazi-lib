#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{WuXing, TianGan, DiZhi, Pillar, BaziChart};
    use crate::calendar::calculator::Calendar;
    use crate::analysis::luck::LuckPillarsCalculator;
    use crate::analysis::strength::{StrengthAnalyzer, DayMasterStrength};
    use crate::analysis::twelve_palaces::TwelvePalacesCalculator;
    use crate::core::nayin::NaYin;
    use crate::core::tengod::TenGod;
    use crate::analysis::interactions::{InteractionAnalyzer, InteractionType};
    use crate::analysis::useful_god::{UsefulGodAnalyzer, UsefulGodResult};
    use crate::core::phase::QiPhase;
    use crate::analysis::void::VoidAnalyzer;
    use crate::analysis::symbolic_stars::SymbolicStarsAnalyzer;
    use chrono::NaiveDateTime;

    #[test]
    fn test_wuxing_cycles() {
        assert_eq!(WuXing::Wood.generates(), WuXing::Fire);
        assert_eq!(WuXing::Wood.controls(), WuXing::Earth);
    }

    #[test]
    fn test_pillar_creation() {
        let p = Pillar::new(TianGan::Jia, DiZhi::Zi);
        assert_eq!(p.element(), WuXing::Wood); // Jia is Wood
    }

    #[test]
    fn test_calendar_calculation() {
        let dt = NaiveDateTime::parse_from_str("2024-11-21 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let calc = Calendar::new();
        let chart = calc.calculate_bazi(dt, "male");

        assert_eq!(chart.year_pillar.tian_gan, TianGan::Jia);
        assert_eq!(chart.year_pillar.di_zhi, DiZhi::Chen);
    }

    #[test]
    fn test_day_pillar_truncation_boundary() {
        let dt = NaiveDateTime::parse_from_str("2000-01-02 01:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let calc = Calendar::new();
        let chart = calc.calculate_bazi(dt, "male");

        // Wu-Wu is 54. Next is 55 -> Ji-Wei.
        assert_eq!(chart.day_pillar.tian_gan, TianGan::Ji);
        assert_eq!(chart.day_pillar.di_zhi, DiZhi::Wei);
    }

    #[test]
    fn test_luck_pillars() {
        let dt = NaiveDateTime::parse_from_str("2024-11-21 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let calc = Calendar::new();
        let chart = calc.calculate_bazi(dt, "male");
        let lp_calc = LuckPillarsCalculator::new(&chart);
        let luck = lp_calc.calculate();

        assert_eq!(luck.len(), 10);
        assert_eq!(luck[0].pillar.tian_gan, TianGan::Bing);
        assert_eq!(luck[0].pillar.di_zhi, DiZhi::Zi);
    }

    #[test]
    fn test_nayin() {
        let p = Pillar::new(TianGan::Jia, DiZhi::Zi);
        let element = NaYin::get_nayin_element(&p);
        assert_eq!(element, WuXing::Metal);
    }

    #[test]
    fn test_strength() {
        let chart_strong = BaziChart::new(
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            "male".to_string(),
            NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let strength_strong = StrengthAnalyzer::new(&chart_strong).analyze();
        assert_eq!(strength_strong, DayMasterStrength::ExtremelyStrong);
    }

    #[test]
    fn test_twelve_palaces() {
        let chart = BaziChart::new(
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Mao), // Month
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin), // Hour
            "male".to_string(),
            NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );

        let calc = TwelvePalacesCalculator::new(&chart);
        let palaces = calc.calculate();

        assert_eq!(palaces.life_palace, DiZhi::Zi);
    }

    #[test]
    fn test_ten_gods() {
        let dm = TianGan::Jia;
        assert_eq!(TenGod::calculate(dm, TianGan::Yi), TenGod::RobWealth);
        assert_eq!(TenGod::calculate(dm, TianGan::Bing), TenGod::EatingGod);
        assert_eq!(TenGod::calculate(dm, TianGan::Geng), TenGod::SevenKillings);
        assert_eq!(TenGod::calculate(dm, TianGan::Ji), TenGod::DirectWealth);
    }

    #[test]
    fn test_interactions() {
        let chart = BaziChart::new(
            Pillar::new(TianGan::Jia, DiZhi::Zi),
            Pillar::new(TianGan::Ji, DiZhi::Chou),
            Pillar::new(TianGan::Bing, DiZhi::Yin),
            Pillar::new(TianGan::Bing, DiZhi::Yin),
            "male".to_string(),
            NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );

        let analyzer = InteractionAnalyzer::new(&chart);
        let results = analyzer.analyze();

        let has_stem_combo = results.iter().any(|r| matches!(r, InteractionType::StemCombination(TianGan::Jia, TianGan::Ji, WuXing::Earth)));
        assert!(has_stem_combo);

        let has_branch_combo = results.iter().any(|r| matches!(r, InteractionType::BranchSixCombination(DiZhi::Zi, DiZhi::Chou, WuXing::Earth)));
        assert!(has_branch_combo);
    }

    #[test]
    fn test_useful_god() {
        let chart = BaziChart::new(
             Pillar::new(TianGan::Jia, DiZhi::Shen),
             Pillar::new(TianGan::Geng, DiZhi::Shen),
             Pillar::new(TianGan::Jia, DiZhi::Shen),
             Pillar::new(TianGan::Geng, DiZhi::Shen),
             "male".to_string(),
             NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );

        let strength = StrengthAnalyzer::new(&chart).analyze();
        assert!(matches!(strength, DayMasterStrength::Weak | DayMasterStrength::ExtremelyWeak));

        let ug_analyzer = UsefulGodAnalyzer::new(strength, &chart);
        let result = ug_analyzer.analyze();

        assert!(result.favorable.contains(&WuXing::Water));
        assert!(result.favorable.contains(&WuXing::Wood));
        assert_eq!(result.useful_god, WuXing::Water);
    }

    #[test]
    fn test_qi_phase() {
        // Jia (Yang Wood) born in Hai (Pig).
        assert_eq!(QiPhase::calculate(TianGan::Jia, DiZhi::Hai), QiPhase::Birth);
        // Jia (Yang Wood) bath in Zi (Rat).
        assert_eq!(QiPhase::calculate(TianGan::Jia, DiZhi::Zi), QiPhase::Bath);
        // Jia (Yang Wood) death in Wu (Horse).
        assert_eq!(QiPhase::calculate(TianGan::Jia, DiZhi::Wu), QiPhase::Death);

        // Yi (Yin Wood) born in Wu (Horse).
        assert_eq!(QiPhase::calculate(TianGan::Yi, DiZhi::Wu), QiPhase::Birth);
        // Yi (Yin Wood) death in Hai (Pig).
        assert_eq!(QiPhase::calculate(TianGan::Yi, DiZhi::Hai), QiPhase::Death);
    }

    #[test]
    fn test_void_branches() {
        // Jia-Zi Pillar.
        // Diff = 0.
        // Void: Xu(10), Hai(11).
        let chart = BaziChart::new(
             Pillar::new(TianGan::Jia, DiZhi::Zi),
             Pillar::new(TianGan::Jia, DiZhi::Zi),
             Pillar::new(TianGan::Jia, DiZhi::Zi), // Day
             Pillar::new(TianGan::Jia, DiZhi::Zi),
             "male".to_string(),
             NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );

        let analyzer = VoidAnalyzer::new(&chart);
        let (v1, v2) = analyzer.get_void_branches();
        assert_eq!(v1, DiZhi::Xu);
        assert_eq!(v2, DiZhi::Hai);
    }

    #[test]
    fn test_expanded_stars() {
        // Test Nobleman (Jia -> Chou/Wei)
        let chart = BaziChart::new(
             Pillar::new(TianGan::Jia, DiZhi::Chou),
             Pillar::new(TianGan::Jia, DiZhi::Zi),
             Pillar::new(TianGan::Jia, DiZhi::Zi), // DM Jia
             Pillar::new(TianGan::Jia, DiZhi::Zi),
             "male".to_string(),
             NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let analyzer = SymbolicStarsAnalyzer::new(&chart);
        let stars = analyzer.analyze();

        assert!(stars.iter().any(|s| s.contains("Nobleman")));

        // Test Academic (Jia -> Si)
        let chart2 = BaziChart::new(
             Pillar::new(TianGan::Jia, DiZhi::Si),
             Pillar::new(TianGan::Jia, DiZhi::Zi),
             Pillar::new(TianGan::Jia, DiZhi::Zi), // DM Jia
             Pillar::new(TianGan::Jia, DiZhi::Zi),
             "male".to_string(),
             NaiveDateTime::parse_from_str("2024-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let analyzer2 = SymbolicStarsAnalyzer::new(&chart2);
        let stars2 = analyzer2.analyze();
        assert!(stars2.iter().any(|s| s.contains("Academic")));
    }
}

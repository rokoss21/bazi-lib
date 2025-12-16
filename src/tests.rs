#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{WuXing, TianGan, DiZhi, Pillar, BaziChart};
    use crate::calendar::calculator::Calendar;
    use crate::analysis::luck::LuckPillarsCalculator;
    use crate::analysis::strength::{StrengthAnalyzer, DayMasterStrength};
    use crate::analysis::twelve_palaces::TwelvePalacesCalculator;
    use crate::core::nayin::NaYin;
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
        // Test with a known date: 2024-11-21 12:00
        // Expected: Year Jia-Chen, Month Yi-Hai, Day Ji-Chou
        let dt = NaiveDateTime::parse_from_str("2024-11-21 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let calc = Calendar::new();
        let chart = calc.calculate_bazi(dt, "male");

        // Year: Jia Chen (2024 is Jia Chen)
        assert_eq!(chart.year_pillar.tian_gan, TianGan::Jia);
        assert_eq!(chart.year_pillar.di_zhi, DiZhi::Chen);

        // Month: Nov 21 is in Pig month (starts Nov 7).
        assert_eq!(chart.month_pillar.tian_gan, TianGan::Yi);
        assert_eq!(chart.month_pillar.di_zhi, DiZhi::Hai);

        // Day: Ji Chou (Verified online for Nov 21 2024)
        assert_eq!(chart.day_pillar.tian_gan, TianGan::Ji);
        assert_eq!(chart.day_pillar.di_zhi, DiZhi::Chou);

        // Hour: 12:00 is Wu (Horse) hour.
        assert_eq!(chart.hour_pillar.di_zhi, DiZhi::Wu);
        assert_eq!(chart.hour_pillar.tian_gan, TianGan::Geng);
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
        // Male born in Yang Year (Jia) -> Forward.
        let dt = NaiveDateTime::parse_from_str("2024-11-21 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let calc = Calendar::new();
        let chart = calc.calculate_bazi(dt, "male");
        // Year is Jia (Yang). Male. -> Forward.
        // Month is Yi-Hai.
        // Next Luck Pillar: Bing-Zi.

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
        assert_eq!(element, WuXing::Metal); // Jia Zi = Sea Metal
    }

    #[test]
    fn test_strength() {
        // 1. Same Season (Strongest)
        // Wood DM in Spring (Tiger/Yin)
        let chart_strong = BaziChart::new(
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin), // Wood Month
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            "male".to_string()
        );
        let strength_strong = StrengthAnalyzer::new(&chart_strong).analyze();
        assert_eq!(strength_strong, DayMasterStrength::ExtremelyStrong);

        // 2. Mother Season (Strong)
        // Wood DM in Winter (Pig/Hai -> Water)
        let chart_mother = BaziChart::new(
            Pillar::new(TianGan::Jia, DiZhi::Hai),
            Pillar::new(TianGan::Jia, DiZhi::Hai), // Water Month (Generates Wood)
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            "male".to_string()
        );
        let strength_mother = StrengthAnalyzer::new(&chart_mother).analyze();
        assert!(matches!(strength_mother, DayMasterStrength::Strong | DayMasterStrength::ExtremelyStrong));

        // 3. Child Season (Weak)
        // Wood DM in Summer (Horse/Wu -> Fire)
        let chart_child = BaziChart::new(
            Pillar::new(TianGan::Wu, DiZhi::Wu),
            Pillar::new(TianGan::Wu, DiZhi::Wu), // Fire Month (Child of Wood) -> Weakens
            Pillar::new(TianGan::Jia, DiZhi::Wu), // Wood DM
            Pillar::new(TianGan::Wu, DiZhi::Wu),
            "male".to_string()
        );
        let strength_child = StrengthAnalyzer::new(&chart_child).analyze();
        assert!(matches!(strength_child, DayMasterStrength::Weak | DayMasterStrength::ExtremelyWeak));
    }

    #[test]
    fn test_twelve_palaces() {
        // Test Palaces
        // Month: Mao (Rabbit) = 2. Hour: Yin (Tiger) = 1.
        // Sum = 3. 14 - 3 = 11.
        // 11 = Zi (Rat) = Life Palace.

        let chart = BaziChart::new(
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Mao), // Month
            Pillar::new(TianGan::Jia, DiZhi::Yin),
            Pillar::new(TianGan::Jia, DiZhi::Yin), // Hour
            "male".to_string()
        );

        let calc = TwelvePalacesCalculator::new(&chart);
        let palaces = calc.calculate();

        assert_eq!(palaces.life_palace, DiZhi::Zi);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{WuXing, TianGan, DiZhi, Pillar, BaziChart};
    use crate::calendar::calculator::Calendar;
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
        // Year Jia -> Month stems start from Bing-Yin.
        // Tiger(1), Rabbit(2)... Pig(10).
        // Sequence: Bing, Ding, Wu, Ji, Geng, Xin, Ren, Gui, Jia, Yi.
        // So Pig month should be Yi-Hai.
        assert_eq!(chart.month_pillar.tian_gan, TianGan::Yi);
        assert_eq!(chart.month_pillar.di_zhi, DiZhi::Hai);

        // Day: Ji Chou (Verified online for Nov 21 2024)
        assert_eq!(chart.day_pillar.tian_gan, TianGan::Ji);
        assert_eq!(chart.day_pillar.di_zhi, DiZhi::Chou);

        // Hour: 12:00 is Wu (Horse) hour (11:00-13:00).
        // Day Ji -> Hour stems start from Jia-Zi.
        // Rat(0), Ox(1), Tiger(2), Rabbit(3), Dragon(4), Snake(5), Horse(6).
        // Jia, Yi, Bing, Ding, Wu, Ji, Geng.
        // So Geng-Wu.
        assert_eq!(chart.hour_pillar.di_zhi, DiZhi::Wu);
        assert_eq!(chart.hour_pillar.tian_gan, TianGan::Geng);
    }

    #[test]
    fn test_day_pillar_truncation_boundary() {
        // Reference: 2000-01-01 was Wu-Wu (Earth Horse).
        // Check 2000-01-02 01:00. Should be next day (Ji-Wei).
        // If we used datetime difference (25 hours since Jan 1 00:00, or 13 hours since Jan 1 12:00),
        // we need to be careful.
        // Ref date is Jan 1 12:00.
        // Target is Jan 2 01:00.
        // Diff is 13 hours. Using integer days, this is 0 days.
        // So it would return Wu-Wu.
        // CORRECT behavior: It is the next CALENDAR day, so it should be Ji-Wei.

        let dt = NaiveDateTime::parse_from_str("2000-01-02 01:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let calc = Calendar::new();
        let chart = calc.calculate_bazi(dt, "male");

        // Wu-Wu is 54. Next is 55 -> Ji-Wei.
        assert_eq!(chart.day_pillar.tian_gan, TianGan::Ji);
        assert_eq!(chart.day_pillar.di_zhi, DiZhi::Wei);
    }
}

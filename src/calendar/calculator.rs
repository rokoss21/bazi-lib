use chrono::{NaiveDateTime, Datelike, Timelike};
use crate::core::{BaziChart, Pillar, TianGan, DiZhi};

// This is the critical part: Converting Date to Pillars.
// 1. Year Pillar: Based on Li Chun (Start of Spring) usually Feb 4.
// 2. Month Pillar: Based on Solar Terms (Jie).
// 3. Day Pillar: Continuous cycle of 60 days. Needs a reference point.
// 4. Hour Pillar: Based on Day Stem and Hour Branch.

// Reference date for Day Pillar:
// Dec 31, 1899 was a Sunday.
// We need a known Gan-Zhi date.
// January 1, 1900:
// 1900 is Geng-Zi year (Start of spring determines when year pillar changes).
// But for Day Pillar, we need a reference.
// Let's use a known anchor:
// Nov 21, 2024 is Jia-Chen (Year), Yi-Hai (Month), Ji-Chou (Day).
// 2024-11-21 12:00 -> Ji-Chou Day.

// We need an algorithm to calculate the Ganzhi for any date.

pub struct Calendar {
    // We could cache solar terms here
}

impl Calendar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_bazi(&self, date: NaiveDateTime, _gender: &str) -> BaziChart {
        // Simplified Logic for Demo:
        // 1. Year Pillar
        // We need to know if the date is before or after Li Chun (approx Feb 4).
        // If before Feb 4, it belongs to previous year.
        let _year = date.year();
        let (year_gan_idx, year_zhi_idx) = self.get_year_gan_zhi(date);

        let year_pillar = Pillar::new(
            TianGan::from_index(year_gan_idx).unwrap(),
            DiZhi::from_index(year_zhi_idx).unwrap()
        );

        // 2. Month Pillar
        // Dependent on Year Stem and Solar Term.
        // We need a function to get the solar term index (0-11) where 0 = Yin month (Tiger), starts ~Feb 4.
        let month_term_idx = self.get_solar_term_index(date); // 0 = Tiger month
        let month_zhi_idx = (month_term_idx + 2) % 12; // Tiger is index 2 in DiZhi (Zi=0, Chou=1, Yin=2)

        // Month Stem rule (Five Tigers Chasing Month):
        // Jia/Ji Year -> Bing Yin month
        // Yi/Geng Year -> Wu Yin month
        // Bing/Xin Year -> Geng Yin month
        // Ding/Ren Year -> Ren Yin month
        // Wu/Gui Year -> Jia Yin month
        // Formula: (YearStemIndex % 5) * 2 + 2 + MonthIndex (0-based from Tiger)
        // Let's verify:
        // Jia (0) -> (0*2 + 2) = 2 (Bing). Correct.
        // Yi (1) -> (1*2 + 2) = 4 (Wu). Correct.

        let year_stem_idx = year_pillar.tian_gan as usize; // enum index matches 0-9
        let month_stem_idx = ((year_stem_idx % 5) * 2 + 2 + month_term_idx) % 10;

        let month_pillar = Pillar::new(
            TianGan::from_index(month_stem_idx).unwrap(),
            DiZhi::from_index(month_zhi_idx).unwrap()
        );

        // 3. Day Pillar
        // Days cycle endlessly.
        // Reference: 1900-01-01 was Monday.
        // We need a specific GanZhi reference.
        // 2024-01-01 was Jia-Zi (No, let's check).
        // 2024-11-21 was Ji-Chou (Day 25 of cycle).
        // Let's assume a function calculate_day_gan_zhi.
        let (day_gan_idx, day_zhi_idx) = self.calculate_day_gan_zhi(date);

        let day_pillar = Pillar::new(
            TianGan::from_index(day_gan_idx).unwrap(),
            DiZhi::from_index(day_zhi_idx).unwrap()
        );

        // 4. Hour Pillar
        // Hour Branch is fixed by time:
        // 23-01: Zi (Rat)
        // 01-03: Chou (Ox)
        // ...
        // 21-23: Hai (Pig)
        let hour = date.hour();
        let _minute = date.minute();
        // Convert to double-hour index. 23:00 is start of Zi (0).
        // (Hour + 1) / 2 % 12
        let hour_zhi_idx = ((hour + 1) / 2) as usize % 12;

        // Hour Stem rule (Five Rats Chasing Hour):
        // Based on Day Stem.
        // Jia/Ji Day -> Jia Zi hour
        // Yi/Geng Day -> Bing Zi hour
        // Bing/Xin Day -> Wu Zi hour
        // Ding/Ren Day -> Geng Zi hour
        // Wu/Gui Day -> Ren Zi hour
        // Formula: (DayStemIndex % 5) * 2 + HourBranchIndex
        // Let's verify:
        // Jia (0) Day, Zi (0) Hour -> (0*2 + 0) = 0 (Jia). Correct.
        // Yi (1) Day, Zi (0) Hour -> (1*2 + 0) = 2 (Bing). Correct.

        let day_stem_idx = day_pillar.tian_gan as usize;
        let hour_stem_idx = ((day_stem_idx % 5) * 2 + hour_zhi_idx) % 10;

        let hour_pillar = Pillar::new(
            TianGan::from_index(hour_stem_idx).unwrap(),
            DiZhi::from_index(hour_zhi_idx).unwrap()
        );

        BaziChart::new(year_pillar, month_pillar, day_pillar, hour_pillar, _gender.to_string(), date)
    }

    fn get_year_gan_zhi(&self, date: NaiveDateTime) -> (usize, usize) {
        // Offset relative to 1984 (Start of a cycle - Jia Zi year).
        // 1984 was Jia Zi.
        // Check if date is before Li Chun of that year.
        let year = date.year();
        let is_before_lichun = self.is_before_lichun(date);

        let mut cal_year = year;
        if is_before_lichun {
            cal_year -= 1;
        }

        let offset = cal_year - 1984;
        // Rust's % operator can return negative for negative numbers, so handle carefully.
        let cycle_idx = (offset % 60 + 60) % 60;

        // Cycle Index 0 = Jia (0) Zi (0)
        // Index 1 = Yi (1) Chou (1)
        // Stem = idx % 10
        // Branch = idx % 12
        ((cycle_idx % 10) as usize, (cycle_idx % 12) as usize)
    }

    fn is_before_lichun(&self, date: NaiveDateTime) -> bool {
        // Simplified: Li Chun is approx Feb 4.
        // In real app, calculate solar longitude 315 degrees.
        // For now:
        if date.month() < 2 {
            return true;
        }
        if date.month() == 2 && date.day() < 4 {
            return true;
        }
        // Very basic approx
        false
    }

    fn get_solar_term_index(&self, date: NaiveDateTime) -> usize {
        // Return 0 for Tiger Month (approx Feb 4 - Mar 5)
        // 1 for Rabbit Month (Mar 5 - Apr 4)
        // ...
        // 11 for Ox Month (Jan 5 - Feb 4)

        // This relies on Month Pillar boundaries (Jie).
        // Approx table:
        // Feb 4: Tiger (2) -> return 0
        // Mar 6: Rabbit (3) -> return 1
        // Apr 5: Dragon (4) -> return 2
        // May 6: Snake (5) -> return 3
        // Jun 6: Horse (6) -> return 4
        // Jul 7: Goat (7) -> return 5
        // Aug 8: Monkey (8) -> return 6
        // Sep 8: Rooster (9) -> return 7
        // Oct 8: Dog (10) -> return 8
        // Nov 7: Pig (11) -> return 9
        // Dec 7: Rat (0) -> return 10
        // Jan 6: Ox (1) -> return 11

        let m = date.month();
        let d = date.day();

        // Rough boundaries
        match m {
            2 => if d >= 4 { 0 } else { 11 },
            3 => if d >= 6 { 1 } else { 0 },
            4 => if d >= 5 { 2 } else { 1 },
            5 => if d >= 6 { 3 } else { 2 },
            6 => if d >= 6 { 4 } else { 3 },
            7 => if d >= 7 { 5 } else { 4 },
            8 => if d >= 8 { 6 } else { 5 },
            9 => if d >= 8 { 7 } else { 6 },
            10 => if d >= 8 { 8 } else { 7 },
            11 => if d >= 7 { 9 } else { 8 },
            12 => if d >= 7 { 10 } else { 9 },
            1 => if d >= 6 { 11 } else { 10 },
            _ => 0
        }
    }

    fn calculate_day_gan_zhi(&self, date: NaiveDateTime) -> (usize, usize) {
        // Reference: 2000-01-01 was Wu-Wu (Earth Horse).
        // 2000-01-01 12:00
        let ref_date = NaiveDateTime::parse_from_str("2000-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        // Wu-Wu is index 54 (if 0-based).

        let ref_idx = 54;

        // Use date() to ignore time component for day pillar calculation
        let diff = date.date().signed_duration_since(ref_date.date()).num_days();

        let mut idx = (ref_idx as i64 + diff) % 60;
        if idx < 0 { idx += 60; }

        ((idx % 10) as usize, (idx % 12) as usize)
    }
}

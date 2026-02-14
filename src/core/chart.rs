use chrono::{NaiveDateTime, Datelike};
use super::tiangan::TianGan;
use super::dizhi::DiZhi;
use super::wuxing::WuXing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pillar {
    pub tian_gan: TianGan,
    pub di_zhi: DiZhi,
}

impl Pillar {
    pub fn new(tian_gan: TianGan, di_zhi: DiZhi) -> Self {
        Self { tian_gan, di_zhi }
    }

    pub fn element(&self) -> WuXing {
        // Usually refers to the NaYin element or the Stem element?
        // README says "pillar.element # WuXing - Pillar element".
        // Often this implies the Na Yin element of the pillar,
        // OR simply the Heavenly Stem element as the "face" of the pillar.
        // Given NaYin has its own calculator in the description, maybe this is just the Stem?
        // But usually "Element of the Pillar" in simple terms = Stem Element.
        self.tian_gan.element()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziChart {
    pub year_pillar: Pillar,
    pub month_pillar: Pillar,
    pub day_pillar: Pillar,
    pub hour_pillar: Pillar,
    pub gender: String, // "male", "female"
    pub birth_date: NaiveDateTime,
}

impl BaziChart {
    pub fn new(year: Pillar, month: Pillar, day: Pillar, hour: Pillar, gender: String, birth_date: NaiveDateTime) -> Self {
        Self {
            year_pillar: year,
            month_pillar: month,
            day_pillar: day,
            hour_pillar: hour,
            gender,
            birth_date,
        }
    }

    pub fn day_master(&self) -> TianGan {
        self.day_pillar.tian_gan
    }

    pub fn birth_year(&self) -> i32 {
        self.birth_date.year()
    }

    pub fn get_element_distribution(&self) -> std::collections::HashMap<WuXing, usize> {
        let mut dist = std::collections::HashMap::new();
        dist.insert(WuXing::Wood, 0);
        dist.insert(WuXing::Fire, 0);
        dist.insert(WuXing::Earth, 0);
        dist.insert(WuXing::Metal, 0);
        dist.insert(WuXing::Water, 0);

        let pillars = [&self.year_pillar, &self.month_pillar, &self.day_pillar, &self.hour_pillar];

        for p in pillars {
            // Add Stem element
            *dist.entry(p.tian_gan.element()).or_default() += 1;

            // Add Branch element (Main Qi)
            *dist.entry(p.di_zhi.element()).or_default() += 1;

            // Note: Some schools calculate distribution based on Hidden Stems percentages.
            // Simplified version: Stem (1 count) + Branch Main Qi (1 count).
            // Or Branch Main Qi counts?
            // The README example shows "Total: 8" which implies 4 pillars x 2 (Stem + Branch) = 8 points.
            // So Stem counts 1, Branch counts 1.
        }

        dist
    }
}

use crate::core::{BaziChart, Pillar, TianGan, DiZhi, Polarity};

#[derive(Debug, Clone)]
pub struct LuckPillar {
    pub pillar: Pillar,
    pub start_age: usize,
    pub end_age: usize,
    pub start_year: i32,
}

pub struct LuckPillarsCalculator<'a> {
    chart: &'a BaziChart,
}

impl<'a> LuckPillarsCalculator<'a> {
    pub fn new(chart: &'a BaziChart) -> Self {
        Self { chart }
    }

    pub fn calculate(&self) -> Vec<LuckPillar> {
        // 1. Determine Direction (Forward or Backward)
        // Yang Male / Yin Female -> Forward
        // Yin Male / Yang Female -> Backward

        let year_stem_polarity = self.chart.year_pillar.tian_gan.polarity();
        let gender = &self.chart.gender; // "male" or "female"

        let forward = match (gender.as_str(), year_stem_polarity) {
            ("male", Polarity::Yang) => true,
            ("female", Polarity::Yin) => true,
            _ => false,
        };

        // 2. Start from Month Pillar
        let start_stem = self.chart.month_pillar.tian_gan as usize; // 0-9
        let start_branch = self.chart.month_pillar.di_zhi as usize; // 0-11

        // 3. Calculate Start Age (Jiao Yun)
        // Strictly speaking, this is diff between Birth Time and nearest Major Solar Term.
        // 3 days = 1 year. 1 day = 4 months. 1 hour = 5 days.
        // For MVP, we can default to starting at age ~3-5 or assume a fixed start if calculation is too heavy without solar term dates.
        // Let's implement a simplified Start Age:
        // Forward: Days from Birth to Next Jie (Solar Term).
        // Backward: Days from Birth to Previous Jie.
        // Since we don't have exact solar term dates in the Chart struct yet, we will default to 4 for now to avoid breaking.
        // TODO: Integrate accurate Solar Term dates for precise Age calculation.
        let start_age = 4;
        let start_year = self.chart.birth_year() + start_age as i32;

        let mut pillars = Vec::new();

        for i in 1..=10 { // Usually calculate 8-10 luck pillars
            let offset = if forward { i as i64 } else { -(i as i64) };

            let mut stem_idx = (start_stem as i64 + offset) % 10;
            if stem_idx < 0 { stem_idx += 10; }

            let mut branch_idx = (start_branch as i64 + offset) % 12;
            if branch_idx < 0 { branch_idx += 12; }

            let pillar = Pillar::new(
                TianGan::from_index(stem_idx as usize).unwrap(),
                DiZhi::from_index(branch_idx as usize).unwrap(),
            );

            let current_start_age = start_age + (i - 1) * 10;

            pillars.push(LuckPillar {
                pillar,
                start_age: current_start_age,
                end_age: current_start_age + 9,
                start_year: start_year + ((i - 1) as i32 * 10),
            });
        }

        pillars
    }
}

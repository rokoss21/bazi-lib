use bazi_lib::core::{BaziChart, TianGan, DiZhi, WuXing};
use bazi_lib::calendar::calculator::Calendar;
use bazi_lib::calendar::solar_time::SolarTimeCalculator;
use bazi_lib::analysis::strength::StrengthAnalyzer;
use bazi_lib::analysis::luck::LuckPillarsCalculator;
use bazi_lib::analysis::useful_god::UsefulGodAnalyzer;
use bazi_lib::analysis::interactions::InteractionAnalyzer;
use bazi_lib::analysis::symbolic_stars::SymbolicStarsAnalyzer;
use bazi_lib::analysis::twelve_palaces::TwelvePalacesCalculator;
use bazi_lib::analysis::void::VoidAnalyzer;
use bazi_lib::core::nayin::NaYin;
use bazi_lib::core::tengod::TenGod;
use bazi_lib::core::phase::QiPhase;
use chrono::NaiveDateTime;
use std::fs::File;
use std::io::Write;

fn main() {
    // Input Data
    let birth_time_str = "1992-01-21 10:50:00";
    let longitude = 36.0667; // Orel, Russia
    let timezone = 3.0; // Moscow Time (UTC+3) roughly.
    // Note: In Jan 1992, Russia might have been UTC+3 or UTC+2.
    // History:
    // Sep 1991: UTC+2.
    // Jan 19, 1992: UTC+3 (Decree Time restored).
    // Jan 21 is after Jan 19, so likely UTC+3.
    let gender = "male";

    let civil_time = NaiveDateTime::parse_from_str(birth_time_str, "%Y-%m-%d %H:%M:%S").unwrap();

    // 1. Calculate True Solar Time
    let solar_time = SolarTimeCalculator::get_true_solar_time(civil_time, longitude, timezone);

    // 2. Generate Chart
    let calendar = Calendar::new();
    let chart = calendar.calculate_bazi(solar_time, gender);

    // 3. Analysis
    let strength_analyzer = StrengthAnalyzer::new(&chart);
    let strength = strength_analyzer.analyze();

    let ug_analyzer = UsefulGodAnalyzer::new(strength, &chart);
    let useful_god = ug_analyzer.analyze();

    let luck_calc = LuckPillarsCalculator::new(&chart);
    let luck_pillars = luck_calc.calculate();

    let stars_analyzer = SymbolicStarsAnalyzer::new(&chart);
    let stars = stars_analyzer.analyze();

    let interaction_analyzer = InteractionAnalyzer::new(&chart);
    let interactions = interaction_analyzer.analyze();

    let palaces_calc = TwelvePalacesCalculator::new(&chart);
    let palaces = palaces_calc.calculate();

    let void_analyzer = VoidAnalyzer::new(&chart);
    let void_branches = void_analyzer.get_void_branches();

    // 4. Format Report
    let mut report = String::new();
    report.push_str(&format!("# BaZi Report for Emil\n\n"));
    report.push_str(&format!("**Birth Date (Civil):** {}\n", birth_time_str));
    report.push_str(&format!("**Birth Date (True Solar):** {}\n", solar_time));
    report.push_str(&format!("**Place:** Orel, Russia (Lon: {:.4})\n", longitude));
    report.push_str(&format!("**Gender:** {}\n\n", gender));

    report.push_str("## 1. The Four Pillars (BaZi Chart)\n\n");
    report.push_str("| Pillar | Year | Month | Day | Hour |\n");
    report.push_str("| :--- | :---: | :---: | :---: | :---: |\n");

    let pillars = [&chart.year_pillar, &chart.month_pillar, &chart.day_pillar, &chart.hour_pillar];

    // Stems
    report.push_str("| **Heavenly Stem** | ");
    for p in pillars {
        report.push_str(&format!("{} ({}) | ", p.tian_gan.chinese_name(), p.tian_gan.element().display_name()));
    }
    report.push_str("\n");

    // Branches
    report.push_str("| **Earthly Branch** | ");
    for p in pillars {
        report.push_str(&format!("{} ({}) | ", p.di_zhi.chinese_name(), p.di_zhi.zodiac_animal()));
    }
    report.push_str("\n");

    // Hidden Stems (Main Qi)
    report.push_str("| **Hidden Stem (Main)** | ");
    for p in pillars {
         let main = p.di_zhi.hidden_stems()[0];
         report.push_str(&format!("{} | ", main.chinese_name()));
    }
    report.push_str("\n");

    // Ten Gods (Stem vs Day Master)
    report.push_str("| **Ten God (Stem)** | ");
    let dm = chart.day_master();
    for p in pillars {
        if p.tian_gan == dm {
             report.push_str("Day Master | ");
        } else {
             let tg = TenGod::calculate(dm, p.tian_gan);
             report.push_str(&format!("{:?} | ", tg));
        }
    }
    report.push_str("\n");

    // Na Yin
    report.push_str("| **Na Yin** | ");
    for p in pillars {
        let ny = NaYin::get_nayin_element(p);
        report.push_str(&format!("{} | ", ny.display_name()));
    }
    report.push_str("\n");

    // Qi Phase
    report.push_str("| **Qi Phase** | ");
    for p in pillars {
        let phase = QiPhase::calculate(dm, p.di_zhi);
        report.push_str(&format!("{:?} | ", phase));
    }
    report.push_str("\n\n");

    report.push_str("## 2. Analysis\n\n");
    report.push_str(&format!("**Day Master:** {} ({})\n", dm.chinese_name(), dm.element().display_name()));
    report.push_str(&format!("**Day Master Strength:** {:?}\n", strength));
    report.push_str(&format!("**Useful God (Yong Shen):** {:?}\n", useful_god.useful_god));
    report.push_str(&format!("**Favorable Elements:** {:?}\n", useful_god.favorable));
    report.push_str(&format!("**Unfavorable Elements:** {:?}\n", useful_god.unfavorable));
    report.push_str(&format!("**Void Branches (Kong Wang):** {:?} / {:?}\n\n", void_branches.0, void_branches.1));

    report.push_str("## 3. Symbolic Stars (Shen Sha)\n\n");
    for star in stars {
        report.push_str(&format!("- {}\n", star));
    }
    report.push_str("\n");

    report.push_str("## 4. Interactions\n\n");
    for interaction in interactions {
        report.push_str(&format!("- {:?}\n", interaction));
    }
    report.push_str("\n");

    report.push_str("## 5. Luck Pillars (Da Yun)\n\n");
    report.push_str("| Age | Year | Stem | Branch | Ten God | Qi Phase |\n");
    report.push_str("| :---: | :---: | :---: | :---: | :---: | :---: |\n");

    for lp in luck_pillars {
        let tg = TenGod::calculate(dm, lp.pillar.tian_gan);
        let phase = QiPhase::calculate(dm, lp.pillar.di_zhi);
        report.push_str(&format!("| {}-{} | {} | {} | {} | {:?} | {:?} |\n",
            lp.start_age, lp.end_age, lp.start_year,
            lp.pillar.tian_gan.chinese_name(), lp.pillar.di_zhi.chinese_name(),
            tg, phase
        ));
    }
    report.push_str("\n");

    report.push_str("## 6. Twelve Palaces (Ming Gong)\n\n");
    report.push_str(&format!("**Life Palace:** {}\n", palaces.life_palace.zodiac_animal()));
    report.push_str(&format!("**Career Palace:** {}\n", palaces.career.zodiac_animal()));
    report.push_str(&format!("**Wealth Palace:** {}\n", palaces.wealth.zodiac_animal()));
    report.push_str(&format!("**Spouse Palace:** {}\n", palaces.spouses.zodiac_animal()));

    // Write to file
    let mut file = File::create("emil.md").unwrap();
    file.write_all(report.as_bytes()).unwrap();
    println!("Report generated in emil.md");
}

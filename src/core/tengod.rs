use crate::core::tiangan::TianGan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenGod {
    Friend,             // Bi Jian (Same Element, Same Polarity)
    RobWealth,          // Jie Cai (Same Element, Different Polarity)
    EatingGod,          // Shi Shen (Generates, Same Polarity)
    HurtingOfficer,     // Shang Guan (Generates, Different Polarity)
    DirectWealth,       // Zheng Cai (Controlled, Different Polarity)
    IndirectWealth,     // Pian Cai (Controlled, Same Polarity)
    DirectOfficer,      // Zheng Guan (Controls Me, Different Polarity)
    SevenKillings,      // Qi Sha (Controls Me, Same Polarity)
    DirectResource,     // Zheng Yin (Generates Me, Different Polarity)
    IndirectResource,   // Pian Yin (Generates Me, Same Polarity)
}

impl TenGod {
    pub fn calculate(day_master: TianGan, target: TianGan) -> Self {
        let dm_elem = day_master.element();
        let target_elem = target.element();
        let same_polarity = day_master.polarity() == target.polarity();

        if dm_elem == target_elem {
            if same_polarity { TenGod::Friend } else { TenGod::RobWealth }
        } else if dm_elem.generates() == target_elem {
            // I generate Target (Output)
            if same_polarity { TenGod::EatingGod } else { TenGod::HurtingOfficer }
        } else if dm_elem.controls() == target_elem {
            // I control Target (Wealth)
            if same_polarity { TenGod::IndirectWealth } else { TenGod::DirectWealth }
        } else if target_elem.controls() == dm_elem {
            // Target controls Me (Power)
            if same_polarity { TenGod::SevenKillings } else { TenGod::DirectOfficer }
        } else {
            // Target generates Me (Resource)
            if same_polarity { TenGod::IndirectResource } else { TenGod::DirectResource }
        }
    }
}

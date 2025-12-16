use crate::analysis::strength::DayMasterStrength;
use crate::core::wuxing::WuXing;
use crate::core::BaziChart;

#[derive(Debug, Clone, PartialEq)]
pub struct UsefulGodResult {
    pub favorable: Vec<WuXing>,
    pub unfavorable: Vec<WuXing>,
    pub useful_god: WuXing, // The most important one
}

pub struct UsefulGodAnalyzer<'a> {
    strength: DayMasterStrength,
    dm_element: WuXing,
    _chart: &'a BaziChart, // Reserved for advanced logic (Climate adjustments)
}

impl<'a> UsefulGodAnalyzer<'a> {
    pub fn new(strength: DayMasterStrength, chart: &'a BaziChart) -> Self {
        Self {
            strength,
            dm_element: chart.day_master().element(),
            _chart: chart,
        }
    }

    pub fn analyze(&self) -> UsefulGodResult {
        // Basic Logic:
        // Weak -> Needs Resource (Mother) and Friend.
        // Strong -> Needs Output (Child), Wealth (Control), Power (Controller).
        // Balanced -> Needs Maintenance? (Context dependent, usually Flow).

        // This is simplified. Real Yong Shen selection involves:
        // 1. Hou (Climate/Temperature) - e.g. Winter Wood needs Fire.
        // 2. Tong Guan (Arbitration) - e.g. Metal vs Wood clash needs Water.
        // 3. Bing Yao (Sickness) - Too much Earth needs Wood.

        // For this implementation, we focus on Balance (Fu Yi).

        match self.strength {
            DayMasterStrength::ExtremelyWeak | DayMasterStrength::Weak => {
                // Needs Strengthen
                let _resource = self.dm_element.weakens(); // The one that produces me (Mother) - wait.
                // Logic check: WuXing::weakens() returns what THIS element weakens.
                // Mother produces Child. Child drains Mother.
                // So "Wood weakens Water". Mother of Wood is Water.
                // We need a helper `mother()` or `generated_by()`.
                // In wuxing.rs we implemented `generates()`.
                // Water generates Wood.
                // We need to find X such that X.generates() == self.dm_element.

                let mother = self.find_mother(self.dm_element);
                let friend = self.dm_element;

                UsefulGodResult {
                    favorable: vec![mother, friend],
                    unfavorable: vec![self.dm_element.generates(), self.dm_element.controls(), self.find_controller(self.dm_element)],
                    useful_god: mother, // Usually Resource is priority for Weak
                }
            },
            DayMasterStrength::ExtremelyStrong | DayMasterStrength::Strong => {
                // Needs Weaken
                let output = self.dm_element.generates();
                let wealth = self.dm_element.controls();
                let power = self.find_controller(self.dm_element);

                UsefulGodResult {
                    favorable: vec![output, wealth, power],
                    unfavorable: vec![self.find_mother(self.dm_element), self.dm_element],
                    useful_god: output, // Usually Output or Power depending on structure.
                }
            },
            DayMasterStrength::Balanced => {
                // Needs Flow.
                // Usually Output is good for flow.
                UsefulGodResult {
                    favorable: vec![self.dm_element.generates()],
                    unfavorable: vec![],
                    useful_god: self.dm_element.generates(),
                }
            }
        }
    }

    fn find_mother(&self, element: WuXing) -> WuXing {
        // X generates Element.
        match element {
            WuXing::Wood => WuXing::Water,
            WuXing::Fire => WuXing::Wood,
            WuXing::Earth => WuXing::Fire,
            WuXing::Metal => WuXing::Earth,
            WuXing::Water => WuXing::Metal,
        }
    }

    fn find_controller(&self, element: WuXing) -> WuXing {
        // X controls Element.
        match element {
            WuXing::Wood => WuXing::Metal,
            WuXing::Fire => WuXing::Water,
            WuXing::Earth => WuXing::Wood,
            WuXing::Metal => WuXing::Fire,
            WuXing::Water => WuXing::Earth,
        }
    }
}

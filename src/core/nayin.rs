use crate::core::Pillar;
use crate::core::wuxing::WuXing;
use crate::core::tiangan::TianGan;
use crate::core::dizhi::DiZhi;

// Na Yin (Melodic Elements) Table
// 60 combinations mapping to an Element (with specific imagery, but we store the element first).

pub struct NaYin;

impl NaYin {
    pub fn get_nayin_element(pillar: &Pillar) -> WuXing {
        // Mapping based on 60 Jia Zi
        // Traditional Formula or Lookup.
        // There is a mathematical pattern, but lookup is safer for completeness.

        let stem = pillar.tian_gan;
        let branch = pillar.di_zhi;

        match (stem, branch) {
            (TianGan::Jia, DiZhi::Zi) | (TianGan::Yi, DiZhi::Chou) => WuXing::Metal, // Sea Metal
            (TianGan::Bing, DiZhi::Yin) | (TianGan::Ding, DiZhi::Mao) => WuXing::Fire, // Furnace Fire
            (TianGan::Wu, DiZhi::Chen) | (TianGan::Ji, DiZhi::Si) => WuXing::Wood, // Forest Wood
            (TianGan::Geng, DiZhi::Wu) | (TianGan::Xin, DiZhi::Wei) => WuXing::Earth, // Road Earth
            (TianGan::Ren, DiZhi::Shen) | (TianGan::Gui, DiZhi::You) => WuXing::Metal, // Sword Metal
            (TianGan::Jia, DiZhi::Xu) | (TianGan::Yi, DiZhi::Hai) => WuXing::Fire, // Mountain Fire

            (TianGan::Bing, DiZhi::Zi) | (TianGan::Ding, DiZhi::Chou) => WuXing::Water, // Mist Water
            (TianGan::Wu, DiZhi::Yin) | (TianGan::Ji, DiZhi::Mao) => WuXing::Earth, // Wall Earth
            (TianGan::Geng, DiZhi::Chen) | (TianGan::Xin, DiZhi::Si) => WuXing::Metal, // White Wax Metal
            (TianGan::Ren, DiZhi::Wu) | (TianGan::Gui, DiZhi::Wei) => WuXing::Wood, // Willow Wood
            (TianGan::Jia, DiZhi::Shen) | (TianGan::Yi, DiZhi::You) => WuXing::Water, // Spring Water
            (TianGan::Bing, DiZhi::Xu) | (TianGan::Ding, DiZhi::Hai) => WuXing::Earth, // Roof Earth

            (TianGan::Wu, DiZhi::Zi) | (TianGan::Ji, DiZhi::Chou) => WuXing::Fire, // Thunder Fire
            (TianGan::Geng, DiZhi::Yin) | (TianGan::Xin, DiZhi::Mao) => WuXing::Wood, // Pine Wood
            (TianGan::Ren, DiZhi::Chen) | (TianGan::Gui, DiZhi::Si) => WuXing::Water, // Flowing Water
            (TianGan::Jia, DiZhi::Wu) | (TianGan::Yi, DiZhi::Wei) => WuXing::Metal, // Sand Metal
            (TianGan::Bing, DiZhi::Shen) | (TianGan::Ding, DiZhi::You) => WuXing::Fire, // Mountain Fire (Different)
            (TianGan::Wu, DiZhi::Xu) | (TianGan::Ji, DiZhi::Hai) => WuXing::Wood, // Plain Wood

            (TianGan::Geng, DiZhi::Zi) | (TianGan::Xin, DiZhi::Chou) => WuXing::Earth, // Wall Earth
            (TianGan::Ren, DiZhi::Yin) | (TianGan::Gui, DiZhi::Mao) => WuXing::Metal, // Foil Metal
            (TianGan::Jia, DiZhi::Chen) | (TianGan::Yi, DiZhi::Si) => WuXing::Fire, // Lamp Fire
            (TianGan::Bing, DiZhi::Wu) | (TianGan::Ding, DiZhi::Wei) => WuXing::Water, // Heaven Water
            (TianGan::Wu, DiZhi::Shen) | (TianGan::Ji, DiZhi::You) => WuXing::Earth, // Highway Earth
            (TianGan::Geng, DiZhi::Xu) | (TianGan::Xin, DiZhi::Hai) => WuXing::Metal, // Hairpin Metal

            (TianGan::Ren, DiZhi::Zi) | (TianGan::Gui, DiZhi::Chou) => WuXing::Wood, // Mulberry Wood
            (TianGan::Jia, DiZhi::Yin) | (TianGan::Yi, DiZhi::Mao) => WuXing::Water, // Great Stream Water
            (TianGan::Bing, DiZhi::Chen) | (TianGan::Ding, DiZhi::Si) => WuXing::Earth, // Sand Earth
            (TianGan::Wu, DiZhi::Wu) | (TianGan::Ji, DiZhi::Wei) => WuXing::Fire, // Heaven Fire
            (TianGan::Geng, DiZhi::Shen) | (TianGan::Xin, DiZhi::You) => WuXing::Wood, // Pomegranate Wood
            (TianGan::Ren, DiZhi::Xu) | (TianGan::Gui, DiZhi::Hai) => WuXing::Water, // Ocean Water

            _ => WuXing::Water, // Fallback/Error
        }
    }
}

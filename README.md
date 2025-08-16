# 🌟 BaZi Library

**Professional Python library for Chinese BaZi astrology (四柱八字) - Complete Four Pillars of Destiny system with element analysis, personality insights, career guidance, and relationship compatibility.**

[![Python Version](https://img.shields.io/badge/python-3.7%2B-blue.svg)](https://python.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-green.svg)](LICENSE)
[![Documentation](https://img.shields.io/badge/docs-complete-blue.svg)](API_REFERENCE.md)
[![Website](https://img.shields.io/badge/Website-astrovisor.io-purple?style=for-the-badge&logo=globe)](https://astrovisor.io/)

**🚀 Production Ready | 🌍 International | 🏛️ Professional | 📚 Comprehensive**

[Features](#-features) • [Documentation](#-documentation) • [Examples](#-examples)


---

## 👨‍💻 Author


**Emil Rokossovsiy**  
*Professional Astrology Software Developer*

[![GitHub](https://img.shields.io/badge/GitHub-rokoss21-black?style=for-the-badge&logo=github)](https://github.com/rokoss21/)
[![Email](https://img.shields.io/badge/Email-ecsiar%40gmail.com-red?style=for-the-badge&logo=gmail)](mailto:ecsiar@gmail.com)
[![Website](https://img.shields.io/badge/Website-astrovisor.io-purple?style=for-the-badge&logo=globe)](https://astrovisor.io/)


---

## 🌟 About

**BaZi Library** is a comprehensive, production-ready Python implementation of the ancient Chinese BaZi (四柱八字) astrology system, also known as "Four Pillars of Destiny." This library provides professional-grade tools for creating and analyzing BaZi charts, including advanced element analysis, personality interpretation, career guidance, and compatibility matching.

### 🎯 **What is BaZi?**
BaZi is a traditional Chinese astrological system that analyzes the four pillars of birth (year, month, day, and hour) to reveal personality traits, life patterns, and destiny influences. It's based on the interaction of the Five Elements (Wu Xing), Heavenly Stems (Tian Gan), and Earthly Branches (Di Zhi).

### 🚀 **Why Choose BaZi Library?**
- ✅ **Production Ready** - Comprehensive testing and professional documentation
- ✅ **International** - Full English documentation and API
- ✅ **Professional** - Enterprise-grade code quality and architecture
- ✅ **Comprehensive** - Complete BaZi analysis system
- ✅ **Extensible** - Modular design for easy customization
- ✅ **Well Documented** - Extensive examples and API reference

### 🌐 **Try the Engine in Action**
**Experience the full power of our BaZi analysis engine on [astrovisor.io](https://astrovisor.io/) - where you can create charts, get detailed interpretations, and explore all the library's capabilities through an intuitive web interface.**

---

## ⚡ Features


### 🏛️ **Core BaZi System**
[![Core](https://img.shields.io/badge/Core-BaZi%20Charts-blue?style=for-the-badge)](https://github.com/rokoss21/bazi-lib#core-module)
[![Elements](https://img.shields.io/badge/Elements-Wu%20Xing-green?style=for-the-badge)](https://github.com/rokoss21/bazi-lib#elements-module)
[![Analysis](https://img.shields.io/badge/Analysis-Complete-orange?style=for-the-badge)](https://github.com/rokoss21/bazi-lib#analysis-features)


#### 🌟 **Chart Creation & Analysis**
- **Four Pillars Calculation** - Automatic computation of year, month, day, and hour pillars with astronomical precision
- **Element Distribution** - Comprehensive Wu Xing (Five Elements) analysis with balance calculations
- **Hidden Stems** - Advanced analysis of concealed heavenly stems and their influence
- **Luck Pillars** - Fortune prediction and life cycle analysis with timing calculations

#### 🔮 **Advanced Interpretations**
- **Personality Analysis** - Deep character insights based on day master and element balance
- **Career Guidance** - Professional recommendations and industry suggestions based on elemental strengths
- **Relationship Compatibility** - Comprehensive compatibility analysis between two charts with scoring
- **Health Insights** - Element balance analysis and health recommendations

#### 🏛️ **Traditional Components**
- **Symbolic Stars (Shen Sha)** - Analysis of auspicious and inauspicious influences with detailed interpretations
- **Twelve Palaces** - Life aspect analysis covering career, wealth, relationships, health, and more
- **Na Yin (Melodic Elements)** - Spiritual development guidance and career path recommendations
- **Useful God** - Determination of favorable directions, elements, and life strategies

#### 🛠️ **Professional Tools**
- **Calendar Support** - Chinese calendar and solar terms integration with astronomical accuracy
- **Timezone Handling** - Precise global time calculations and daylight saving time support
- **Performance Optimization** - Intelligent caching and lazy loading for memory efficiency
- **Error Handling** - Comprehensive exception management with detailed error messages

#### 🌍 **International Features**
- **Multi-language Support** - English, Chinese characters, and Pinyin support
- **Cultural Context** - Traditional Chinese astrology principles with modern interpretations
- **Global Compatibility** - Works with any timezone and geographical location
- **Professional Standards** - Enterprise-grade code quality and documentation

---


## 🏗️ Architecture


### 📁 **Module Structure**
```
bazi_lib/
├── 🏗️ core/           # Core BaZi functionality
├── 📅 calendar/        # Chinese calendar & solar terms
├── 🧩 elements/        # Element analysis tools
├── 🧠 interpretation/  # Personality & career analysis
├── 🏛️ pillars/         # Pillar management
└── 🛠️ utils/          # Utility functions
```


### 🏗️ **Core Module (`bazi_lib.core`)**
The heart of the library containing all essential BaZi classes and analyzers.

#### **Main Classes**
- **`BaziChart`** - Primary chart creation and analysis class with comprehensive methods
- **`TianGan`** - Heavenly Stems (天干) management with element and polarity properties
- **`DiZhi`** - Earthly Branches (地支) management with animal associations and hidden stems
- **`WuXing`** - Five Elements (五行) system with generation and control cycles
- **`TenGod`** - Ten Gods analysis system for relationship dynamics

#### **Analyzers**
- **`SymbolicStarsAnalyzer`** - Shen Sha (神煞) analysis with detailed interpretations
- **`TwelvePalacesCalculator`** - Twelve Palaces calculation for life aspect analysis
- **`NaYinCalculator`** - Na Yin (纳音) analysis for spiritual and career guidance
- **`UsefulGodAnalyzer`** - Useful God determination for favorable directions
- **`CompatibilityAnalyzer`** - Relationship compatibility with scoring system
- **`HiddenStemsAnalyzer`** - Hidden stems analysis for deeper chart insights
- **`LuckPillarsCalculator`** - Luck pillars calculation for timing predictions

### 📅 **Calendar Module (`bazi_lib.calendar`)**
- **`ChineseCalendar`** - Chinese calendar conversion with cycle calculations
- **`SolarTerms`** - 24 solar terms calculation with astronomical precision

### 🧠 **Interpretation Module (`bazi_lib.interpretation`)**
- **`PersonalityAnalyzer`** - Day master personality analysis with trait mapping
- **`CareerAnalyzer`** - Career potential and recommendations based on elements

---

## 🔮 **Advanced Capabilities**

### **Element Analysis System**
- **Wu Xing Cycles** - Complete generation, control, and weakening cycles
- **Element Balance** - Automatic calculation of element distribution and balance
- **Favorable Elements** - Identification of beneficial elements for personal growth
- **Element Interactions** - Detailed analysis of how elements affect each other

### **Personality & Career Analysis**
- **Day Master Analysis** - Deep personality insights based on birth day element
- **Career Mapping** - Industry recommendations based on elemental strengths
- **Life Path Guidance** - Direction suggestions for personal development
- **Strength Assessment** - Evaluation of personal elemental balance

### **Relationship & Compatibility**
- **Chart Comparison** - Detailed analysis of two birth charts
- **Compatibility Scoring** - Numerical scoring system (0-100) for relationships
- **Element Harmony** - Analysis of elemental compatibility between partners
- **Timing Insights** - Best periods for relationship development

### **Traditional Astrology Features**
- **Solar Terms** - 24 traditional Chinese solar terms with precise calculations
- **Chinese Calendar** - Traditional calendar system integration
- **Astronomical Accuracy** - Precise calculations based on astronomical data
- **Cultural Context** - Traditional interpretations with modern applications

---

# 📚 BaZi Library - Complete API Reference

---

## 🏗️ Core Module - bazi_lib.core

### BaziChart Class

Main class for creating and analyzing BaZi charts.

#### Constructor
```python
BaziChart(
    birth_date: datetime,
    timezone: str = 'UTC',
    longitude: float = 0.0,
    gender: str = 'unknown'
)
```

**Parameters:**
- `birth_date`: Birth date and time (datetime)
- `timezone`: Timezone (str), default 'UTC'
- `longitude`: Birth place longitude (float), default 0.0
- `gender`: Gender ('male', 'female', 'unknown'), default 'unknown'

#### Properties
```python
# Main properties
chart.day_master           # TianGan - Day Master
chart.day_master_element   # WuXing - Day Master element
chart.year_pillar         # Pillar - Year pillar
chart.month_pillar        # Pillar - Month pillar
chart.day_pillar          # Pillar - Day pillar
chart.hour_pillar         # Pillar - Hour pillar

# Additional properties
chart.birth_date          # datetime - Birth date
chart.timezone            # str - Timezone
chart.longitude           # float - Longitude
chart.gender              # str - Gender
```

#### Methods

##### 1. get_complete_analysis()
```python
analysis = chart.get_complete_analysis()
```
**Returns:** dict - Complete BaZi chart analysis

**Contains:**
- `day_master`: Day master information
- `element_distribution`: Element distribution
- `hidden_stems`: Hidden stems
- `luck_pillars`: Luck pillars
- `symbolic_stars`: Symbolic stars
- `twelve_palaces`: Twelve palaces
- `na_yin`: Na Yin
- `useful_god`: Useful god
- `special_combinations`: Special combinations

##### 2. get_element_distribution()
```python
distribution = chart.get_element_distribution()
```
**Returns:** dict - Distribution of the five elements

**Example output:**
```python
{
    'WOOD': 3,
    'FIRE': 2,
    'EARTH': 1,
    'METAL': 2,
    'WATER': 0,
    'total': 8
}
```

##### 3. get_hidden_stems()
```python
hidden = chart.get_hidden_stems()
```
**Returns:** dict - Hidden stems analysis

**Contains:**
- `total_hidden_stems`: Total number of hidden stems
- `hidden_stems_by_pillar`: Hidden stems by pillar
- `analysis`: Analysis of hidden stems influence

##### 4. get_luck_pillars()
```python
luck = chart.get_luck_pillars()
```
**Returns:** dict - Luck pillars

**Contains:**
- `pillars`: List of luck pillars
- `analysis`: Luck pillars analysis
- `favorable_elements`: Favorable elements

##### 5. get_symbolic_stars()
```python
stars = chart.get_symbolic_stars()
```
**Returns:** dict - Symbolic stars

**Contains:**
- `stars`: List of found stars
- `analysis`: Analysis of stars influence
- `categories`: Star categories

##### 6. get_twelve_palaces()
```python
palaces = chart.get_twelve_palaces()
```
**Returns:** dict - Twelve palaces

**Contains:**
- `palaces`: List of all palaces
- `career_palace`: Career palace
- `wealth_palace`: Wealth palace
- `relationships_palace`: Relationships palace
- `health_palace`: Health palace

##### 7. get_na_yin()
```python
na_yin = chart.get_na_yin()
```
**Returns:** dict - Na Yin (melodic elements)

**Contains:**
- `element`: Na Yin element
- `analysis`: Influence analysis
- `career_guidance`: Career guidance
- `spiritual_guidance`: Spiritual guidance

##### 8. get_useful_god()
```python
useful_god = chart.get_useful_god()
```
**Returns:** dict - Useful god

**Contains:**
- `type`: Useful god type
- `analysis`: Influence analysis
- `recommendations`: Recommendations

##### 9. get_special_combinations()
```python
combinations = chart.get_special_combinations()
```
**Returns:** dict - Special combinations

**Contains:**
- `combinations`: List of found combinations
- `analysis`: Influence analysis
- `strength`: Combination strength

---

### TianGan Class (Heavenly Stems)

#### Enum Values
```python
class TianGan(Enum):
    JIA = "甲"    # Wood Yang
    YI = "乙"     # Wood Yin
    BING = "丙"   # Fire Yang
    DING = "丁"   # Fire Yin
    WU = "戊"     # Earth Yang
    JI = "己"     # Earth Yin
    GENG = "庚"   # Metal Yang
    XIN = "辛"    # Metal Yin
    REN = "壬"    # Water Yang
    GUI = "癸"    # Water Yin
```

#### Properties
```python
tian_gan.display_name    # str - English name
tian_gan.element         # WuXing - Associated element
tian_gan.polarity        # Polarity - Yang/Yin polarity
tian_gan.chinese_name    # str - Chinese symbol
```

#### Methods
```python
tian_gan.represents()    # str - What it represents
tian_gan.english_name    # str - English name (alias)
```

---

### DiZhi Class (Earthly Branches)

#### Enum Values
```python
class DiZhi(Enum):
    ZI = "子"     # Rat
    CHOU = "丑"   # Ox
    YIN = "寅"    # Tiger
    MAO = "卯"    # Rabbit
    CHEN = "辰"   # Dragon
    SI = "巳"     # Snake
    WU = "午"     # Horse
    WEI = "未"    # Goat
    SHEN = "申"   # Monkey
    YOU = "酉"    # Rooster
    XU = "戌"     # Dog
    HAI = "亥"    # Pig
```

#### Properties
```python
di_zhi.display_name      # str - Animal name
di_zhi.element           # WuXing - Associated element
di_zhi.polarity          # Polarity - Yang/Yin polarity
di_zhi.chinese_name      # str - Chinese symbol
di_zhi.hidden_stems      # List[TianGan] - Hidden stems
```

#### Methods
```python
di_zhi.get_hidden_stems()    # List[TianGan] - Get hidden stems
```

---

### WuXing Class (Five Elements)

#### Enum Values
```python
class WuXing(Enum):
    WOOD = "Wood"     # Wood
    FIRE = "Fire"     # Fire
    EARTH = "Earth"   # Earth
    METAL = "Metal"   # Metal
    WATER = "Water"   # Water
```

#### Properties
```python
wu_xing.display_name   # str - Element name
wu_xing.generates      # WuXing - Element it generates
wu_xing.controls       # WuXing - Element it controls
wu_xing.weakens        # WuXing - Element it weakens
```

#### Methods
```python
wu_xing.get_cycle()    # dict - Get element cycle
```

---

### TenGod Class (Ten Gods)

#### Enum Values
```python
class TenGod(Enum):
    COMPARISON = "Comparison"           # Comparison
    HARM_WEALTH = "Harm Wealth"        # Harm to wealth
    DIRECT_OFFICIAL = "Direct Official" # Direct official
    DIRECT_WEALTH = "Direct Wealth"     # Direct wealth
    EATING_GOD = "Eating God"          # Eating god
    FRIEND = "Friend"                   # Friend
    HARM_OFFICIAL = "Harm Official"     # Harm to official
    INDIRECT_OFFICIAL = "Indirect Official" # Indirect official
    INDIRECT_WEALTH = "Indirect Wealth" # Indirect wealth
    SEVEN_KILLINGS = "Seven Killings"   # Seven killings
```

#### Properties
```python
ten_god.display_name   # str - English name
ten_god.represents     # str - What it represents
ten_god.english_name   # str - English name (alias)
```

---

### SymbolicStarsAnalyzer Class

Analyzer for symbolic stars (Shen Sha).

#### Constructor
```python
SymbolicStarsAnalyzer(chart: BaziChart)
```

#### Methods

##### analyze_symbolic_stars()
```python
stars = analyzer.analyze_symbolic_stars()
```
**Returns:** dict - Symbolic stars analysis

**Contains:**
- `stars`: List of found stars
- `analysis`: Influence analysis
- `categories`: Star categories

---

### TwelvePalacesCalculator Class

Calculator for twelve palaces.

#### Constructor
```python
TwelvePalacesCalculator(chart: BaziChart)
```

#### Methods

##### calculate_twelve_palaces()
```python
palaces = calculator.calculate_twelve_palaces()
```
**Returns:** TwelvePalacesAnalysis - Twelve palaces analysis

**Contains:**
- `career_palace`: Career palace
- `wealth_palace`: Wealth palace
- `relationships_palace`: Relationships palace
- `health_palace`: Health palace
- `brothers_palace`: Brothers palace
- `travel_palace`: Travel palace
- `children_palace`: Children palace
- `parents_palace`: Parents palace
- `property_palace`: Property palace
- `friends_palace`: Friends palace
- `spouse_palace`: Spouse palace
- `fame_palace`: Fame palace

---

### NaYinCalculator Class

Calculator for Na Yin (melodic elements).

#### Constructor
```python
NaYinCalculator(chart: BaziChart)
```

#### Methods

##### calculate_na_yin()
```python
na_yin = calculator.calculate_na_yin()
```
**Returns:** NaYinAnalysis - Na Yin analysis

**Contains:**
- `element`: Na Yin element
- `analysis`: Influence analysis
- `career_guidance`: Career guidance
- `spiritual_guidance`: Spiritual guidance

---

### UsefulGodAnalyzer Class

Analyzer for useful god.

#### Constructor
```python
UsefulGodAnalyzer(chart: BaziChart)
```

#### Methods

##### analyze_useful_god()
```python
useful_god = analyzer.analyze_useful_god()
```
**Returns:** UsefulGodAnalysis - Useful god analysis

**Contains:**
- `type`: Useful god type
- `analysis`: Influence analysis
- `recommendations`: Recommendations

---

### CompatibilityAnalyzer Class

Analyzer for compatibility between two charts.

#### Constructor
```python
CompatibilityAnalyzer()
```

#### Methods

##### analyze_compatibility(chart1: BaziChart, chart2: BaziChart)
```python
compatibility = analyzer.analyze_compatibility(chart1, chart2)
```
**Returns:** CompatibilityAnalysis - Compatibility analysis

**Contains:**
- `score`: Overall compatibility score (0-100)
- `type`: Compatibility type
- `element_harmony`: Element harmony
- `day_master_compatibility`: Day master compatibility
- `recommendations`: Recommendations

---

### HiddenStemsAnalyzer Class

Analyzer for hidden stems.

#### Constructor
```python
HiddenStemsAnalyzer(chart: BaziChart)
```

#### Methods

##### analyze_hidden_stems()
```python
hidden = analyzer.analyze_hidden_stems()
```
**Returns:** HiddenStemsAnalysis - Hidden stems analysis

**Contains:**
- `total_hidden_stems`: Total number of hidden stems
- `hidden_stems_by_pillar`: Hidden stems by pillar
- `analysis`: Influence analysis

---

### LuckPillarsCalculator Class

Calculator for luck pillars.

#### Constructor
```python
LuckPillarsCalculator(chart: BaziChart)
```

#### Methods

##### calculate_luck_pillars()
```python
luck = calculator.calculate_luck_pillars()
```
**Returns:** LuckPillarsAnalysis - Luck pillars analysis

**Contains:**
- `pillars`: List of luck pillars
- `analysis`: Luck pillars analysis
- `favorable_elements`: Favorable elements

---

### SpecialCombinationsAnalyzer Class

Analyzer for special combinations.

#### Constructor
```python
SpecialCombinationsAnalyzer(chart: BaziChart)
```

#### Methods

##### analyze_special_combinations()
```python
combinations = analyzer.analyze_special_combinations()
```
**Returns:** SpecialCombinationsAnalysis - Special combinations analysis

**Contains:**
- `combinations`: List of found combinations
- `analysis`: Influence analysis
- `strength`: Combination strength

---

### DynamicAnalyzer Class

Analyzer for dynamic influences.

#### Constructor
```python
DynamicAnalyzer(chart: BaziChart)
```

#### Methods

##### analyze_dynamic_influences()
```python
influences = analyzer.analyze_dynamic_influences()
```
**Returns:** List[DynamicInfluence] - List of dynamic influences

**Contains:**
- `type`: Influence type
- `strength`: Influence strength
- `description`: Influence description

---

## 📅 Calendar Module - bazi_lib.calendar

### ChineseCalendar Class

Chinese calendar.

#### Methods

##### get_chinese_date(date: datetime)
```python
chinese_date = calendar.get_chinese_date(date)
```
**Returns:** dict - Chinese date

**Contains:**
- `year`: Year in Chinese calendar
- `month`: Month in Chinese calendar
- `day`: Day in Chinese calendar
- `cycle`: Year cycle

---

### SolarTerms Class

Solar terms.

#### Methods

##### get_solar_term(date: datetime)
```python
solar_term = solar_terms.get_solar_term(date)
```
**Returns:** str - Solar term name

---

## 🧩 Elements Module - bazi_lib.elements

### ElementAnalyzer Class

Element analyzer.

#### Methods

##### analyze_element_balance(elements: List[WuXing])
```python
balance = analyzer.analyze_element_balance(elements)
```
**Returns:** dict - Element balance analysis

**Contains:**
- `strongest`: Strongest element
- `weakest`: Weakest element
- `recommendations`: Balance recommendations

---

## 🧠 Interpretation Module - bazi_lib.interpretation

### PersonalityAnalyzer Class

Personality analyzer.

#### Methods

##### analyze_day_master_personality(day_master: TianGan)
```python
personality = analyzer.analyze_day_master_personality(day_master)
```
**Returns:** dict - Personality analysis by day master

**Contains:**
- `core_traits`: Core personality traits
- `strengths`: Strengths
- `weaknesses`: Weaknesses
- `recommendations`: Recommendations

---

### CareerAnalyzer Class

Career analyzer.

#### Methods

##### analyze_career_potential(day_master: TianGan, element_balance: dict)
```python
career = analyzer.analyze_career_potential(day_master, element_balance)
```
**Returns:** dict - Career potential analysis

**Contains:**
- `primary_career_areas`: Primary career areas
- `secondary_career_areas`: Secondary career areas
- `favorable_industries`: Favorable industries
- `recommendations`: Career recommendations

---

## 🏛️ Pillars Module - bazi_lib.pillars

### Pillar Class

BaZi pillar.

#### Properties
```python
pillar.tian_gan    # TianGan - Heavenly stem
pillar.di_zhi      # DiZhi - Earthly branch
pillar.element     # WuXing - Pillar element
pillar.polarity    # Polarity - Pillar polarity
```

#### Methods

##### get_hidden_stems()
```python
hidden_stems = pillar.get_hidden_stems()
```
**Returns:** List[TianGan] - Hidden stems in pillar

---

## 🛠️ Utils Module - bazi_lib.utils

### Helpers Class

Helper functions.

#### Methods

##### format_chart_display(chart: BaziChart)
```python
display = helpers.format_chart_display(chart)
```
**Returns:** str - Formatted chart display

##### validate_birth_date(date: datetime)
```python
is_valid = helpers.validate_birth_date(date)
```
**Returns:** bool - Birth date validity

---

## 📖 Usage Examples

### Basic Chart Creation and Analysis

```python
from bazi_lib.core import BaziChart
from datetime import datetime

# Create chart
chart = BaziChart(
    birth_date=datetime(1990, 5, 15, 14, 30),
    timezone='Asia/Shanghai',
    longitude=121.4737,
    gender='male'
)

# Complete analysis
analysis = chart.get_complete_analysis()

# Element analysis
element_dist = chart.get_element_distribution()
print(f"Element distribution: {element_dist}")

# Hidden stems analysis
hidden = chart.get_hidden_stems()
print(f"Hidden stems: {hidden['total_hidden_stems']}")

# Luck pillars analysis
luck = chart.get_luck_pillars()
print(f"Luck pillars: {len(luck['pillars'])}")
```

### Advanced Analysis

```python
from bazi_lib.core import (
    SymbolicStarsAnalyzer,
    TwelvePalacesCalculator,
    NaYinCalculator,
    CompatibilityAnalyzer
)

# Symbolic stars analysis
stars_analyzer = SymbolicStarsAnalyzer(chart)
stars = stars_analyzer.analyze_symbolic_stars()
print(f"Symbolic stars: {len(stars['stars'])}")

# Twelve palaces analysis
palaces_calc = TwelvePalacesCalculator(chart)
palaces = palaces_calc.calculate_twelve_palaces()
print(f"Career palace: {palaces.career_palace.palace_type.display_name}")

# Na Yin analysis
na_yin_calc = NaYinCalculator(chart)
na_yin = na_yin_calc.calculate_na_yin()
print(f"Na Yin: {na_yin.element.display_name}")

# Compatibility analysis
chart2 = BaziChart(
    birth_date=datetime(1992, 8, 20, 10, 15),
    timezone='Asia/Shanghai',
    longitude=121.4737,
    gender='female'
)

compat_analyzer = CompatibilityAnalyzer()
compatibility = compat_analyzer.analyze_compatibility(chart, chart2)
print(f"Compatibility score: {compatibility.score}/100")
```

### Personality and Career Analysis

```python
from bazi_lib.interpretation import PersonalityAnalyzer, CareerAnalyzer

# Personality analysis
personality_analyzer = PersonalityAnalyzer()
personality = personality_analyzer.analyze_day_master_personality(chart.day_master)
print(f"Core traits: {personality['core_traits']}")

# Career analysis
career_analyzer = CareerAnalyzer()
career = career_analyzer.analyze_career_potential(
    chart.day_master, 
    chart.get_element_distribution()
)
print(f"Career areas: {career['primary_career_areas']}")
```

### Working with Elements and Combinations

```python
from bazi_lib.core import WuXing, TianGan, DiZhi

# Working with elements
wood = WuXing.WOOD
print(f"Wood generates: {wood.generates.display_name}")
print(f"Wood controls: {wood.controls.display_name}")

# Working with heavenly stems
jia = TianGan.JIA
print(f"Jia element: {jia.element.display_name}")
print(f"Jia polarity: {jia.polarity.display_name}")

# Working with earthly branches
zi = DiZhi.ZI
print(f"Zi animal: {zi.display_name}")
print(f"Zi element: {zi.element.display_name}")
print(f"Zi hidden stems: {[stem.chinese_name for stem in zi.hidden_stems]}")
```

### Error Handling

```python
try:
    # Create chart with invalid data
    chart = BaziChart(
        birth_date="invalid_date",  # Error: datetime expected
        timezone='Invalid/Timezone',  # Error: invalid timezone
        longitude=200,  # Error: longitude out of range
        gender='invalid'  # Error: invalid gender
    )
except ValueError as e:
    print(f"Validation error: {e}")
except Exception as e:
    print(f"Unexpected error: {e}")
```

### Performance Optimization

```python
# Enable caching for repeated calculations
chart.enable_caching = True

# Lazy loading for memory efficiency
chart.lazy_loading = True

# Clear cache when needed
chart.clear_cache()

# Get only needed data instead of complete analysis
element_dist = chart.get_element_distribution()  # Faster than get_complete_analysis()
hidden_stems = chart.get_hidden_stems()         # Faster than get_complete_analysis()
```

---

## 🚨 Error Handling

### Common Exceptions

#### ValueError
- **Invalid birth date**: Invalid birth date
- **Invalid timezone**: Invalid timezone
- **Invalid longitude**: Longitude out of range (-180 to 180)
- **Invalid gender**: Invalid gender (must be 'male', 'female' or 'unknown')

#### TypeError
- **Wrong parameter types**: Wrong parameter types
- **Missing required parameters**: Missing required parameters

#### RuntimeError
- **Calculation errors**: Calculation errors
- **Data corruption**: Data corruption

### Error Handling Best Practices

```python
def safe_bazi_analysis(birth_date, timezone, longitude, gender):
    try:
        # Input validation
        if not isinstance(birth_date, datetime):
            raise ValueError("birth_date must be a datetime object")
        
        if longitude < -180 or longitude > 180:
            raise ValueError("longitude must be between -180 and 180")
        
        # Create chart
        chart = BaziChart(birth_date, timezone, longitude, gender)
        
        # Analysis
        analysis = chart.get_complete_analysis()
        return analysis
        
    except ValueError as e:
        print(f"Validation error: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error: {e}")
        return None
```

---

## 📊 Performance Considerations

### Memory Usage
- **Basic chart**: ~1-5 MB
- **Complete analysis**: ~10-20 MB
- **Multiple charts**: Linear increase

### Execution Time
- **Chart creation**: < 100ms
- **Element analysis**: < 50ms
- **Complete analysis**: < 500ms
- **Compatibility analysis**: < 200ms

### Optimization Tips
1. **Use caching** for repeated calculations
2. **Enable lazy loading** for memory efficiency
3. **Call specific methods** instead of complete analysis
4. **Clear cache** when memory is limited
5. **Use appropriate data types** for large datasets

---

## 🔧 Advanced Usage

### Custom Analyzers

```python
class CustomAnalyzer:
    def __init__(self, chart: BaziChart):
        self.chart = chart
    
    def analyze_custom_pattern(self):
        # Your analysis logic
        pass

# Usage
custom_analyzer = CustomAnalyzer(chart)
result = custom_analyzer.analyze_custom_pattern()
```

### Batch Processing

```python
def analyze_multiple_charts(charts_data):
    results = []
    for data in charts_data:
        try:
            chart = BaziChart(**data)
            analysis = chart.get_complete_analysis()
            results.append(analysis)
        except Exception as e:
            print(f"Error processing chart: {e}")
            results.append(None)
    return results

# Usage
charts_data = [
    {'birth_date': datetime(1990, 5, 15, 14, 30), 'timezone': 'UTC', 'longitude': 0, 'gender': 'male'},
    {'birth_date': datetime(1992, 8, 20, 10, 15), 'timezone': 'UTC', 'longitude': 0, 'gender': 'female'},
]

results = analyze_multiple_charts(charts_data)
```

---

## 📄 License

<div align="center">

**This project is licensed under the Apache License 2.0**

[![License](https://img.shields.io/badge/License-Apache%202.0-green.svg)](LICENSE)

</div>

**Apache License 2.0** provides:
- ✅ **Commercial use** - You can use this library in commercial projects
- ✅ **Modification** - You can modify and distribute the code
- ✅ **Patent protection** - Protection from patent claims
- ✅ **Attribution** - You must include the original license and copyright notice

**Copyright (c) 2024 Emil Rokossovsiy**

For full license text, see [LICENSE](LICENSE) file.

---

## 🌟 Support & Community


### 📞 **Get Help**
[![GitHub Issues](https://img.shields.io/badge/GitHub-Issues-black?style=for-the-badge&logo=github)](https://github.com/rokoss21/bazi-lib/issues)
[![Email Support](https://img.shields.io/badge/Email-Support-red?style=for-the-badge&logo=gmail)](mailto:ecsiar@gmail.com)

### 🌐 **Connect**
[![Website](https://img.shields.io/badge/Website-astrovisor.io-purple?style=for-the-badge&logo=globe)](https://astrovisor.io/)
[![GitHub Profile](https://img.shields.io/badge/GitHub-rokoss21-black?style=for-the-badge&logo=github)](https://github.com/rokoss21/)


---


## 🌟 **Star this repository if you find it useful!**

[![GitHub Stars](https://img.shields.io/badge/GitHub-Stars-black?style=for-the-badge&logo=github)](https://github.com/rokoss21/bazi-lib)
[![GitHub Forks](https://img.shields.io/badge/GitHub-Forks-black?style=for-the-badge&logo=github)](https://github.com/rokoss21/bazi-lib)

---

**Developed with ❤️ by Emil Rokossovsiy**  
**Version**: 1.0.0  
**Python Support**: 3.7+  
**Status**: 🚀 Production Ready  
**License**: Apache 2.0

[![GitHub](https://img.shields.io/badge/GitHub-rokoss21-black?style=for-the-badge&logo=github)](https://github.com/rokoss21/)
[![Email](https://img.shields.io/badge/Email-ecsiar%40gmail.com-red?style=for-the-badge&logo=gmail)](mailto:ecsiar@gmail.com)
[![Website](https://img.shields.io/badge/Website-astrovisor.io-purple?style=for-the-badge&logo=globe)](https://astrovisor.io/)

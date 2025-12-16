use chrono::{NaiveDateTime, Datelike, Duration};

pub struct SolarTimeCalculator;

impl SolarTimeCalculator {
    /// Adjusts the local civil time to True Solar Time (TST).
    ///
    /// # Arguments
    /// * `date`: The local civil date and time.
    /// * `longitude`: The longitude of the birth place (East is positive, West is negative).
    /// * `timezone_offset_hours`: The timezone offset from UTC in hours (e.g., +8.0 for Beijing).
    ///
    /// # Returns
    /// * `NaiveDateTime`: The adjusted True Solar Time.
    pub fn get_true_solar_time(date: NaiveDateTime, longitude: f64, timezone_offset_hours: f64) -> NaiveDateTime {
        // 1. Longitude Correction (LMT)
        // Earth rotates 15 degrees per hour, or 1 degree every 4 minutes.
        // Standard meridian for timezone = offset * 15.
        // Difference = (Local Longitude - Standard Meridian).

        let standard_meridian = timezone_offset_hours * 15.0;
        let diff_degrees = longitude - standard_meridian;
        let correction_minutes = diff_degrees * 4.0;

        // 2. Equation of Time (EoT) - Correction for Earth's elliptical orbit and axial tilt.
        // Simplified approximation formula.
        // B = 360 * (n - 81) / 365
        // EoT = 9.87 * sin(2B) - 7.53 * cos(B) - 1.5 * sin(B)
        // where n is day of year.

        let day_of_year = date.ordinal() as f64;
        let b: f64 = 360.0 * (day_of_year - 81.0) / 365.0;
        let b_rad = b.to_radians();

        let eot_minutes = 9.87 * (2.0 * b_rad).sin() - 7.53 * b_rad.cos() - 1.5 * b_rad.sin();

        let total_correction_seconds = ((correction_minutes + eot_minutes) * 60.0) as i64;

        date + Duration::try_seconds(total_correction_seconds).unwrap_or(Duration::zero())
    }
}

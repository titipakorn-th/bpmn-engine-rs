//! Timer Duration Utilities
//!
//! Parses ISO 8601 duration format and calculates due dates.

use chrono::{DateTime, Duration, Utc};

/// Parse ISO 8601 duration string
///
/// # Examples
/// - "PT3D" → 3 days
/// - "PT24H" → 24 hours
/// - "PT1H30M" → 1 hour 30 minutes
/// - "R/PT1H" → 1 hour (with R/ prefix stripped)
pub fn parse_iso8601_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();
    let _is_repeating = s.starts_with("R/");
    let duration_part = s.strip_prefix("R/").unwrap_or(s);

    // Must start with P
    if !duration_part.starts_with('P') {
        return Err("Invalid ISO 8601 duration: must start with P".to_string());
    }

    let remainder = &duration_part[1..];
    let mut days = 0i64;
    let mut months = 0i64;
    let mut secs = 0i64;

    // Check for T separator
    let has_time = remainder.contains('T');

    if has_time {
        // Split at T
        let date_part = remainder.split('T').next().unwrap_or("");
        let time_part = remainder.split('T').nth(1).unwrap_or("");

        // Parse date part (e.g., "3D" → 3 days)
        // Format: PnYnMnD
        let mut current = String::new();
        for c in date_part.chars() {
            match c {
                'Y' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    months += val * 12;
                    current.clear();
                }
                'M' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    // Could be months or minutes - context determines
                    if date_part.contains('T') {
                        secs += val * 60; // Minutes before T = months, after T = minutes
                    } else {
                        months += val;
                    }
                    current.clear();
                }
                'D' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    days += val;
                    current.clear();
                }
                '0'..='9' => current.push(c),
                _ => {}
            }
        }

        // Parse time part (e.g., "1H30M" → 1 hour 30 minutes)
        let mut current = String::new();
        for c in time_part.chars() {
            match c {
                'H' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    secs += val * 3600;
                    current.clear();
                }
                'M' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    secs += val * 60;
                    current.clear();
                }
                'S' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    secs += val;
                    current.clear();
                }
                'D' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    days += val;
                    current.clear();
                }
                '0'..='9' => current.push(c),
                _ => {}
            }
        }
    } else {
        // Date only part
        let mut current = String::new();
        for c in remainder.chars() {
            match c {
                'Y' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    months += val * 12;
                    current.clear();
                }
                'M' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    months += val;
                    current.clear();
                }
                'D' => {
                    let val: i64 = current.parse().unwrap_or(0);
                    days += val;
                    current.clear();
                }
                'T' => {} // Should not happen here
                '0'..='9' => current.push(c),
                _ => {}
            }
        }
    }

    let duration = Duration::days(days) + Duration::days(months * 30) + Duration::seconds(secs);
    Ok(duration)
}

/// Calculate due date from timer definition
pub fn calculate_due_date(timer_str: &str) -> Result<DateTime<Utc>, String> {
    let duration = parse_iso8601_duration(timer_str)?;
    Ok(Utc::now() + duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_days() {
        let dur = parse_iso8601_duration("PT3D").unwrap();
        assert_eq!(dur.num_days(), 3);
    }

    #[test]
    fn test_parse_hours() {
        let dur = parse_iso8601_duration("PT24H").unwrap();
        assert_eq!(dur.num_hours(), 24);
    }

    #[test]
    fn test_parse_duration() {
        let dur = parse_iso8601_duration("PT1H30M").unwrap();
        assert_eq!(dur.num_minutes(), 90);
    }

    #[test]
    fn test_parse_repeating() {
        let dur = parse_iso8601_duration("R/PT1H").unwrap();
        assert_eq!(dur.num_hours(), 1);
    }
}
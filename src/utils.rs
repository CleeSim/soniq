//! Utility functions for common operations like timestamp parsing, and formatting.

use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, TimeZone, Utc};
use serde::de::{self, Deserialize, Deserializer};

/// Generic helper to parse numeric types from strings
pub fn from_str_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => {
            let parsed = s.parse().map_err(de::Error::custom)?;
            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}

/// Same as above, but for required fields
pub fn from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

/// Masks an API key by replacing all but the first 3 characters with `*`.
///
/// Useful for logging without exposing full keys.
pub fn mask_api_key(api_key: &str) -> String {
    api_key
        .chars()
        .enumerate()
        .map(|(i, c)| if i < 3 { c } else { '*' })
        .collect()
}

/// Returns the current UNIX timestamp in seconds (UTC).
pub fn timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_else(|_| 0)
}

/// Converts a UNIX timestamp to a `DateTime<Utc>`.
pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    // `timestamp_opt` returns a `LocalResult` which can be `None` if the timestamp
    // is out of range. We fall back to the UNIX epoch if that happens.
    // For `Utc`, the result will never be `Ambiguous`.
    Utc.timestamp_opt(timestamp, 0)
        .single()
        .unwrap_or_else(|| Utc.timestamp_opt(0, 0).single().unwrap())
}

/// Formats a `DateTime<Utc>` as an ISO 8601 string.
pub fn format_datetime_iso(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Formats a duration in seconds to a human-friendly format like "1h 23m 45s".
pub fn format_duration_human(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    let mut parts = Vec::new();
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if secs > 0 || parts.is_empty() {
        parts.push(format!("{}s", secs));
    }

    parts.join(" ")
}

/// Formats a number with commas as thousands separators.
pub fn format_number(num: i64) -> String {
    let num_str = num.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in num_str.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }

    // Reverse the result to get the correct order
    result.chars().rev().collect()
}

/// Computes the percentage of a value relative to the total.
/// Returns `None` if total is zero.
pub fn percent(value: u64, total: u64) -> Option<f64> {
    if total == 0 {
        None
    } else {
        Some((value as f64 / total as f64) * 100.0)
    }
}

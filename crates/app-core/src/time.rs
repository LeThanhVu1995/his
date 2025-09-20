// src/time.rs placeholder
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone, Utc, Datelike};

/// Thời điểm hiện tại theo UTC.
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Parse chuỗi RFC3339 bất kỳ timezone -> DateTime<Utc>.
pub fn parse_rfc3339_to_utc(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let dt = DateTime::parse_from_rfc3339(s)?; // DateTime<FixedOffset>
    Ok(dt.with_timezone(&Utc))
}

/// Bắt đầu ngày (00:00:00) theo timezone `tz`, trả về UTC.
pub fn start_of_day_utc(date: NaiveDate, tz: FixedOffset) -> DateTime<Utc> {
    tz.from_local_datetime(&NaiveDateTime::new(date, chrono::NaiveTime::MIN))
        .single()
        .unwrap_or_else(|| tz.with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0).unwrap())
        .with_timezone(&Utc)
}

/// Kết thúc ngày (23:59:59.999_999_999) theo timezone `tz`, trả về UTC.
pub fn end_of_day_utc(date: NaiveDate, tz: FixedOffset) -> DateTime<Utc> {
    // Sử dụng cuối ngày = start_of_next_day - 1 nanosecond
    let next = date.succ_opt().unwrap_or(date);
    let next_start = tz
        .from_local_datetime(&NaiveDateTime::new(next, chrono::NaiveTime::MIN))
        .single()
        .unwrap_or_else(|| tz.with_ymd_and_hms(next.year(), next.month(), next.day(), 0, 0, 0).unwrap());
    (next_start - chrono::Duration::nanoseconds(1)).with_timezone(&Utc)
}

// /* -------------------- Tests -------------------- */
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use pretty_assertions::assert!(matches);

//     #[test]
//     fn parse_rfc3339_ok() {
//         let dt = parse_rfc3339_to_utc("2024-03-01T12:34:56+07:00").unwrap();
//         assert_eq!(dt.time().hour(), 5); // 12:34:56+07 -> 05:34:56Z
//     }

//     #[test]
//     fn sod_eod_vn_tz() {
//         use chrono::{NaiveDate, FixedOffset, Timelike};
//         let tz = FixedOffset::east_opt(7 * 3600).unwrap();
//         let date = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();
//         let sod = start_of_day_utc(date, tz);
//         let eod = end_of_day_utc(date, tz);
//         // SOD 00:00 VN -> 17:00Z ngày trước
//         assert_eq!(sod.hour(), 17);
//         assert!(eod > sod);
//     }
// }

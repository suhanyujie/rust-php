extern crate chrono;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::{DateTime};
use self::chrono::{Utc, FixedOffset};
use self::chrono::offset::TimeZone;
use chrono::prelude::Local;
use std::collections::HashMap;

// 关于时间的库，可以参考 https://github.com/chronotope/chrono/

/// 获取微秒级的时间戳
/// 方法实现参考 https://users.rust-lang.org/t/convert-std-time-systemtime-to-chrono-datetime-datetime/7684
pub fn microtime() -> u128 {
    let now = SystemTime::now();
    let micro_seconds = now.duration_since(UNIX_EPOCH).unwrap().as_micros();
    return micro_seconds;
}

/// 获取当前时间的秒级时间戳
pub fn time() -> u128 {
    // SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u128
    let datetime: DateTime<Local> = Local::now();
    let local_time = DateTime::<Utc>::from_utc(datetime.naive_utc(), Utc);
    // 转换为 Asia/ShangHai 时区
    let china_timezone = FixedOffset::east(8*3600);
    local_time.with_timezone(&china_timezone);
    local_time.timestamp() as u128
}

/// 根据时间戳将其格式化为当前的"标准日期时间"格式
pub fn date(format_str: &str, mut timestamp: u128) -> String {
    if timestamp <= 1 {
        timestamp = time();
    }
    let mut flag_map: HashMap<&str, &str> = HashMap::new();
    flag_map.insert(&"Y", "%Y");
    flag_map.insert(&"m", "%m");
    flag_map.insert(&"d", "%d");
    flag_map.insert(&"H", "%H");
    flag_map.insert(&"h", "%h");
    flag_map.insert(&"i", "%M");
    flag_map.insert(&"s", "%S");
    let mut format_str_res = format_str.to_string();
    flag_map.iter().for_each(|(key, value)|{
        format_str_res = format_str_res.replace(key, value);
    });
    Utc.timestamp(timestamp as i64, 0).format(&format_str_res).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn test_microtime() {
        let t1 = microtime();
        // todo something
        sleep(Duration::new(2, 0));
        let t2 = microtime();
        let diff = (t2 - t1) / 1000 / 1000;
        println!("{}", diff);
        assert!(diff == 2);
    }

    #[test]
    fn test_time() {
        let t1 = time();
        println!("current timestamp is: {}", t1);
        assert!(t1 == time());
    }

    #[test]
    fn test_date() {
        let t1 = date(&"Y-m-d H:i:s", 1579500612);
        assert!(t1 == "2020-01-20 06:10:12".to_string());
    }
}

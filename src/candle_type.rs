use rust_extensions::date_time::*;

const MINUTE_KEY: &'static str = "1m";
const MINUTE_5_KEY: &'static str = "5m";
const HOUR_KEY: &'static str = "1h";
const DAY_KEY: &'static str = "1d";
const MONTH_KEY: &'static str = "1M";

#[derive(Debug, Clone, Copy)]
pub enum CandleType {
    Minute,
    Min5,
    Hour,
    Day,
    Month,
}
impl CandleType {
    pub const ALL: [CandleType; 5] = [
        CandleType::Minute,
        CandleType::Min5,
        CandleType::Hour,
        CandleType::Day,
        CandleType::Month,
    ];
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Minute => MINUTE_KEY,
            Self::Min5 => MINUTE_5_KEY,
            Self::Hour => HOUR_KEY,
            Self::Day => DAY_KEY,
            Self::Month => MONTH_KEY,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            MINUTE_KEY => Self::Minute,
            MINUTE_5_KEY => Self::Min5,
            HOUR_KEY => Self::Hour,
            DAY_KEY => Self::Day,
            MONTH_KEY => Self::Month,
            _ => {
                panic!("Invalid candle interval key: {}", src);
            }
        }
    }

    pub fn to_interval_key(&self, now: DateTimeAsMicroseconds) -> i64 {
        match self {
            Self::Minute => {
                let interval_key: IntervalKey<MinuteKey> = now.into();
                interval_key.to_i64()
            }
            Self::Hour => {
                let interval_key: IntervalKey<HourKey> = now.into();
                interval_key.to_i64()
            }
            Self::Day => {
                let interval_key: IntervalKey<DayKey> = now.into();
                interval_key.to_i64()
            }
            Self::Month => {
                let interval_key: IntervalKey<MonthKey> = now.into();
                interval_key.to_i64()
            }
            Self::Min5 => {
                let interval_key: IntervalKey<Minute5Key> = now.into();
                interval_key.to_i64()
            }
        }
    }
}

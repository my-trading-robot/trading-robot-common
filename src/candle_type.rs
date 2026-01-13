const MINUTE_KEY: &'static str = "1m";
const MINUTE_5_KEY: &'static str = "5m";
const HOUR_KEY: &'static str = "1h";
const DAY_KEY: &'static str = "1d";
const MONTH_KEY: &'static str = "1M";

#[derive(Clone, Copy)]
pub enum CandleType {
    Minute,
    Minute5,
    Hour,
    Day,
    Month,
}
impl CandleType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Minute => MINUTE_KEY,
            Self::Minute5 => MINUTE_5_KEY,
            Self::Hour => HOUR_KEY,
            Self::Day => DAY_KEY,
            Self::Month => MONTH_KEY,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            MINUTE_KEY => Self::Minute,
            MINUTE_5_KEY => Self::Minute5,
            HOUR_KEY => Self::Hour,
            DAY_KEY => Self::Day,
            MONTH_KEY => Self::Month,
            _ => {
                panic!("Invalid candle interval key: {}", src);
            }
        }
    }
}

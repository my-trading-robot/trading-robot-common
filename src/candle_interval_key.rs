use rust_extensions::date_time::*;

use crate::CandleType;

pub struct CandleIntervalKey {
    value: u64,
    candle_type: CandleType,
}

impl CandleIntervalKey {
    pub fn new(date: DateTimeAsMicroseconds, candle_type: CandleType) -> Self {
        let value = match candle_type {
            CandleType::Minute => {
                let key: IntervalKey<MinuteKey> = date.try_into().unwrap();
                key.to_i64() as u64
            }
            CandleType::Hour => {
                let key: IntervalKey<HourKey> = date.try_into().unwrap();
                key.to_i64() as u64
            }
            CandleType::Day => {
                let key: IntervalKey<DayKey> = date.try_into().unwrap();
                key.to_i64() as u64
            }
            CandleType::Month => {
                let key: IntervalKey<MonthKey> = date.try_into().unwrap();
                key.to_i64() as u64
            }

            CandleType::Minute5 => {
                let key: IntervalKey<Minute5Key> = date.try_into().unwrap();
                key.to_i64() as u64
            }
        };
        Self { value, candle_type }
    }

    pub fn to_u64(&self) -> u64 {
        self.value
    }

    pub fn as_u64_ref(&self) -> &u64 {
        &self.value
    }

    pub fn get_candle_type(&self) -> CandleType {
        self.candle_type
    }
}

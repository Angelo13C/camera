use a13c_embedded::{
	features::storage::embedded_sdmmc::{TimeSource as TimeSourceTrait, Timestamp},
	hardware::espressif::peripherals::real_time::RealTime,
	peripherals::time::real_time::RealTimeClock,
};
use esp_idf_svc::wifi::*;

pub struct TimeSource(pub RealTime<EspWifi<'static>>);

impl TimeSourceTrait for TimeSource
{
	fn get_timestamp(&self) -> Timestamp
	{
		let (date, time) = self.0.now().unwrap();
		Timestamp {
			year_since_1970: (date.year() - 1970) as u8,
			zero_indexed_month: date.month() as u8 - 1,
			zero_indexed_day: date.day() - 1,
			hours: time.hour(),
			minutes: time.minute(),
			seconds: time.second(),
		}
	}
}

pub mod camera;

use core::fmt::Debug;
use std::net::IpAddr;

extern crate alloc;
use alloc::boxed::Box;

use a13c_embedded::{
	features::{communication::http::server::HttpServer, storage::embedded_sdmmc::TimeSource},
	peripherals::{
		time::{real_time::RealTimeClock, system_time::SystemTime},
		watchdog::WatchdogCreator,
	},
};
use embedded_hal::{
	delay::DelayNs,
	digital::{InputPin, OutputPin},
	spi::SpiDevice,
};
use embedded_svc::wifi::Wifi;

use self::camera::Camera;
use crate::features::http_server::{stream::PossibleHttpRequest as StreamPossibleHttpRequest, PossibleHttpRequest};

pub trait Peripherals
{
	type Camera: Camera;

	type WifiDriver: Wifi;
	type Server: HttpServer<HttpRequest = PossibleHttpRequest>;
	type StreamServer: HttpServer<HttpRequest = StreamPossibleHttpRequest>;
	type ServerError: Debug;

	type SdCardSpi: SpiDevice;
	type SdCardCS: OutputPin;
	type SdCardDelay: DelayNs;
	type SdCardTimeSource: TimeSource;

	type PirSensorPin: InputPin;

	type WatchdogCreator: WatchdogCreator;

	type RealTimeClock: RealTimeClock;

	fn take_camera(&mut self) -> Option<Self::Camera>;

	fn take_wifi_driver(&mut self) -> Option<Self::WifiDriver>;
	fn get_ip_address_from_wifi_driver_function() -> fn(&Self::WifiDriver) -> Option<IpAddr>;
	fn take_http_server(&mut self) -> Option<Box<dyn FnOnce() -> Result<Self::Server, Self::ServerError>>>;
	fn take_stream_http_server(&mut self)
		-> Option<Box<dyn FnOnce() -> Result<Self::StreamServer, Self::ServerError>>>;

	fn take_sd_card_spi(&mut self) -> Option<Self::SdCardSpi>;
	fn take_sd_card_cs(&mut self) -> Option<Self::SdCardCS>;
	fn take_sd_card_delay(&mut self) -> Option<Self::SdCardDelay>;
	fn take_sd_card_time_source(&mut self) -> Option<Self::SdCardTimeSource>;

	fn take_pir_sensor_pin(&mut self) -> Option<Self::PirSensorPin>;

	fn take_watchdog_creator(&mut self) -> Option<Self::WatchdogCreator>;

	fn take_real_time_clock(&mut self) -> Option<Self::RealTimeClock>;
}

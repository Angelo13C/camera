use core::time::Duration;
use std::net::IpAddr;

use a13c_embedded::{
	hardware::espressif::{
		features::communication::http_server::HttpServer,
		peripherals::{delay::Delay, real_time::RealTime, watchdog::WatchdogCreator},
	},
	peripherals::time::real_time::time::UtcOffset,
	utils::physical_quantities::frequency::Frequency,
};
use enumset::EnumSet;
use esp_idf_hal::{
	gpio::*,
	i2c::I2cDriver,
	io::EspIOError,
	ledc::{LedcChannel, LedcTimer},
	spi::{
		config::{BitOrder, Config, DriverConfig, Duplex},
		Dma, SpiSingleDeviceDriver,
	},
	sys::EspError,
	task::watchdog::*,
	units::Hertz,
};
#[cfg(esp_idf_esp_https_server_enable)]
use esp_idf_svc::tls::X509;
use esp_idf_svc::{
	eventloop::EspSystemEventLoop,
	http::server::{Configuration, EspHttpServer},
	nvs::EspDefaultNvsPartition,
	sntp::*,
	wifi::{Configuration as WifiConfiguration, *},
};
use firmware_core::{
	configuration::peripherals::Peripherals as PeripheralsTrait,
	features::http_server::{stream::PossibleHttpRequest as StreamPossibleHttpRequest, PossibleHttpRequest},
};

use crate::{
	esp32_camera::{Camera, CameraGrabMode, FrameBufferLocation, FrameSize, PixelFormat},
	time_source::TimeSource,
};

pub const HTTP_SERVER_CONFIG: Configuration = Configuration {
	http_port: 80,
	ctrl_port: 32768,
	https_port: 443,
	core_id: 1,
	max_sessions: 2,
	session_timeout: Duration::from_secs(20 * 60),
	#[cfg(not(esp_idf_esp_https_server_enable))]
	stack_size: 6144 + firmware_core::features::http_server::STACK_SIZE,
	#[cfg(esp_idf_esp_https_server_enable)]
	stack_size: 10240 + firmware_core::features::http_server::STACK_SIZE,
	max_open_sockets: 2,
	max_uri_handlers: firmware_core::features::http_server::http_request_handlers_count(),
	max_resp_headers: 8,
	lru_purge_enable: false,
	uri_match_wildcard: false,
	#[cfg(esp_idf_esp_https_server_enable)]
	server_certificate: X509::der(include_bytes!("../../cert.pem")),
	#[cfg(esp_idf_esp_https_server_enable)]
	private_key: X509::der(include_bytes!("../../key.pem")),
};

pub const STREAM_HTTP_SERVER_CONFIG: Configuration = Configuration {
	http_port: 81,
	ctrl_port: 32769,
	https_port: 60000,
	core_id: 1,
	max_sessions: 1,
	session_timeout: Duration::from_secs(20 * 60),
	#[cfg(not(esp_idf_esp_https_server_enable))]
	stack_size: 6144 + firmware_core::features::http_server::STACK_SIZE,
	#[cfg(esp_idf_esp_https_server_enable)]
	stack_size: 10240 + firmware_core::features::http_server::STACK_SIZE,
	max_open_sockets: 2,
	max_uri_handlers: firmware_core::features::http_server::stream::http_request_handlers_count(),
	max_resp_headers: 8,
	lru_purge_enable: false,
	uri_match_wildcard: false,
	#[cfg(esp_idf_esp_https_server_enable)]
	server_certificate: X509::der(include_bytes!("../../cert.pem")),
	#[cfg(esp_idf_esp_https_server_enable)]
	private_key: X509::der(include_bytes!("../../key.pem")),
};

impl PeripheralsTrait for Peripherals
{
	type Camera = Camera<'static>;

	type WifiDriver = EspWifi<'static>;
	type Server = HttpServer<'static, PossibleHttpRequest>;
	type StreamServer = HttpServer<'static, StreamPossibleHttpRequest>;
	type ServerError = EspIOError;

	type SdCardSpi = SpiSingleDeviceDriver<'static>;
	type SdCardCS = PinDriver<'static, Gpio13, Output>;
	type SdCardDelay = Delay;
	type SdCardTimeSource = TimeSource;

	type PirSensorPin = a13c_embedded::hardware::mock::MockInputPin; //PinDriver<'static, Gpio16, Input>;

	type WatchdogCreator = WatchdogCreator;

	type RealTimeClock = RealTime<Self::WifiDriver>;

	fn take_camera(&mut self) -> Option<Self::Camera>
	{
		self.camera.take()
	}

	fn take_wifi_driver(&mut self) -> Option<Self::WifiDriver>
	{
		self.wifi_driver.take()
	}

	fn get_ip_address_from_wifi_driver_function() -> fn(&Self::WifiDriver) -> Option<IpAddr>
	{
		|wifi_driver| {
			wifi_driver
				.sta_netif()
				.get_ip_info()
				.ok()
				.map(|info| IpAddr::V4(info.ip))
		}
	}

	fn take_http_server(&mut self) -> Option<Box<dyn FnOnce() -> Result<Self::Server, Self::ServerError>>>
	{
		self.http_server.take()
	}

	fn take_stream_http_server(&mut self)
		-> Option<Box<dyn FnOnce() -> Result<Self::StreamServer, Self::ServerError>>>
	{
		self.stream_http_server.take()
	}

	fn take_sd_card_spi(&mut self) -> Option<Self::SdCardSpi>
	{
		self.sd_card_spi.take()
	}

	fn take_sd_card_cs(&mut self) -> Option<Self::SdCardCS>
	{
		self.sd_card_cs.take()
	}

	fn take_sd_card_delay(&mut self) -> Option<Self::SdCardDelay>
	{
		self.sd_card_delay.take()
	}

	fn take_sd_card_time_source(&mut self) -> Option<Self::SdCardTimeSource>
	{
		self.sd_card_time_source.take()
	}

	fn take_pir_sensor_pin(&mut self) -> Option<Self::PirSensorPin>
	{
		self.pir_sensor_pin.take()
	}

	fn take_watchdog_creator(&mut self) -> Option<Self::WatchdogCreator>
	{
		Some(self.watchdog_creator.clone())
	}

	fn take_real_time_clock(&mut self) -> Option<Self::RealTimeClock>
	{
		self.real_time_clock.take()
	}
}

pub const SD_CARD_SPI_DRIVER_CONFIG: DriverConfig = DriverConfig {
	dma: Dma::Auto(150_000),
	intr_flags: EnumSet::EMPTY,
};

pub const SD_CARD_SPI_CONFIG: Config = Config {
	baudrate: Hertz(20_000_000),
	data_mode: esp_idf_hal::spi::config::MODE_0,
	write_only: false,
	duplex: Duplex::Full,
	bit_order: BitOrder::MsbFirst,
	cs_active_high: false,
	input_delay_ns: 1,
	polling: true,
	allow_pre_post_delays: false,
	queue_size: 1,
};

pub struct Peripherals
{
	camera: Option<<Self as PeripheralsTrait>::Camera>,
	wifi_driver: Option<<Self as PeripheralsTrait>::WifiDriver>,
	http_server: Option<
		Box<dyn FnOnce() -> Result<<Self as PeripheralsTrait>::Server, <Self as PeripheralsTrait>::ServerError>>,
	>,
	stream_http_server: Option<
		Box<dyn FnOnce() -> Result<<Self as PeripheralsTrait>::StreamServer, <Self as PeripheralsTrait>::ServerError>>,
	>,
	sd_card_spi: Option<<Self as PeripheralsTrait>::SdCardSpi>,
	sd_card_cs: Option<<Self as PeripheralsTrait>::SdCardCS>,
	sd_card_delay: Option<<Self as PeripheralsTrait>::SdCardDelay>,
	sd_card_time_source: Option<<Self as PeripheralsTrait>::SdCardTimeSource>,
	pir_sensor_pin: Option<<Self as PeripheralsTrait>::PirSensorPin>,
	watchdog_creator: <Self as PeripheralsTrait>::WatchdogCreator,
	real_time_clock: Option<<Self as PeripheralsTrait>::RealTimeClock>,
}

impl Peripherals
{
	pub fn from_esp_peripherals(peripherals: esp_idf_hal::peripherals::Peripherals) -> Result<Self, EspError>
	{
		let sys_loop = EspSystemEventLoop::take()?;
		let nvs = EspDefaultNvsPartition::take()?;

		let mut wifi_driver = EspWifi::wrap(WifiDriver::new(peripherals.modem, sys_loop, Some(nvs))?)?;
		wifi_driver.set_configuration(&WifiConfiguration::Client(ClientConfiguration {
			ssid: env!("WIFI_SSID").try_into().unwrap(),
			bssid: None,
			auth_method: AuthMethod::WPA2Personal,
			password: env!("WIFI_PASSWORD").try_into().unwrap(),
			channel: None,
		}))?;
		wifi_driver.start()?;
		wifi_driver.connect()?;

		let i2c = Box::leak(Box::new(I2cDriver::new(
			peripherals.i2c0,
			peripherals.pins.gpio26,
			peripherals.pins.gpio27,
			&Camera::I2C_CONFIGURATION,
		)?));

		let utc_offset = UtcOffset::from_hms(2, 0, 0).unwrap();

		Ok(Self {
			camera: Some(Camera::new(
				peripherals.pins.gpio32,
				None as Option<esp_idf_hal::gpio::AnyIOPin>,
				peripherals.pins.gpio0,
				peripherals.pins.gpio5,
				peripherals.pins.gpio18,
				peripherals.pins.gpio19,
				peripherals.pins.gpio21,
				peripherals.pins.gpio36,
				peripherals.pins.gpio39,
				peripherals.pins.gpio34,
				peripherals.pins.gpio35,
				peripherals.pins.gpio25,
				peripherals.pins.gpio23,
				peripherals.pins.gpio22,
				&i2c,
				esp_idf_hal::ledc::TIMER0::timer(),
				esp_idf_hal::ledc::CHANNEL0::channel(),
				Frequency::from_megahertz(10),
				PixelFormat::JPEG,
				FrameSize::SVGA,
				CameraGrabMode::WhenEmpty,
				FrameBufferLocation::PSRAM,
			)?),
			wifi_driver: Some(wifi_driver),
			http_server: Some(Box::new(move || {
				Ok(HttpServer::new(EspHttpServer::new(&HTTP_SERVER_CONFIG)?))
			})),
			stream_http_server: Some(Box::new(move || {
				Ok(HttpServer::new(EspHttpServer::new(&STREAM_HTTP_SERVER_CONFIG)?))
			})),
			sd_card_spi: Some(SpiSingleDeviceDriver::new_single(
				peripherals.spi2,
				peripherals.pins.gpio14,
				peripherals.pins.gpio15,
				Some(peripherals.pins.gpio2),
				None as Option<AnyOutputPin>,
				&SD_CARD_SPI_DRIVER_CONFIG,
				&SD_CARD_SPI_CONFIG,
			)?),
			sd_card_cs: Some(PinDriver::output(peripherals.pins.gpio13)?),
			sd_card_delay: Some(Delay),
			sd_card_time_source: Some(TimeSource(RealTime::new(
				utc_offset.clone(),
				None,
				Some(EspSntp::new(&SntpConf { ..Default::default() })?),
			))),
			pir_sensor_pin: Some(a13c_embedded::hardware::mock::MockInputPin::Ok { is_high: true }), // PinDriver::input(peripherals.pins.gpio16)?),
			watchdog_creator: WatchdogCreator(TWDTDriver::new(
				peripherals.twdt,
				&TWDTConfig {
					duration: Duration::from_secs(5),
					panic_on_trigger: true,
					subscribed_idle_tasks: EnumSet::EMPTY,
				},
			)?),
			real_time_clock: Some(RealTime::new(utc_offset, None, None)),
		})
	}
}

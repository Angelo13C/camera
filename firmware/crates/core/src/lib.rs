pub mod configuration;
pub mod errors;
pub mod features;

use core::time::Duration;

use a13c_embedded::peripherals::{time::real_time::RealTimeClock, watchdog::*};
use configuration::{
	customization::Customization,
	peripherals::{
		camera::{Camera as CameraTrait, Image},
		Peripherals,
	},
	Configuration,
};
use errors::*;
use features::{
	http_server::{register_all_requests, HttpServerData},
	storage::Storage,
	trigger::ImageTrigger,
};

pub struct Camera<C: Configuration>
{
	camera: <C::Peripherals as Peripherals>::Camera,
	http_server: <C::Peripherals as Peripherals>::Server,
	stream_http_server: <C::Peripherals as Peripherals>::StreamServer,
	wifi_driver: <C::Peripherals as Peripherals>::WifiDriver,
	get_ip_address_from_wifi_driver_fn:
		fn(&<<C as Configuration>::Peripherals as Peripherals>::WifiDriver) -> Option<std::net::IpAddr>,
	storage: Storage<
		<C::Peripherals as Peripherals>::SdCardSpi,
		<C::Peripherals as Peripherals>::SdCardCS,
		<C::Peripherals as Peripherals>::SdCardDelay,
		<C::Peripherals as Peripherals>::SdCardTimeSource,
		3,
		3,
		1,
	>,
	watchdog: Option<<<C::Peripherals as Peripherals>::WatchdogCreator as WatchdogCreator>::Watchdog>,
	test_counter: u32,
	http_server_data: HttpServerData,
	image_trigger: ImageTrigger<
		<C::Peripherals as Peripherals>::PirSensorPin,
		<C::Customization as Customization>::EnableOnConditionsList,
	>,
	real_time_clock: <C::Peripherals as Peripherals>::RealTimeClock,
}

impl<C: Configuration> Camera<C>
{
	pub fn new(mut configuration: C) -> Result<Self, CreationError<C>>
	{
		let mut peripherals = configuration.peripherals();
		let customization = configuration.customization();

		let mut http_server = (peripherals
			.take_http_server()
			.ok_or(CreationError::PeripheralMissing { name: "HTTP server" })?)()
		.map_err(CreationError::StartHttpServer)?;
		let mut stream_http_server =
			(peripherals
				.take_stream_http_server()
				.ok_or(CreationError::PeripheralMissing {
					name: "Stream HTTP server",
				})?)()
			.map_err(CreationError::StartStreamHttpServer)?;
		let http_server_data = HttpServerData::new();
		register_all_requests(&mut http_server, &mut stream_http_server, http_server_data.clone())
			.map_err(CreationError::RegisterURIHandlerHttpServer)?;

		Ok(Self {
			test_counter: 0,
			camera: peripherals
				.take_camera()
				.ok_or(CreationError::PeripheralMissing { name: "Camera" })?,
			http_server,
			stream_http_server,
			wifi_driver: peripherals
				.take_wifi_driver()
				.ok_or(CreationError::PeripheralMissing { name: "WiFi driver" })?,
			get_ip_address_from_wifi_driver_fn: C::Peripherals::get_ip_address_from_wifi_driver_function(),
			storage: Storage::new(
				peripherals
					.take_sd_card_spi()
					.ok_or(CreationError::PeripheralMissing { name: "SD Card SPI" })?,
				peripherals
					.take_sd_card_cs()
					.ok_or(CreationError::PeripheralMissing { name: "SD Card CS" })?,
				peripherals
					.take_sd_card_delay()
					.ok_or(CreationError::PeripheralMissing { name: "SD Card delay" })?,
				peripherals
					.take_sd_card_time_source()
					.ok_or(CreationError::PeripheralMissing {
						name: "SD Card time source",
					})?,
			)
			.map_err(CreationError::SdCard)?,
			watchdog: peripherals
				.take_watchdog_creator()
				.map(|watchdog_creator| watchdog_creator.watch_current_thread())
				.flatten(),
			image_trigger: ImageTrigger::new(
				peripherals
					.take_pir_sensor_pin()
					.ok_or(CreationError::PeripheralMissing { name: "PIR sensor pin" })?,
				customization.enable_image_trigger_on(),
				customization.trigger_duration(),
			),
			http_server_data,
			real_time_clock: peripherals
				.take_real_time_clock()
				.ok_or(CreationError::<C>::PeripheralMissing {
					name: "Real time clock",
				})?,
		})
	}

	pub fn tick(&mut self) -> Result<(), TickError<C>>
	{
		if let Some(watchdog) = self.watchdog.as_mut()
		{
			watchdog.feed().map_err(TickError::WatchdogReset)?;
		}

		if let Ok(current_date_and_time) = self.real_time_clock.now()
		{
			self.image_trigger
				.tick(Some(current_date_and_time))
				.map_err(TickError::CouldntReadPirSensorPin)?;

			if self.image_trigger.needs_to_capture_image()
			{
				let image = self.camera.get_image().map_err(TickError::Camera)?;
				self.http_server_data
					.write_image(image.get_pixels(), image.get_timestamp());

				if self.image_trigger.needs_to_store_image()
				{
					self.test_counter += 1;
					self.storage
						.store_image(image.get_pixels(), &format!("img_{}.jpg", self.test_counter))
						.unwrap();
					log::info!("Stored image: img_{}.jpg", self.test_counter);
				}
				self.http_server_data.write_image(&[], Duration::default());
			}
		}

		Ok(())
	}
}

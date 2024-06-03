mod configuration;
mod esp32_camera;
mod time_source;

use configuration::{Configuration, Peripherals};
use esp_idf_hal::peripherals::Peripherals as EspPeripherals;
use esp_idf_sys::{self as _, EspError};
use firmware_core::Camera; // If using the `binstart` feature of `esp-idf-sys`, always keep this module importeduse peripherals::Peripherals;

fn main()
{
	// It is necessary to call this function once. Otherwise some patches to the runtime
	// implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
	esp_idf_sys::link_patches();
	// Bind the log crate to the ESP Logging facilities
	esp_idf_svc::log::EspLogger::initialize_default();

	esp_idf_sys::esp!(unsafe { esp_idf_sys::esp_netif_init() }).unwrap();

	let mut camera = create_camera().unwrap();
	loop
	{
		if let Err(error) = camera.tick()
		{
			match error
			{
				firmware_core::errors::TickError::Camera(_) => (),
				_ => panic!(""),
			}
		}
	}
}

fn create_camera() -> Result<Camera<Configuration>, CreateCameraError>
{
	let peripherals =
		Peripherals::from_esp_peripherals(EspPeripherals::take().map_err(CreateCameraError::CantTakeEspPeripherals)?)
			.map_err(CreateCameraError::CantCreatePeripherals)?;
	Camera::new(Configuration::new(peripherals)).map_err(CreateCameraError::CameraCreation)
}

#[derive(Debug)]
enum CreateCameraError
{
	CantTakeEspPeripherals(EspError),
	CantCreatePeripherals(EspError),
	CameraCreation(firmware_core::errors::CreationError<Configuration>),
}

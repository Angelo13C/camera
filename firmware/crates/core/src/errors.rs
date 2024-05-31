use a13c_embedded::{
	features::{communication::http::server::HttpServer, storage::embedded_sdmmc::SdCardError},
	peripherals::watchdog::{Watchdog, WatchdogCreator},
};
use embedded_hal::digital::{ErrorType, InputPin};

use crate::{
	configuration::{
		customization::Customization,
		peripherals::{camera::Camera, Peripherals},
		Configuration,
	},
	features::http_server::RegisterError,
};

/// An error that can occur when you instatiate a [`AirMonitor`] struct.
pub enum CreationError<C: Configuration>
{
	/// A peripheral from the provided ones is missing (`name` is the name of the peripheral that's missing).
	/// This means that `peripherals.take_...()` returned `None` instead of `Some`.
	PeripheralMissing
	{
		name: &'static str,
	},

	StartHttpServer(<C::Peripherals as Peripherals>::ServerError),
	StartStreamHttpServer(<C::Peripherals as Peripherals>::ServerError),
	RegisterURIHandlerHttpServer(
		RegisterError<
			<<C::Peripherals as Peripherals>::Server as HttpServer>::Error,
			<<C::Peripherals as Peripherals>::StreamServer as HttpServer>::Error,
		>,
	),
	SdCard(a13c_embedded::features::storage::embedded_sdmmc::Error<SdCardError>),
}

impl<C: Configuration> core::fmt::Debug for CreationError<C>
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		match self
		{
			Self::PeripheralMissing { name } => f.debug_struct("PeripheralMissing").field("name", name).finish(),
			Self::StartHttpServer(error) => f.debug_tuple("Start HTTP server").field(error).finish(),
			Self::StartStreamHttpServer(error) => f.debug_tuple("Start stream HTTP server").field(error).finish(),
			Self::SdCard(error) => f.debug_tuple("SD Card").field(error).finish(),
			Self::RegisterURIHandlerHttpServer(error) =>
			{
				f.debug_tuple("Register URI handler HTTP server").field(error).finish()
			},
		}
	}
}

pub enum TickError<C: Configuration>
{
	Camera(<<C::Peripherals as Peripherals>::Camera as Camera>::Error),
	CouldntReadPirSensorPin(<<C::Peripherals as Peripherals>::PirSensorPin as ErrorType>::Error),
	WatchdogReset(<<<C::Peripherals as Peripherals>::WatchdogCreator as WatchdogCreator>::Watchdog as Watchdog>::Error),
}

impl<C: Configuration> core::fmt::Debug for TickError<C>
{
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
	{
		match self
		{
			Self::Camera(arg0) => f.debug_tuple("Camera").field(arg0).finish(),
			Self::CouldntReadPirSensorPin(arg0) => f.debug_tuple("CouldntReadPirSensorPin").field(arg0).finish(),
			Self::WatchdogReset(arg0) => f.debug_tuple("WatchdogReset").field(arg0).finish(),
		}
	}
}

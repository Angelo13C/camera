mod peripherals;

use firmware_core::configuration::Configuration as ConfigurationTrait;

use self::customization::Customization;

mod customization;

pub use peripherals::Peripherals;

pub struct Configuration(Option<Peripherals>);

impl Configuration
{
	pub fn new(peripherals: Peripherals) -> Self
	{
		Self(Some(peripherals))
	}
}

impl ConfigurationTrait for Configuration
{
	type Peripherals = Peripherals;
	type Customization = Customization;

	fn peripherals(&mut self) -> Self::Peripherals
	{
		self.0.take().unwrap()
	}

	fn customization(&mut self) -> Self::Customization
	{
		Customization
	}
}

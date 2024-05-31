use self::{customization::Customization, peripherals::Peripherals};

pub mod customization;
pub mod peripherals;

pub trait Configuration
{
	type Peripherals: Peripherals;
	type Customization: Customization;

	fn peripherals(&mut self) -> Self::Peripherals;
	fn customization(&mut self) -> Self::Customization;
}

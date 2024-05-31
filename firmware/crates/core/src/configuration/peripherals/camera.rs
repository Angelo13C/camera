use core::{fmt::Debug, time::Duration};

use a13c_embedded::utils::math::micromath::micromath::vector::U16x2;

pub trait Camera
{
	type Image<'a>: Image
	where Self: 'a;
	type Error: Debug;

	fn get_image<'a>(&'a self) -> Result<Self::Image<'a>, Self::Error>;
}

pub trait Image
{
	fn get_pixels(&self) -> &[u8];
	fn get_size(&self) -> U16x2;
	fn get_timestamp(&self) -> Duration;
}

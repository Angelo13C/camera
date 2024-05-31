use core::time::Duration;
use std::sync::Arc;

use spin::Mutex;

use crate::configuration::peripherals::camera::Image;

pub struct HttpServerData
{
	image: Arc<Mutex<(*const u8, usize, Duration)>>,
}

impl Clone for HttpServerData
{
	fn clone(&self) -> Self
	{
		Self {
			image: Arc::clone(&self.image),
		}
	}
}

impl HttpServerData
{
	pub fn new() -> Self
	{
		Self {
			image: Arc::new(Mutex::new((core::ptr::null(), 0, Duration::default()))),
		}
	}

	pub fn read_image_bytes<T>(&mut self, callback: impl FnOnce(Option<(&[u8], Duration)>) -> T) -> T
	{
		let mut image_guard = self.image.lock();
		if image_guard.0.is_null() || image_guard.1 == 0
		{
			(callback)(None)
		}
		else
		{
			let image_bytes = unsafe { core::slice::from_raw_parts(image_guard.0, image_guard.1) };
			let return_value = (callback)(Some((image_bytes, image_guard.2)));
			image_guard.0 = core::ptr::null();
			image_guard.1 = 0;
			return_value
		}
	}

	pub fn write_image(&mut self, image: &[u8], timestamp: Duration)
	{
		let mut image_guard = self.image.lock();
		*image_guard = (image as *const [u8] as *const u8, image.len(), timestamp);
	}
}

unsafe impl Send for HttpServerData {}

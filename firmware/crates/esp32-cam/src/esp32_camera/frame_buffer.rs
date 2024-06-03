#![allow(dead_code)]

use core::time::Duration;

use esp_idf_sys::camera;

use super::*;

pub struct FrameBuffer<'a>
{
	pub(super) fb: *mut camera::camera_fb_t,
	pub(super) _p: PhantomData<&'a camera::camera_fb_t>,
}

impl<'a> FrameBuffer<'a>
{
	pub fn data(&self) -> &'a [u8]
	{
		unsafe { core::slice::from_raw_parts((*self.fb).buf, (*self.fb).len) }
	}

	pub fn width(&self) -> usize
	{
		unsafe { (*self.fb).width }
	}

	pub fn height(&self) -> usize
	{
		unsafe { (*self.fb).height }
	}

	pub fn format(&self) -> PixelFormat
	{
		PixelFormat::from(unsafe { (*self.fb).format })
	}

	pub fn timestamp(&self) -> camera::timeval
	{
		unsafe { (*self.fb).timestamp }
	}
}

impl<'a> Drop for FrameBuffer<'a>
{
	fn drop(&mut self)
	{
		unsafe { camera::esp_camera_fb_return(self.fb) }
	}
}

impl<'a> Image for FrameBuffer<'a>
{
	fn get_pixels(&self) -> &[u8]
	{
		&self.data()
	}

	fn get_size(&self) -> U16x2
	{
		U16x2 {
			x: self.width() as u16,
			y: self.height() as u16,
		}
	}

	fn get_timestamp(&self) -> Duration
	{
		let timestamp = self.timestamp();
		Duration::new(timestamp.tv_sec as u64, timestamp.tv_usec as u32 * 1_000)
	}
}

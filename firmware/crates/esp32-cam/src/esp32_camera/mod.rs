#![allow(dead_code)]

mod frame_buffer;
mod sensor;
mod settings;

use std::marker::PhantomData;

use a13c_embedded::utils::{math::micromath::micromath::vector::U16x2, physical_quantities::frequency::Frequency};
use camera::{camera_config_t__bindgen_ty_1, camera_config_t__bindgen_ty_2};
use esp_idf_hal::{gpio::*, i2c::I2cDriver, peripheral::Peripheral};
use esp_idf_sys::*;
use firmware_core::configuration::peripherals::camera::{Camera as CameraTrait, Image};
pub use frame_buffer::FrameBuffer;
pub use sensor::*;
pub use settings::*;

pub struct Camera<'a>
{
	_p: PhantomData<&'a ()>,
}

impl<'a> Camera<'a>
{
	pub const I2C_CONFIGURATION: esp_idf_hal::i2c::config::Config = esp_idf_hal::i2c::config::Config {
		baudrate: esp_idf_hal::prelude::Hertz(100_000),
		sda_pullup_enabled: true,
		scl_pullup_enabled: true,
		timeout: None,
		intr_flags: enumset::EnumSet::EMPTY,
	};

	pub fn new(
		pin_pwdn: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
		pin_reset: Option<impl Peripheral<P = impl InputPin + OutputPin> + 'a>,
		pin_xclk: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
		pin_d0: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
		pin_d1: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
		pin_d2: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
		pin_d3: impl Peripheral<P = impl InputPin + OutputPin> + 'a, pin_d4: impl Peripheral<P = impl InputPin> + 'a,
		pin_d5: impl Peripheral<P = impl InputPin> + 'a, pin_d6: impl Peripheral<P = impl InputPin> + 'a,
		pin_d7: impl Peripheral<P = impl InputPin> + 'a,
		pin_vsync: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
		pin_href: impl Peripheral<P = impl InputPin + OutputPin> + 'a,
		pin_pclk: impl Peripheral<P = impl InputPin + OutputPin> + 'a, i2c_driver: &I2cDriver<'a>,
		ledc_timer: esp_idf_sys::ledc_timer_t, ledc_channel: esp_idf_sys::ledc_channel_t, xclk_frequency: Frequency,
		pixel_format: PixelFormat, frame_size: FrameSize, camera_grab_mode: CameraGrabMode,
		frame_buffer_location: FrameBufferLocation,
	) -> Result<Self, esp_idf_sys::EspError>
	{
		esp_idf_hal::into_ref!(
			pin_pwdn, pin_xclk, pin_d0, pin_d1, pin_d2, pin_d3, pin_d4, pin_d5, pin_d6, pin_d7, pin_vsync, pin_href,
			pin_pclk
		);
		let config = camera::camera_config_t {
			pin_pwdn: pin_pwdn.pin(),
			pin_reset: pin_reset.map(|pin| pin.into_ref().pin()).unwrap_or(-1),
			pin_xclk: pin_xclk.pin(),

			__bindgen_anon_1: camera_config_t__bindgen_ty_1 { pin_sccb_sda: -1 },
			__bindgen_anon_2: camera_config_t__bindgen_ty_2 { pin_sccb_scl: -1 },

			pin_d0: pin_d0.pin(),
			pin_d1: pin_d1.pin(),
			pin_d2: pin_d2.pin(),
			pin_d3: pin_d3.pin(),
			pin_d4: pin_d4.pin(),
			pin_d5: pin_d5.pin(),
			pin_d6: pin_d6.pin(),
			pin_d7: pin_d7.pin(),
			pin_vsync: pin_vsync.pin(),
			pin_href: pin_href.pin(),
			pin_pclk: pin_pclk.pin(),

			xclk_freq_hz: xclk_frequency.as_hertz() as i32,

			sccb_i2c_port: i2c_driver.port(),

			ledc_timer,
			ledc_channel,

			pixel_format: pixel_format.into(),
			frame_size: frame_size.into(),

			jpeg_quality: 10,
			fb_count: 2,
			grab_mode: camera_grab_mode.into(),

			fb_location: frame_buffer_location.into(),
			..Default::default()
		};

		esp_idf_sys::esp!(unsafe { camera::esp_camera_init(&config) })?;
		let self_ = Self { _p: PhantomData };

		let sensor = self_.get_sensor();
		sensor.set_brightness(0)?;
		sensor.set_contrast(0)?;
		sensor.set_saturation(0)?;
		sensor.set_special_effect(0)?;
		sensor.set_whitebal(true)?;
		sensor.set_awb_gain(true)?;
		sensor.set_wb_mode(0)?; // 0 to 4 - if awb_gain enabled (0 - Auto, 1 - Sunny, 2 - Cloudy, 3 - Office, 4 - Home)
		sensor.set_exposure_ctrl(true)?;
		sensor.set_aec2(false)?;
		sensor.set_gain_ctrl(false)?;
		sensor.set_agc_gain(3)?; // 0 to 30
		sensor.set_gainceiling(6)?; // 0 to 6
		sensor.set_bpc(false)?;
		sensor.set_wpc(true)?;
		sensor.set_raw_gma(false)?; // If enabled (makes much lighter and noisy)
		sensor.set_lenc(true)?;
		sensor.set_hmirror(false)?;
		sensor.set_vflip(false)?;
		sensor.set_dcw(true)?;
		sensor.set_colorbar(false)?;

		Ok(self_)
	}

	pub fn get_framebuffer(&self) -> Option<FrameBuffer>
	{
		let fb = unsafe { camera::esp_camera_fb_get() };
		if fb.is_null()
		{
			None
		}
		else
		{
			Some(FrameBuffer { fb, _p: PhantomData })
		}
	}

	pub fn get_sensor(&self) -> CameraSensor<'a>
	{
		CameraSensor {
			sensor: unsafe { camera::esp_camera_sensor_get() },
			_p: PhantomData,
		}
	}
}

impl<'a> Drop for Camera<'a>
{
	fn drop(&mut self)
	{
		esp!(unsafe { camera::esp_camera_deinit() }).expect("error during esp_camera_deinit")
	}
}

impl<'a> CameraTrait for Camera<'a>
{
	type Image<'b> = FrameBuffer<'b> where Self: 'b;
	type Error = CameraError;

	fn get_image<'b>(&'b self) -> Result<Self::Image<'b>, Self::Error>
	{
		let framebuffer = self.get_framebuffer().ok_or(CameraError::NoFrameBuffer)?;

		Ok(framebuffer)
	}
}

#[derive(Debug)]
pub enum CameraError
{
	NoFrameBuffer,
}

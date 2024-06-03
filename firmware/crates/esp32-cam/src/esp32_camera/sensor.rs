#![allow(dead_code)]

use std::marker::PhantomData;

use esp_idf_sys::*;

pub struct CameraSensor<'a>
{
	pub(super) sensor: *mut camera::sensor_t,
	pub(super) _p: PhantomData<&'a camera::sensor_t>,
}

impl<'a> CameraSensor<'a>
{
	pub fn init_status(&self) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).init_status.unwrap()(self.sensor) })
	}
	pub fn reset(&self) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).reset.unwrap()(self.sensor) })
	}
	pub fn set_pixformat(&self, format: camera::pixformat_t) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_pixformat.unwrap()(self.sensor, format) })
	}
	pub fn set_framesize(&self, framesize: camera::framesize_t) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_framesize.unwrap()(self.sensor, framesize) })
	}
	pub fn set_contrast(&self, level: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_contrast.unwrap()(self.sensor, level) })
	}
	pub fn set_brightness(&self, level: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_brightness.unwrap()(self.sensor, level) })
	}
	pub fn set_saturation(&self, level: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_saturation.unwrap()(self.sensor, level) })
	}
	pub fn set_sharpness(&self, level: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_sharpness.unwrap()(self.sensor, level) })
	}
	pub fn set_denoise(&self, level: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_denoise.unwrap()(self.sensor, level) })
	}
	pub fn set_gainceiling(&self, gainceiling: camera::gainceiling_t) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_gainceiling.unwrap()(self.sensor, gainceiling) })
	}
	pub fn set_quality(&self, quality: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_quality.unwrap()(self.sensor, quality) })
	}
	pub fn set_colorbar(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_colorbar.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_whitebal(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_whitebal.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_gain_ctrl(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_gain_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_exposure_ctrl(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_exposure_ctrl.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_hmirror(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_hmirror.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_vflip(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_vflip.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_aec2(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_aec2.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_awb_gain(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_awb_gain.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_agc_gain(&self, gain: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_agc_gain.unwrap()(self.sensor, gain) })
	}
	pub fn set_aec_value(&self, gain: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_aec_value.unwrap()(self.sensor, gain) })
	}
	pub fn set_special_effect(&self, effect: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_special_effect.unwrap()(self.sensor, effect) })
	}
	pub fn set_wb_mode(&self, mode: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_wb_mode.unwrap()(self.sensor, mode) })
	}
	pub fn set_ae_level(&self, level: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_ae_level.unwrap()(self.sensor, level) })
	}
	pub fn set_dcw(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_dcw.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_bpc(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_bpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_wpc(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_wpc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_raw_gma(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_raw_gma.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn set_lenc(&self, enable: bool) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_lenc.unwrap()(self.sensor, if enable { 1 } else { 0 }) })
	}
	pub fn get_reg(&self, reg: i32, mask: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).get_reg.unwrap()(self.sensor, reg, mask) })
	}
	pub fn set_reg(&self, reg: i32, mask: i32, value: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_reg.unwrap()(self.sensor, reg, mask, value) })
	}
	pub fn set_res_raw(
		&self, start_x: i32, start_y: i32, end_x: i32, end_y: i32, offset_x: i32, offset_y: i32, total_x: i32,
		total_y: i32, output_x: i32, output_y: i32, scale: bool, binning: bool,
	) -> Result<(), EspError>
	{
		esp!(unsafe {
			(*self.sensor).set_res_raw.unwrap()(
				self.sensor,
				start_x,
				start_y,
				end_x,
				end_y,
				offset_x,
				offset_y,
				total_x,
				total_y,
				output_x,
				output_y,
				scale,
				binning,
			)
		})
	}
	pub fn set_pll(
		&self, bypass: i32, mul: i32, sys: i32, root: i32, pre: i32, seld5: i32, pclken: i32, pclk: i32,
	) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_pll.unwrap()(self.sensor, bypass, mul, sys, root, pre, seld5, pclken, pclk,) })
	}
	pub fn set_xclk(&self, timer: i32, xclk: i32) -> Result<(), EspError>
	{
		esp!(unsafe { (*self.sensor).set_xclk.unwrap()(self.sensor, timer, xclk) })
	}
}

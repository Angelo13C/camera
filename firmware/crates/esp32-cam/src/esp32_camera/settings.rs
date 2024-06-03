#![allow(unused)]
use esp_idf_sys::camera;

#[repr(C)]
pub enum PixelFormat
{
	/// 2BPP/RGB565
	RGB565,
	/// 2BPP/YUV422
	YUV422,
	/// 1.5BPP/YUV420
	YUV420,
	/// 1BPP/GRAYSCALE
	GRAYSCALE,
	/// JPEG/COMPRESSED
	JPEG,
	/// 3BPP/RGB888
	RGB888,
	/// RAW
	RAW,
	/// 3BP2P/RGB444
	RGB444,
	/// 3BP2P/RGB555
	RGB555,
}

impl Into<camera::pixformat_t> for PixelFormat
{
	fn into(self) -> camera::pixformat_t
	{
		match self
		{
			PixelFormat::RGB565 => camera::pixformat_t_PIXFORMAT_RGB565,
			PixelFormat::YUV422 => camera::pixformat_t_PIXFORMAT_YUV422,
			PixelFormat::YUV420 => camera::pixformat_t_PIXFORMAT_YUV420,
			PixelFormat::GRAYSCALE => camera::pixformat_t_PIXFORMAT_GRAYSCALE,
			PixelFormat::JPEG => camera::pixformat_t_PIXFORMAT_JPEG,
			PixelFormat::RGB888 => camera::pixformat_t_PIXFORMAT_RGB888,
			PixelFormat::RAW => camera::pixformat_t_PIXFORMAT_RAW,
			PixelFormat::RGB444 => camera::pixformat_t_PIXFORMAT_RGB444,
			PixelFormat::RGB555 => camera::pixformat_t_PIXFORMAT_RGB555,
		}
	}
}

impl From<camera::pixformat_t> for PixelFormat
{
	fn from(value: camera::pixformat_t) -> Self
	{
		match value
		{
			camera::pixformat_t_PIXFORMAT_RGB565 => PixelFormat::RGB565,
			camera::pixformat_t_PIXFORMAT_YUV422 => PixelFormat::YUV422,
			camera::pixformat_t_PIXFORMAT_YUV420 => PixelFormat::YUV420,
			camera::pixformat_t_PIXFORMAT_GRAYSCALE => PixelFormat::GRAYSCALE,
			camera::pixformat_t_PIXFORMAT_JPEG => PixelFormat::JPEG,
			camera::pixformat_t_PIXFORMAT_RGB888 => PixelFormat::RGB888,
			camera::pixformat_t_PIXFORMAT_RAW => PixelFormat::RAW,
			camera::pixformat_t_PIXFORMAT_RGB444 => PixelFormat::RGB444,
			camera::pixformat_t_PIXFORMAT_RGB555 => PixelFormat::RGB555,
			_ => panic!("Invalid value"),
		}
	}
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum FrameSize
{
	/// 96x96
	Size96X96,
	/// 160x120
	QQVGA,
	/// 176x144
	QCIF,
	/// 240x176
	HQVGA,
	/// 240x240
	Size240X240,
	/// 320x240
	QVGA,
	/// 400x296
	CIF,
	/// 480x320
	HVGA,
	/// 640x480
	VGA,
	/// 800x600
	SVGA,
	/// 1024x768
	XGA,
	/// 1280x720
	HD,
	/// 1280x1024
	SXGA,
	/// 1600x1200
	UXGA,

	// 3MP Sensors
	/// 1920x1080
	FHD,
	///  720x1280
	P_HD,
	///  864x1536
	P_3MP,
	/// 2048x1536
	QXGA,

	// 5MP Sensors
	/// 2560x1440
	QHD,
	/// 2560x1600
	WQXGA,
	/// 1080x1920
	P_FHD,
	/// 2560x1920
	QSXGA,

	/// Invalid
	INVALID,
}

impl Into<camera::framesize_t> for FrameSize
{
	fn into(self) -> camera::framesize_t
	{
		match self
		{
			FrameSize::Size96X96 => camera::framesize_t_FRAMESIZE_96X96,
			FrameSize::QQVGA => camera::framesize_t_FRAMESIZE_QQVGA,
			FrameSize::QCIF => camera::framesize_t_FRAMESIZE_QCIF,
			FrameSize::HQVGA => camera::framesize_t_FRAMESIZE_HQVGA,
			FrameSize::Size240X240 => camera::framesize_t_FRAMESIZE_240X240,
			FrameSize::QVGA => camera::framesize_t_FRAMESIZE_QVGA,
			FrameSize::CIF => camera::framesize_t_FRAMESIZE_CIF,
			FrameSize::HVGA => camera::framesize_t_FRAMESIZE_HVGA,
			FrameSize::VGA => camera::framesize_t_FRAMESIZE_VGA,
			FrameSize::SVGA => camera::framesize_t_FRAMESIZE_SVGA,
			FrameSize::XGA => camera::framesize_t_FRAMESIZE_XGA,
			FrameSize::HD => camera::framesize_t_FRAMESIZE_HD,
			FrameSize::SXGA => camera::framesize_t_FRAMESIZE_SXGA,
			FrameSize::UXGA => camera::framesize_t_FRAMESIZE_UXGA,
			FrameSize::FHD => camera::framesize_t_FRAMESIZE_FHD,
			FrameSize::P_HD => camera::framesize_t_FRAMESIZE_P_HD,
			FrameSize::P_3MP => camera::framesize_t_FRAMESIZE_P_3MP,
			FrameSize::QXGA => camera::framesize_t_FRAMESIZE_QXGA,
			FrameSize::QHD => camera::framesize_t_FRAMESIZE_QHD,
			FrameSize::WQXGA => camera::framesize_t_FRAMESIZE_WQXGA,
			FrameSize::P_FHD => camera::framesize_t_FRAMESIZE_P_FHD,
			FrameSize::QSXGA => camera::framesize_t_FRAMESIZE_QSXGA,
			FrameSize::INVALID => camera::framesize_t_FRAMESIZE_INVALID,
		}
	}
}

/// Configuration structure for camera initialization
pub enum CameraGrabMode
{
	/// Fills buffers when they are empty. Less resources but first 'fb_count' frames might be old
	WhenEmpty,
	/// Except when 1 frame buffer is used, queue will always contain the last 'fb_count' frames
	Latest,
}

impl Into<camera::camera_grab_mode_t> for CameraGrabMode
{
	fn into(self) -> camera::camera_grab_mode_t
	{
		match self
		{
			CameraGrabMode::WhenEmpty => camera::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY,
			CameraGrabMode::Latest => camera::camera_grab_mode_t_CAMERA_GRAB_LATEST,
		}
	}
}

/// Camera frame buffer location
pub enum FrameBufferLocation
{
	/// Frame buffer is placed in external PSRAM
	PSRAM,
	/// Frame buffer is placed in internal DRAM
	DRAM,
}

impl Into<camera::camera_fb_location_t> for FrameBufferLocation
{
	fn into(self) -> camera::camera_fb_location_t
	{
		match self
		{
			FrameBufferLocation::PSRAM => camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
			FrameBufferLocation::DRAM => camera::camera_fb_location_t_CAMERA_FB_IN_DRAM,
		}
	}
}

#[allow(non_camel_case_types)]
type time_t = u32;
#[allow(non_camel_case_types)]
type suseconds_t = u32;
#[repr(C)]
struct Timeval
{
	/// Seconds
	tv_sec: time_t,
	/// Microseconds
	tv_usec: suseconds_t,
}

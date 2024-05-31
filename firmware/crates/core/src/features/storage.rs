use a13c_embedded::features::storage::embedded_sdmmc::*;

pub struct Storage<
	S: embedded_hal::spi::SpiDevice,
	CS: embedded_hal::digital::OutputPin,
	D: embedded_hal::delay::DelayNs,
	T: TimeSource,
	const MAX_DIRS: usize = 3,
	const MAX_FILES: usize = 3,
	const MAX_VOLUMES: usize = 1,
> {
	volume_manager: VolumeManager<SdCard<S, CS, D>, T, MAX_DIRS, MAX_FILES, MAX_VOLUMES>,
	volume0: RawVolume,
	raw_root_dir: Option<RawDirectory>,
}

impl<
		S: embedded_hal::spi::SpiDevice,
		CS: embedded_hal::digital::OutputPin,
		D: embedded_hal::delay::DelayNs,
		T: TimeSource,
		const MAX_DIRS: usize,
		const MAX_FILES: usize,
		const MAX_VOLUMES: usize,
	> Storage<S, CS, D, T, MAX_DIRS, MAX_FILES, MAX_VOLUMES>
{
	pub fn new(spi: S, cs: CS, delay: D, time_source: T) -> Result<Self, Error<SdCardError>>
	{
		let sdcard = SdCard::new(spi, cs, delay);

		match sdcard.num_bytes()
		{
			Ok(num_bytes) => log::info!("Card size is {} bytes", num_bytes),
			Err(error) => log::warn!("Error: {:#?}", error),
		}

		let mut volume_manager = VolumeManager::new_with_limits(sdcard, time_source, 0);
		let mut volume0 = volume_manager.open_volume(VolumeIdx(0))?;
		let raw_root_dir = volume0.open_root_dir()?.to_raw_directory();

		let volume0 = volume0.to_raw_volume();

		Ok(Self {
			volume_manager,
			volume0,
			raw_root_dir: Some(raw_root_dir),
		})
	}

	pub fn store_image(
		&mut self, image: &[u8], file_name: &str,
	) -> Result<(), Error<<SdCard<S, CS, D> as BlockDevice>::Error>>
	{
		if let Some(raw_root_dir) = self.raw_root_dir.take()
		{
			let mut root_dir = raw_root_dir.to_directory(&mut self.volume_manager);
			let mut my_file = root_dir.open_file_in_dir(file_name, Mode::ReadWriteCreateOrTruncate)?;
			my_file.write(image)?;
			core::mem::drop(my_file);
			self.raw_root_dir = Some(root_dir.to_raw_directory());
		}

		Ok(())
	}
}

impl<
		S: embedded_hal::spi::SpiDevice,
		CS: embedded_hal::digital::OutputPin,
		D: embedded_hal::delay::DelayNs,
		T: TimeSource,
		const MAX_DIRS: usize,
		const MAX_FILES: usize,
		const MAX_VOLUMES: usize,
	> Drop for Storage<S, CS, D, T, MAX_DIRS, MAX_FILES, MAX_VOLUMES>
{
	fn drop(&mut self)
	{
		if let Some(raw_root_dir) = self.raw_root_dir.take()
		{
			self.volume_manager.close_dir(raw_root_dir).unwrap();
		}
		self.volume_manager.close_volume(self.volume0).unwrap();
	}
}

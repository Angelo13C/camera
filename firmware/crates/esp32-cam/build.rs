use std::path::Path;

fn main()
{
	embuild::espidf::sysenv::output();

	set_environment_variables();
}

fn set_environment_variables()
{
	const DIRECTORY_PATH: &str = "../../../private/Secrets/";

	fn set_environment_variable(relative_path: &str, environment_variable_key: &str)
	{
		if let Ok(environment_variable_value) =
			std::fs::read_to_string(Path::new(DIRECTORY_PATH).join(Path::new(relative_path)))
		{
			println!(
				"cargo:rustc-env={}={}",
				environment_variable_key, environment_variable_value
			);
		}
	}

	set_environment_variable("WiFi/SSID.txt", "WIFI_SSID");
	set_environment_variable("WiFi/Password.txt", "WIFI_PASSWORD");
}

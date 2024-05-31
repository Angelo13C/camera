mod data;

use a13c_embedded::{features::communication::http::server::HttpServer, impl_http_requests};
use embedded_svc::http::{
	server::{Connection, Request},
	Method,
};
use strum::{EnumCount, IntoEnumIterator};

pub use self::data::HttpServerData;

pub const STACK_SIZE: usize = 1_000;

pub fn register_all_requests<
	S: HttpServer<Error = E, HttpRequest = PossibleHttpRequest>,
	StreamS: HttpServer<Error = StreamE, HttpRequest = stream::PossibleHttpRequest>,
	E,
	StreamE,
>(
	http_server: &mut S, stream_http_server: &mut StreamS, data: <stream::PossibleHttpRequest as HttpRequest>::Data,
) -> Result<(), RegisterError<E, StreamE>>
{
	for possible_request in PossibleHttpRequest::iter()
	{
		http_server
			.register_request(possible_request, ())
			.map_err(RegisterError::Main)?;
	}
	for possible_request in stream::PossibleHttpRequest::iter()
	{
		stream_http_server
			.register_request(possible_request, data.clone())
			.map_err(RegisterError::Stream)?;
	}

	Ok(())
}

#[derive(Debug)]
pub enum RegisterError<E, StreamE>
{
	Main(E),
	Stream(StreamE),
}

impl_http_requests!((),
	Index => Method::Get => "/" => index
);

fn index<C: Connection>(mut request: Request<&mut C>, _: ()) -> Result<(), C::Error>
{
	log::info!("Start handling `index` request");

	const INDEX_HTML: &'static [u8] = include_bytes!("../../../../../../website/index.html");
	let mut response = request.into_response(OK_RESPONSE, None, &[("Access-Control-Allow-Origin", "*")])?;

	response.write(INDEX_HTML)?;

	Ok(())
}

const OK_RESPONSE: u16 = 200;

pub mod stream
{
	use super::*;

	impl_http_requests!(HttpServerData,
		Stream => Method::Get => "/stream" => stream
	);

	// Check this: https://stackoverflow.com/questions/47729941/mjpeg-over-http-specification
	fn stream<C: Connection>(request: Request<&mut C>, mut data: HttpServerData) -> Result<(), C::Error>
	{
		log::info!("Start handling `stream` request");

		const BOUNDARY: &'static str = "\r\n--123456789000000000000987654321\r\n";

		let mut response = request.into_response(
			OK_RESPONSE,
			None,
			&[
				embedded_svc::http::headers::content_type(
					"multipart/x-mixed-replace;boundary=123456789000000000000987654321",
				),
				("Access-Control-Allow-Origin", "*"),
				("X-Framerate", "60"),
			],
		)?;

		loop
		{
			let _ = data.read_image_bytes(|image| {
				if let Some((image, timestamp)) = image
				{
					let mut result = response.write(BOUNDARY.as_bytes())?;
					result += response.write(b"Content-Type: image/jpeg\r\n")?;
					response.write(format!("Content-Length: {}\r\n", image.len()).as_bytes())?;
					result += response.write(
						format!(
							"X-Timestamp: {}.{:06}\r\n\r\n",
							timestamp.as_secs(),
							timestamp.subsec_micros()
						)
						.as_bytes(),
					)?;
					result += response.write(image)?;
					Ok(result)
				}
				else
				{
					Ok(0)
				}
			})?;
		}

		Ok(())
	}
}

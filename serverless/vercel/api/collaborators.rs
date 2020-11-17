use http::{StatusCode};
use now_lambda::{lambda, error::NowError, IntoResponse, Request, Response};
use std::error::Error;
use utils::collaborators;

fn handler(_: Request) -> Result<impl IntoResponse, NowError> {
    println!("{}", collaborators);
	let response = Response::builder()
		.status(StatusCode::OK)
		.header("Content-Type", "application/json")
		.body(collaborators.to_string())
		.expect("Internal Server Error");

        Ok(response)
}

// Start the runtime with the handler
fn main() -> Result<(), Box<dyn Error>> {
	Ok(lambda!(handler))
}
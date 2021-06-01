use fastly::{Error, Request, Response};

const BACKEND_NAME: &str = "backend_name";

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    let mut response = req.send(BACKEND_NAME)?;
    response.set_header("some-testing", "header");
    Ok(response)
}

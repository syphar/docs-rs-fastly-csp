use fastly::{Error, Request};
use lol_html::{element, HtmlRewriter, Settings};

const BACKEND_NAME: &str = "backend_name";

fn main() -> Result<(), Error> {
    let req = Request::from_client();
    let mut response = req.send(BACKEND_NAME)?;

    // TODO: decide by content-type if we want to rewrite or not.

    let mut random = [0u8; 36];
    getrandom::getrandom(&mut random).expect("failed to generate a nonce");

    let nonce_attr = format!("nonce-{}", base64::encode(&random));

    response.set_header(
        "Content-Security-Policy",
        format!(
            "default-src 'none'; \
            base-uri 'none'; \
            img-src 'self' https:; \
            style-src 'self'; \
            font-src 'self'; \
            script-src '{}'",
            nonce_attr,
        ),
    );

    let mut backend_response_body = response.take_body();
    let mut new_body_sink = response.stream_to_client();

    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("script", |el| {
                el.set_attribute("nonce", &nonce_attr)?;
                Ok(())
            })],
            ..Settings::default()
        },
        |rewrite_result: &[u8]| {
            new_body_sink.write_bytes(rewrite_result);
        },
    );

    for chunk in backend_response_body.read_chunks(4096) {
        let chunk = chunk?;
        rewriter.write(&chunk)?;
    }

    rewriter.end()?;

    Ok(())
}

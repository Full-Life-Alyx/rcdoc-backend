use poem_openapi::{payload::PlainText, OpenApi};

pub struct TestService;

#[OpenApi]
impl TestService {
    #[oai(path = "/ping", method = "get")]
    async fn ping(&self) -> PlainText<&'static str> {
        PlainText("PONG!")
    }
}



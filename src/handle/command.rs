pub struct CommandHandler;

type JsonResult<T> = Result<(StatusCode, Json<T>), StatusCode>;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewQuoteResponse {
    order_id: i64,
}

impl CommandHandler {
    pub async fn create_quote(State(mut service): State<T>) -> JsonResult<NewQuoteResponse> {
        todo!()
    }
}

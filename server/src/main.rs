async fn handle_event(mut request: tide::Request<()>) -> Result<String, tide::Error> {
    let post = request.body_string().await.unwrap_or_else(|_| "".to_string());
    Ok("Edit".to_string())
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();
    let mut app = tide::new();
    app.at("/meal_vote").post(handle_event);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

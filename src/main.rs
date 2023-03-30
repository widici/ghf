mod api;
use crate::api::request::{UserData, request_user};

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let data: UserData = request_user("jake").await?;
    println!("{:?}", data.joined);
    Ok(())
}
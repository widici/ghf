mod api;
use crate::api::request::{UserData, request_user};

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let data: UserData = request_user("jake").await?;
    println!("{:?}", data.joined);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::request::{UserData, request_user};

    #[tokio::test]
    async fn test_request_user() -> Result<(), reqwest::Error>{
        let _: UserData = request_user("jake").await?;
        Ok(())
    }
}
mod api;
use crate::api::profile::{ProfileData, request_profile};

#[tokio::main]
pub async fn main() -> Result<(), reqwest::Error> {
    let data: ProfileData = request_profile("jake").await?;
    println!("{:?}", data.joined);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::api::profile::{ProfileData, request_profile};

    #[tokio::test]
    async fn test_request_profile() -> Result<(), reqwest::Error> {
        let user: ProfileData = request_profile("widici").await?;
        assert_eq!(&user.joined, "2021-05-14T20:14:08Z");
        Ok(())
    }
}


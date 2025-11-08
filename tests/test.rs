#[cfg(test)]
mod tests {
    
    
    use monnify_rs::*;
    

    #[tokio::test]
    async fn test_get_token() {
        let payload = AuthRequest {
            api_key: "MK_TEST_DZ6E3NAEU5".to_string(),
            secret_key: "BMRVEMP7J84HEH6D0AYP70EK8CK96WZX".to_string(),
        };
    
        match get_token(payload).await {
            Ok(response) => println!("✅ Token: {:?}", response.response_body.access_token),
            Err(e) => eprintln!("❌ Failed: {:?}", e),
        }
        
    }
    
}

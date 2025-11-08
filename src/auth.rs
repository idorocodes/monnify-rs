use reqwest::{self, Client};
use base64::{engine::general_purpose::STANDARD,Engine as _};
use serde::{Deserialize,Serialize};
use anyhow::{Error, Ok};


#[derive(Serialize,Deserialize)]
pub struct AuthRequest{
    api_key : String,
    secret_key  : String,
}

#[derive(Deserialize,Serialize)]
pub struct AuthResponse{
     requestSuccessful : bool,
     responseMessage : String,
     responseCode : String,
     responseBody : ResponseBody
}
#[derive(Deserialize,Serialize)]
pub struct ResponseBody{
 accessToken : String,
 expiresIn : i32
}


 pub async fn get_token(payload:AuthRequest) -> Result<AuthResponse,Error>{
 
    let api_key = payload.api_key;
    let secret_key = payload.secret_key;
    let to = format!("{}:{}",api_key,secret_key);
    let auth = STANDARD.encode(to);
    
    let basic_text  =  format!("Basic {}",auth);
    
    let url = "https://sandbox.monnify.com/api/v1/auth/login";
    
    let client = Client::new();
    let res  = client.post(url)
        .header("Authorization", auth)
        .send()
        .await?;
 
    
    
    Ok()
}
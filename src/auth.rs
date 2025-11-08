use reqwest::{self, Client};
use base64::{engine::general_purpose::STANDARD,Engine as _};
use serde::{Deserialize,Serialize};
use anyhow::{Error, Ok};


#[derive(Serialize,Deserialize,Debug)]
pub struct AuthRequest{
    pub api_key : String,
    pub secret_key  : String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    pub request_successful: bool,
    pub response_message: String,
    pub response_code: String,
    pub response_body: ResponseBody,
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    pub access_token: String,
    pub expires_in: i32,
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
        .header("Authorization", basic_text)
        .send()
        .await?;

    let request_respons = res.json().await?;
       



    Ok(request_respons)
}

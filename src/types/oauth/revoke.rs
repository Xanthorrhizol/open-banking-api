use super::ResponseCode;
use crate::types::{HttpMethod, Scope};
use serde::{Deserialize, Serialize};

pub const METHOD: HttpMethod = HttpMethod::Post;

/// 토큰폐기 API request body
/// - client_id: 오픈뱅킹에서 발급한 이용기관 앱의 Client ID
/// - client_secret: 오픈뱅킹에서 발급한 이용기관 앱의 Client Secret
/// - access_token: 폐기하고자 하는 Access Token
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    client_id: String,
    client_secret: String,
    access_token: String,
}

impl RequestBody {
    pub fn new(client_id: String, client_secret: String, access_token: String) -> Self {
        Self {
            client_id,
            client_secret,
            access_token,
        }
    }
}

/// 토큰폐기 API request body
/// - rsp_code: 응답코드(API)(5자리)
/// - rsp_message: 응답메시지(API)(300자리)
/// - client_id: 오픈뱅킹에서 발급한 이용기관 앱의 Client ID
/// - client_secret: 오픈뱅킹에서 발급한 이용기관 앱의 Client Secret
/// - access_token: 폐기한 Access Token
/// - refresh_token: 폐기한 Refresh Token(Refresh Token 이 있는 경우 명시)
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    rsp_code: ResponseCode,
    rsp_message: String,
    client_id: Option<String>,
    client_secret: Option<String>,
    access_token: Option<String>,
    refersh_token: Option<String>,
}

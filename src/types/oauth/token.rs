use super::ResponseCode;
use crate::types::{HttpMethod, Scope};
use serde::{Deserialize, Serialize};

pub const METHOD: HttpMethod = HttpMethod::Post;

/// 토큰발급 API request body
/// - code: 사용자인증 성공 후 획득한 Authorization Code
/// - client_id: 오픈뱅킹에서 발급한 이용기관 앱의 Client ID
/// - client_secret: 오픈뱅킹에서 발급한 이용기관 앱의 Client Secret
/// - redirect_uri: Access Token 을 전달받을 Callback URL
/// - grant_type: 3-legged 인증을 위한 권한부여 방식 지정(고정값: "authorization_code")
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestBody {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String,
}

impl RequestBody {
    pub fn new(
        code: String,
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            code,
            client_id,
            client_secret,
            redirect_uri,
            grant_type: "authorization_code".to_string(),
        }
    }
}

/// 토큰발급 API response body
/// - access_token: 오픈뱅킹에서 발행된 Access Token
/// - token_type: Access Token 유형(고정값: Bearer)
/// - expires_in: Access Token 만료 기간(초)
/// - refresh_token: Access Token 갱신 시 사용하는 Refresh Token
/// - scope: Access Token 권한 범위(사용자인증 시 요청했던 권한 범위와 동일)
/// - user_seq_no: 사용자일련번호(10자리)
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    rsp_code: ResponseCode,
    rsp_message: String,
    access_token: Option<String>,
    token_type: Option<String>,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<Scope>,
    user_seq_no: Option<String>,
}

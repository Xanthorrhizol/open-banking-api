use super::ResponseCode;
use crate::types::{AuthType, ClientDeviceType, HttpMethod, Lang, RegisterKind, Scope};
use serde::{Deserialize, Serialize};

pub const METHOD: HttpMethod = HttpMethod::Post;

/// 인증생략 이용 시 이용하는 헤더
/// - user_seq_no(Kftc-Bfop-UserSeqNo): 기존 고객의 사용자일련번호
/// - user_connection_info(Kftc-Bfop-UserCI): 사용자 CI(Connection Info)
/// - access_token(Kftc-Bfop-AccessToken): "login" scope을 포함한 토큰
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Header {
    user_seq_no: String,
    user_connection_info: String,
    access_token: String,
}

impl Header {
    pub fn new(user_seq_no: String, user_connection_info: String, access_token: String) -> Self {
        Self {
            user_seq_no,
            user_connection_info,
            access_token,
        }
    }
}

/// 사용자인증/서비스등록확인 API request body
/// - response_type: OAuth 2.0 인증요청 시 반환되는 형태(고정값 "code")
/// - client_id: 오픈뱅킹에서 발급한 이용기관 앱의 Client ID
/// - redirect_uri: 사용자인증이 성공하면 이용기관으로 연결되는 URL
/// - scope: Access Token 권한 범위
/// - client_info: 이용기관이 세팅하는 임의의 정보(Callback 호출 시 그대로 전달됨)
/// - state: CSRF 보안위협에 대응하기 위해 이용기관이 세팅하는 난수값
/// - auth_type: 사용자인증타입 구분
/// - lang: 다국어 설정
/// - cellphone_cert_yn: 휴대전화 인증 사용여부(미지정시 기본값 : true)
/// - authorized_cert_yn: 공동･금융인증서 사용여부(미지정시 기본값 : true)
/// - account_hold_auth_yn: 계좌소유인증 사용여부(register_info가 Account인 경우만 true 가능)
/// - register_info: 등록정보 종류(미지정 시 기본값 : Account)
/// - accountinfo_yn: 계좌통합조회 사용여부. 어카운트인포의 계좌통합조회를 통해 계좌를 등록하는 경우 true(등록정보가 ‘계좌’인 경우만 가능)
/// - accountinfo_api_tran_id: (accountinfo_yn가 true인 경우 필수) 계좌통합조회 결과 중 고객이 선택한 특정 계좌 정보 입력 (계좌통합조회 결과로 수신한 값을 그대로 입력해야 함)
/// - accountinfo_list_num: (accountinfo_yn가 true인 경우 필수) 계좌통합조회 결과 중 고객이 선택한 특정 계좌 정보 입력 (계좌통합조회 결과로 수신한 값을 그대로 입력해야 함)
/// - client_device_type: 고객 접속 단말기 구분
/// - client_device_ip: 고객의 접속 단말기 IP 주소
/// - client_device_mac: 고객의 접속 단말기 MAC 주소
/// - client_device_id: 고객의 접속 단말기를 구분할 수 있는 고유식별정보(PC: HDD Serial Number, Android: SSAID, Ios: UUID)
/// - client_device_num: 고객의 접속 단말기의 휴대폰번호. 단말기 구분이 모바일(‘AD’, ‘IO’) 인 경우 설정(‘-’는 제외함)
/// - client_device_version: 고객의 접속 단말기 OS 버전
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct RequestBody {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    scope: Vec<Scope>,
    client_info: Option<Vec<u8>>,
    state: Vec<u8>,
    auth_type: AuthType,
    lang: Option<Lang>,
    cellphone_cert_yn: Option<bool>,
    authorized_cert_yn: Option<bool>,
    account_hold_auth_yn: Option<bool>,
    register_info: Option<RegisterKind>,
    accountinfo_yn: Option<bool>,
    accountinfo_api_tran_id: Option<String>,
    accountinfo_list_num: Option<Vec<i32>>,
    client_device_type: Option<ClientDeviceType>,
    client_device_ip: Option<String>,
    client_device_mac: Option<String>,
    client_device_id: Option<String>,
    client_device_num: Option<String>,
    client_device_version: Option<String>,
}

impl RequestBody {
    /// 사용자인증 API Request Body
    pub(crate) fn new_authorize(
        client_id: String,
        redirect_uri: String,
        scope: Vec<Scope>,
        client_info: Option<Vec<u8>>,
        state: Vec<u8>,
        auth_type: AuthType,
        lang: Option<Lang>,
        cellphone_cert_yn: Option<bool>,
        authorized_cert_yn: Option<bool>,
        account_hold_auth_yn: Option<bool>,
        register_info: Option<RegisterKind>,
        accountinfo_yn: Option<bool>,
        accountinfo_api_tran_id: Option<String>,
        accountinfo_list_num: Option<Vec<i32>>,
        client_device_type: Option<ClientDeviceType>,
        client_device_ip: Option<String>,
        client_device_mac: Option<String>,
        client_device_id: Option<String>,
        client_device_num: Option<String>,
        client_device_version: Option<String>,
    ) -> Self {
        Self {
            response_type: "code".to_string(),
            client_id,
            redirect_uri,
            scope,
            client_info,
            state,
            auth_type,
            lang,
            cellphone_cert_yn,
            authorized_cert_yn,
            account_hold_auth_yn,
            register_info,
            accountinfo_yn,
            accountinfo_api_tran_id,
            accountinfo_list_num,
            client_device_type,
            client_device_ip,
            client_device_mac,
            client_device_id,
            client_device_num,
            client_device_version,
        }
    }

    /// 서비스등록확인 API Request Body
    pub(crate) fn new_authorize_account(
        client_id: String,
        redirect_uri: String,
        scope: Vec<Scope>,
        client_info: Option<Vec<u8>>,
        state: Vec<u8>,
        auth_type: AuthType,
        lang: Option<Lang>,
        cellphone_cert_yn: Option<bool>,
        authorized_cert_yn: Option<bool>,
        register_info: Option<RegisterKind>,
    ) -> Self {
        Self {
            response_type: "code".to_string(),
            client_id,
            redirect_uri,
            scope,
            client_info,
            state,
            auth_type,
            lang,
            cellphone_cert_yn,
            authorized_cert_yn,
            account_hold_auth_yn: None,
            register_info,
            accountinfo_yn: None,
            accountinfo_api_tran_id: None,
            accountinfo_list_num: None,
            client_device_type: None,
            client_device_ip: None,
            client_device_mac: None,
            client_device_id: None,
            client_device_num: None,
            client_device_version: None,
        }
    }
}

/// 사용자인증/서비스등록확인 API response body(callback url로 호출됨)
/// - code: 사용자인증 성공 시 반환되는 코드
/// - scope: Access Token 권한 범위 (다중 scope 가능)
/// - client_info: 요청 시 이용기관이 세팅한 client_info 값을 그대로 전달
/// - state: 요청 시 이용기관이 세팅한 state 값을 그대로 전달
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ResponseBody {
    rsp_code: ResponseCode,
    rsp_message: String,
    code: Option<String>,
    scope: Option<Vec<Scope>>,
    client_info: Option<String>,
    state: Option<String>,
}

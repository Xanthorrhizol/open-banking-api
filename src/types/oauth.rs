pub enum Scope {
    Login,
    Inquiry,
    Transfer,
    CardInfo,
    FintechInfo,
    InsuInfo,
    LoanInfo,
}

impl From<&str> for Scope {
    fn from(s: &str) -> Self {
        match s {
            "login" => Self::Login,
            "inquiry" => Self::Inquiry,
            "transfer" => Self::Transfer,
            "cardinfo" => Self::CardInfo,
            "fintechinfo" => Self::FintechInfo,
            "insuinfo" => Self::InsuInfo,
            "loaninfo" => Self::LoanInfo,
            _ => unreachable!(),
        }
    }
}

impl Into<&str> for Scope {
    fn into(self) -> &'static str {
        match self {
            Self::Login => "login",
            Self::Inquiry => "inquiry",
            Self::Transfer => "transfer",
            Self::CardInfo => "cardinfo",
            Self::FintechInfo => "fintechinfo",
            Self::InsuInfo => "insuinfo",
            Self::LoanInfo => "loaninfo",
        }
    }
}

#[repr(i32)]
pub enum AuthType {
    First = 0,
    Ignore = 2,
}

pub enum Lang {
    Kor,
    Eng,
}
impl Into<&str> for Lang {
    fn into(self) -> &'static str {
        match self {
            Self::Kor => "kor",
            Self::Eng => "eng",
        }
    }
}

pub enum RegisterKind {
    Account,   // 계좌
    Card,      // 카드(신용/체크)
    Pays,      // 선불
    Insurance, // 보험
    Loan,      // 캐피탈(대출/리스)
}

impl Into<&str> for RegisterKind {
    fn into(self) -> &'static str {
        match self {
            Self::Account => "A",
            Self::Card => "C",
            Self::Pays => "F",
            Self::Insurance => "I",
            Self::Loan => "L",
        }
    }
}

pub enum ClientDeviceType {
    PC,
    Andriod,
    Ios,
}

impl Into<&str> for ClientDeviceType {
    fn into(self) -> &'static str {
        match self {
            Self::PC => "PC",
            Self::Andriod => "AD",
            Self::Ios => "IO",
        }
    }
}

/// 인증생략 이용 시 이용하는 헤더
/// - user_seq_no(Kftc-Bfop-UserSeqNo): 기존 고객의 사용자일련번호
/// - user_connection_info(Kftc-Bfop-UserCI): 사용자 CI(Connection Info)
/// - access_token(Kftc-Bfop-AccessToken): "login" scope을 포함한 토큰
pub struct Header {
    user_seq_no: i32,
    user_connection_info: String,
    access_token: String,
}

/// OAuth2.0 공통 Body
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
pub struct Body {
    response_type: String,
    client_id: String,
    redirect_uri: String,
    scope: Vec<Scope>,
    client_info: Option<[u8; 256]>,
    state: [u8; 32],
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

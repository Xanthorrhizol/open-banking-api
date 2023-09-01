pub(crate) mod oauth;

#[cfg(feature = "prod")]
pub const ENDPOINT_URL: &'static str = "https://openapi.openbanking.or.kr";
#[cfg(feature = "test")]
pub const ENDPOINT_URL: &'static str = "https://testapi.openbanking.or.kr";

pub(crate) enum Endpoint {
    OAuth(OAuthEndpoint),     // 사용자인증(OAuth 2.0)
    User(UserEndpoint),       // 사용자/계좌관리
    Inquiry(InquiryEndpoint), // 조회서비스
}

pub(crate) enum OAuthEndpoint {
    Authorize,        // 사용자인증(GET)
    Token,            // 토큰발급(POST)
    Revoke,           // 토큰폐기(POST)
    AuthorizeAccount, // 서비스등록확인(POST)
}

pub(crate) enum UserEndpoint {
    UserInfo,         // 사용자정보조회(GET)
    Unlink,           // 사용자로그인연결동의해체(POST)
    ListAccount,      // 등록계좌조회(GET)
    UpdateAccount,    // 계좌정보변경(POST)
    CancelAccount,    // 계좌해지(POST)
    CancelCard,       // 카드조회해지(POST)
    CancelPays,       // 선불사용자조회해지(POST)
    CancelInsurances, // 보험사용자조회해지(POST)
    CancelLoans,      // 대출/리스사용자조회해지(POST)
    Quit,             // 사용자탈퇴(POST)
}

pub(crate) enum InquiryEndpoint {
    Balance,         // 잔액조회(GET)
    ListTransaction, // 거래내역조회(GET)
    RealName,        // 계좌실명조회(POST)
    RemitList,       // 송금인정보조회(POST)
    Receive,         // 수취조회(POST)
}

impl Into<String> for Endpoint {
    fn into(self) -> String {
        match self {
            Self::OAuth(inner) => inner.into(),
            Self::User(inner) => inner.into(),
            Self::Inquiry(inner) => inner.into(),
        }
    }
}

impl Into<String> for OAuthEndpoint {
    fn into(self) -> String {
        format!(
            "{}{}",
            ENDPOINT_URL,
            match self {
                Self::Authorize => "/oauth/2.0/authorize",
                Self::Token => "/oauth/2.0/token",
                Self::Revoke => "/oauth/2.0/revoke",
                Self::AuthorizeAccount => "/oauth/2.0/authorize_account",
            }
        )
    }
}

impl Into<String> for UserEndpoint {
    fn into(self) -> String {
        format!(
            "{}/v2.0{}",
            ENDPOINT_URL,
            match self {
                Self::UserInfo => "/user/me",
                Self::Unlink => "/user/unlink",
                Self::ListAccount => "/account/list",
                Self::UpdateAccount => "/account/update_info",
                Self::CancelAccount => "/account/cancel",
                Self::CancelCard => "/cards/cancel",
                Self::CancelPays => "/pays/cancel",
                Self::CancelInsurances => "/insurances/cancel",
                Self::CancelLoans => "/loans/cancel",
                Self::Quit => "/user/close",
            }
        )
    }
}

impl Into<String> for InquiryEndpoint {
    fn into(self) -> String {
        format!(
            "{}/v2.0{}",
            ENDPOINT_URL,
            match self {
                Self::Balance => "/account/balance/fin_num",
                Self::ListTransaction => "/account/transaction_list/fin_num",
                Self::RealName => "/inquiry/real_name",
                Self::RemitList => "/inquiry/remit_list",
                Self::Receive => "/inquiry/receive",
            }
        )
    }
}

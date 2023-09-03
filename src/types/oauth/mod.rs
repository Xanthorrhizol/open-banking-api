pub(crate) mod authorize;
pub(crate) mod token;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum ResponseCode {
    #[serde(rename = "O0000")]
    Success,
    #[serde(rename = "O0001")]
    ParameterError,
    #[serde(rename = "O0002")]
    AccessTokenDenied,
    #[serde(rename = "O0003")]
    AccessTokenExpired,
    #[serde(rename = "O0004")]
    AccessDenied,
    #[serde(rename = "O0005")]
    NotFound,
    #[serde(rename = "O0006")]
    IPDisallowed,
    #[serde(rename = "O0007")]
    NoRoute,
    #[serde(rename = "O0008")]
    AccountPlanLimitReached,
    #[serde(rename = "O0009")]
    ApiPlanLimitReached,
    #[serde(rename = "O0010")]
    WrongMethod,
    #[serde(rename = "O0011")]
    WrongScope,
    #[serde(rename = "O0012")]
    DenialOfService,
    #[serde(rename = "O0013")]
    TooMuchCall,
    #[serde(rename = "O0014")]
    RefreshTokenDenied,
    #[serde(rename = "O0015")]
    RefreshTokenExpired,
}

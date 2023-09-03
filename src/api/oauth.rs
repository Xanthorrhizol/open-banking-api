use crate::types::{endpoint::*, GetHash, HttpMethod};
use crate::ApiError;
use serde::{de::DeserializeOwned, Serialize};

pub struct OAuthApi {
    client: reqwest::Client,
}

impl OAuthApi {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn call<ReqHeader, ReqBody, ResBody>(
        &self,
        endpoint: Endpoint,
        method: HttpMethod,
        header: Option<ReqHeader>,
        body: Option<ReqBody>,
    ) -> Result<ResBody, ApiError>
    where
        ReqHeader: Serialize + GetHash,
        ReqBody: Serialize,
        ResBody: DeserializeOwned,
    {
        let result = match method {
            HttpMethod::Get => match header {
                None => {
                    self.client
                        .get(Into::<String>::into(endpoint))
                        .send()
                        .await?
                        .text()
                        .await?
                }
                Some(header) => {
                    let mut header_map = reqwest::header::HeaderMap::new();
                    for (&k, v) in header.get_hash().iter() {
                        header_map.insert(k, reqwest::header::HeaderValue::from_str(&v.clone())?);
                    }
                    self.client
                        .get(Into::<String>::into(endpoint))
                        .headers(header_map)
                        .send()
                        .await?
                        .text()
                        .await?
                }
            },
            HttpMethod::Post => {
                if body.is_some() {
                    match header {
                        None => {
                            self.client
                                .post(Into::<String>::into(endpoint))
                                .body(serde_json::to_string(&body.unwrap())?)
                                .send()
                                .await?
                                .text()
                                .await?
                        }
                        Some(header) => {
                            let mut header_map = reqwest::header::HeaderMap::new();
                            for (&k, v) in header.get_hash().iter() {
                                header_map
                                    .insert(k, reqwest::header::HeaderValue::from_str(&v.clone())?);
                            }
                            self.client
                                .post(Into::<String>::into(endpoint))
                                .headers(header_map)
                                .body(serde_json::to_string(&body.unwrap())?)
                                .send()
                                .await?
                                .text()
                                .await?
                        }
                    }
                } else {
                    match header {
                        None => {
                            self.client
                                .post(Into::<String>::into(endpoint))
                                .send()
                                .await?
                                .text()
                                .await?
                        }
                        Some(header) => {
                            let mut header_map = reqwest::header::HeaderMap::new();
                            for (&k, v) in header.get_hash().iter() {
                                header_map
                                    .insert(k, reqwest::header::HeaderValue::from_str(&v.clone())?);
                            }
                            self.client
                                .post(Into::<String>::into(endpoint))
                                .headers(header_map)
                                .send()
                                .await?
                                .text()
                                .await?
                        }
                    }
                }
            }
        };
        let parsed: ResBody = serde_json::from_str(&result)?;
        Ok(parsed)
    }
}

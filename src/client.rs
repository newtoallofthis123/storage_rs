use std::{collections::HashMap, str::FromStr};

use reqwest::header::{HeaderName, HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::{
    bucket::{Bucket, BucketCreateOptions},
    errors::{ResError, StorageError},
};

#[derive(Debug)]
pub struct StorageClient {
    pub base_url: reqwest::Url,
    pub token: String,
    req_client: reqwest::Client,
}

impl StorageClient {
    pub fn new(project_url: &str, token: &str) -> Result<Self, StorageError> {
        let url = match reqwest::Url::parse(project_url) {
            Ok(url) => url,
            Err(_) => return Err(StorageError::MalformerdProjectUrl(project_url.to_string())),
        };

        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(ACCEPT, HeaderValue::from_str("application/json").unwrap());
        default_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );
        default_headers.insert(
            HeaderName::from_str("apikey").unwrap(),
            HeaderValue::from_str(token).unwrap(),
        );
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .expect("Unable to parse to client");

        Ok(StorageClient {
            base_url: url,
            token: token.to_string(),
            req_client: client,
        })
    }

    pub async fn health_check(&self) -> Result<String, StorageError> {
        let url = self
            .base_url
            .join("/health")
            .expect("Unable to join the health url");

        let res = self
            .req_client
            .get(url)
            .send()
            .await
            .map_err(StorageError::ReqwestError)?;

        let text = res.text().await.map_err(|_| StorageError::TextParseError)?;
        Ok(text)
    }

    pub async fn metrics_check(&self) -> Result<bool, StorageError> {
        let url = self
            .base_url
            .join("metrics")
            .expect("Unable to join the metrics url");

        self.req_client
            .get(url)
            .send()
            .await
            .map_err(StorageError::ReqwestError)?;

        Ok(true)
    }

    pub async fn get_bucket(&self, bucket_name: &str) -> Result<Bucket, StorageError> {
        let url = self
            .base_url
            .join(&format!("bucket/{}", bucket_name))
            .expect("Unable to join the bucker URL");

        let res = self
            .req_client
            .get(url)
            .send()
            .await
            .map_err(StorageError::ReqwestError)?;

        if res.status() == 200 {
            Ok(res
                .json::<Bucket>()
                .await
                .map_err(StorageError::BucketParseError)?)
        } else {
            Err(StorageError::ResponseError(
                res.json::<ResError>()
                    .await
                    .map_err(StorageError::BucketParseError)?,
            ))
        }
    }

    pub async fn create_bucket(
        &self,
        bucket_options: BucketCreateOptions,
    ) -> Result<String, StorageError> {
        let options =
            serde_json::to_string(&bucket_options).map_err(StorageError::SerdeParseError)?;
        let url = self
            .base_url
            .join("bucket")
            .expect("Unable to join the bucker URL");

        let res = self
            .req_client
            .post(url)
            .body(options)
            .send()
            .await
            .map_err(StorageError::ReqwestError)?;

        if res.status() == 200 {
            Ok(format!("bucket: {} created!", bucket_options.name))
        } else {
            Err(StorageError::ResponseError(
                res.json::<ResError>()
                    .await
                    .map_err(StorageError::BucketParseError)?,
            ))
        }
    }
}

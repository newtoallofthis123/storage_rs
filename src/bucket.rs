use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub public: bool,
    pub file_size_limit: Option<u32>,
    pub allowed_mime_types: Option<Vec<String>>,
    pub owner: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct BucketCreateOptions {
    pub name: String,
    pub id: Option<String>,
    pub public: bool,
    pub file_size_limit: Option<u32>,
    pub allowed_mime_types: Option<Vec<String>>,
}

impl BucketCreateOptions {
    pub fn new(
        name: &str,
        public: bool,
        id: Option<String>,
        file_size_limit: Option<u32>,
        allowed_mime_types: Option<Vec<String>>,
    ) -> Self {
        BucketCreateOptions {
            name: name.to_string(),
            id,
            public,
            file_size_limit,
            allowed_mime_types,
        }
    }

    pub fn default(name: &str, public: bool) -> Self {
        BucketCreateOptions {
            name: name.to_string(),
            public,
            id: None,
            file_size_limit: None,
            allowed_mime_types: None,
        }
    }
}

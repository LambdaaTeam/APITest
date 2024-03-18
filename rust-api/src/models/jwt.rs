use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: String) -> Self {
        Self {
            sub,
            exp: 60 * 60 * 24 * 7, // 7 days
        }
    }
}

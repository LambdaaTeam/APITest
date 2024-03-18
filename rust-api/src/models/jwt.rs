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
            exp: chrono::Duration::try_hours(24 * 7)
                .unwrap()
                .num_seconds() as usize,
        }
    }
}

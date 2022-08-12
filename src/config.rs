use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
pub struct AppConfig {
    pub database_id: String,
    pub token: String,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub settings: DatabaseSettings,
    pub port: u16,
}


#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}
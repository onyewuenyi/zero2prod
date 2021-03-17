#[derive(serde::Deserialize)]
pub struct AppSettings {
    pub app_port: u16,
    pub database: DatabaseSettings 
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub uname: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String
}

impl DatabaseSettings {
    pub fn conn_str(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",self.uname, self.password, self.host, self.port, self.database_name)
    }
}
pub fn get_configuration() -> Result<AppSettings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
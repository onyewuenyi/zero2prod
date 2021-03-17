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
    // db name provided to conn to a logical DB
    pub fn conn_str(&self) -> String {
        format!("postgres://{}:{}@{}:{}/{}",self.uname, self.password, self.host, self.port, self.database_name)
    }
    
    // conn string w/o DB name. Db name excluded to enable you to connect to the Postgres instance instead of a logical DB
    pub fn conn_str_without_db(&self) -> String {
        format!("postgres://{}:{}@{}:{}",self.uname, self.password, self.host, self.port)
    }    
}
pub fn get_configuration() -> Result<AppSettings, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
use config;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_url: String,
    pub secret: Vec<u8>
}

impl Config {
    pub fn load(path: &str) -> Result<Config, String> {
        let mut cfg = config::Config::new();
        let _ = cfg.merge(config::File::new(path, config::FileFormat::Yaml))
            .or_else(|e| Err(format!("{}", e)));
        Config::parse_config(cfg)
    }

    fn parse_config(cfg: config::Config) -> Result<Config, String> {
        let host: String = cfg.get_str("http.host")
            .ok_or("Invalid or missing http.host key")?;
        let port: u16 = cfg.get_int("http.port")
            .ok_or("Invalid or missing http.port key")? as u16;
        let db_url: String = cfg.get_str("db.url")
            .ok_or("Invalid or missing db.url key")?;
        let secret: String = cfg.get_str("server.secret")
            .ok_or("Invalid or missing secret")?;
        Ok(Config {
               host: host,
               port: port,
               db_url: db_url,
               secret: secret.as_bytes().to_vec()
           })
    }
}

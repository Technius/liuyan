use config;

pub struct Config {
    pub host: String,
    pub port: u16,
    pub cors: Option<String>,
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
        fn get_key<T, F>(key: &str, f: F) -> Result<T, String>
            where F: Fn(&str) -> Option<T> {
            f(key).ok_or(format!("Invalid or missing key: {}", key))
        }
        let host: String = get_key("http.host", |k| cfg.get_str(k))?;
        let port: u16 = get_key("http.port", |k| cfg.get_int(k))? as u16;
        let db_url: String = get_key("db.url", |k| cfg.get_str(k))?;
        let secret: String = get_key("server.secret", |k| cfg.get_str(k))?;
        let cors = cfg.get_str("http.cors");
        Ok(Config {
               host: host,
               port: port,
               cors: cors,
               db_url: db_url,
               secret: secret.as_bytes().to_vec()
           })
    }
}

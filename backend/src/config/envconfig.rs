use envconfig::Envconfig;

// Envconfig is being used so that we can build config struct from .env file.

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(from = "PORT", default = "8090")]
    pub port: u16
}

impl Config {
    pub fn load() -> Self {
        return Config::init_from_env().expect("unable to load config");
    }
}
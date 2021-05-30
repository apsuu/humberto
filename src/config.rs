use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        let this = envy::from_env::<Self>()?;
        Ok(this)
    }
}

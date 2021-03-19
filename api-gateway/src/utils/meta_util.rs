#[derive(PartialEq)]
pub enum ENV {
    LOCAL,
    PRODUCTION,
}

pub struct MetaInfo {
    env: ENV,
}

impl ENV {
    pub fn from_string(env: &str) -> Self {
        match env {
            "local" => Self::LOCAL,
            "production" => Self::PRODUCTION,
            _ => panic!("Unknown environment"),
        }
    }
}

impl MetaInfo {
    pub fn new(env: ENV) -> Self {
        Self { env }
    }

    pub fn is_production(&self) -> bool {
        self.env == ENV::PRODUCTION
    }
}

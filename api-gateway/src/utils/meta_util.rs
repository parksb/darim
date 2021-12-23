#[derive(PartialEq)]
pub enum ENVIRONMENT {
    LOCAL,
    PRODUCTION,
}

pub struct MetaInfo {
    env: ENVIRONMENT,
}

impl ENVIRONMENT {
    pub fn from_string(env: &str) -> Self {
        match env {
            "local" => Self::LOCAL,
            "production" => Self::PRODUCTION,
            _ => panic!("Unknown environment"),
        }
    }
}

impl MetaInfo {
    pub fn new(env: ENVIRONMENT) -> Self {
        Self { env }
    }

    pub fn is_production(&self) -> bool {
        self.env == ENVIRONMENT::PRODUCTION
    }
}

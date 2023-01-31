use std::env;

#[derive(Clone)]
pub struct JWTConfig {
    pub public_key: String,
    pub private_key: String,
}

impl JWTConfig {
    pub fn init_from_env() -> Self {
        Self {
            public_key: env::var("APP_KEY_PUBLIC")
                .expect("You must set the APP_KEY_PUBLIC environment var!").replace("\r\n", ""),
            private_key: env::var("APP_KEY_PRIVATE")
                .expect("You must set the APP_KEY_PRIVATE environment var!").replace("\r\n", ""),
        }
    }
}

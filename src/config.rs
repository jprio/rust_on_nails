#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    // Initialise form our environment
    pub fn new() -> Config {
        let database_url = "postgresql://rio:!g*CbrtSU8SeGTs@free-tier7.aws-eu-west-1.cockroachlabs.cloud:26257/defaultdb?sslmode=verify-full&options=--cluster=lunar-tern-2155";
        #let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

        Config { database_url }
    }
}

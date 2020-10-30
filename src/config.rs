use std::fmt;

/// The config reads the environment variables and saves the values to configure the server
#[derive(Debug, Clone)]
pub struct Config {
    pub server_address: String,
    pub db_address: String,
    pub pool_limit: Option<u32>,
    pub api_key: String,
}

impl Config {
    //TODO: impl optional arguments!
    pub fn new() -> Self {
        let values = read_env();

        // extract the pool limit num
        // string -> u32 -> Option<u32>
        // TODO: the conversion looks a bit weird, maybe change it in the future
        let pool_limit_num = values[2].clone().parse().unwrap();
        let mut pool_limit: Option<u32> = None;
        if pool_limit_num != 0 {
            pool_limit = Some(pool_limit_num);
        }

        Self {
            server_address: values[0].clone(),
            db_address: values[1].clone(),
            pool_limit,
            api_key: values[3].clone()
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SERVER_ADDRESS: {} \n \
            DB_ADDRESS: {} \n \
            POOL_LIMIT: {:?} \n ",
            self.server_address, self.db_address, self.pool_limit
        )
    }
}

// Extracts the environment variables or supplies default values used in the docker-compose.yml if environment variables were not set
fn read_env() -> [String; 4] {
    // read all environment variables
    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
    let db_address = std::env::var("DB_ADDRESS")
        .unwrap_or_else(|_| "postgres://admin:admin@127.0.0.1:5432/smv".to_string());
    let pool_limit = std::env::var("POOL_LIMIT").unwrap_or_else(|_| "0".to_string());
    let api_key = dotenv::var("API_KEY").expect("Error: Api key not found");

    [server_address, db_address, pool_limit, api_key]
}

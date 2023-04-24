pub struct DbConfig {
    pub db_url: &'static str,
    pub db_username: &'static str,
    pub db_password: &'static str,
    pub db_ns: &'static str,
    pub db_name: &'static str,
}

impl DbConfig {
    pub fn read_from_env() -> Self {
        Self {
            db_url: dotenv!("SURREALDB_URL"),
            db_username: dotenv!("SURREALDB_USERNAME"),
            db_password: dotenv!("SURREALDB_PASSWORD"),
            db_ns: dotenv!("SURREALDB_NS"),
            db_name: dotenv!("SURREALDB_DATABASE"),
        }
    }
}

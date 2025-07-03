use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://app_user:supersecret@127.0.0.1:3306/actix_web").await
}

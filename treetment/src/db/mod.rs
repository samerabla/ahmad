use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

pub async fn insert_contact_request(
    pool: &PgPool,
    name: String,
    phone: String,
    email: String,
    service: String,
    message: String,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = sqlx::query_scalar!(
        r#"
        INSERT INTO contact_requests (name, phone, email, service, message)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
        name,
        if phone.is_empty() { None } else { Some(phone) },
        email,
        if service.is_empty() { None } else { Some(service) },
        if message.is_empty() { None } else { Some(message) },
    )
    .fetch_one(pool)
    .await?;

    Ok(id)
}

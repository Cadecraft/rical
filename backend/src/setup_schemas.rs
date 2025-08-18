use sqlx::{
    postgres::Postgres,
    Pool
};

pub async fn setup_schemas(pool: &Pool<Postgres>) {
    println!("Setting up schemas...");
    let mut transaction = pool.begin().await.unwrap();

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS account(
            account_id BIGSERIAL PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            hashed_password TEXT NOT NULL
        );
        "#).execute(&mut *transaction).await.expect("Failed to create account table");

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS task(
            account_id BIGINT references account(account_id),
            task_id BIGSERIAL PRIMARY KEY,
            year INTEGER NOT NULL,
            month INTEGER NOT NULL,
            day INTEGER NOT NULL,
            start_min INTEGER,
            end_min INTEGER,
            title TEXT NOT NULL,
            description TEXT,
            complete BOOLEAN NOT NULL,
            CHECK (month >= 0 AND month < 12),
            CHECK (day >= 0 AND day <= 31),
            CHECK (start_min IS NULL OR (start_min >= 0 AND start_min < 1440)),
            CHECK (end_min IS NULL OR (end_min >= 0 AND end_min < 1440)),
            CHECK (end_min IS NULL OR (end_min IS NOT NULL AND start_min IS NOT NULL))
        );
        "#).execute(&mut *transaction).await.expect("Failed to create task table");

    transaction.commit().await.unwrap();
    println!("Set up schemas");
}

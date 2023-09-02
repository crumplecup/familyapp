use anyhow::anyhow;
use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};
use shuttle_secrets::SecretStore;
use std::path::PathBuf;
use tower_http::services::ServeDir;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
    #[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    let secret = if let Some(pass) = secret_store.get("ADMIN_PASSWORD") {
        pass
    } else {
        return Err(anyhow!("Password was not found").into());
    };
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let router = Router::new().route("/", get(hello_world))
        .nest_service("/assets", ServeDir::new(static_folder));

    Ok(router.into())
}

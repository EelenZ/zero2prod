use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // We got rid of the double-wrapping using .app_data()
    .execute(connection.get_ref())
    .await
    .map_err(|e| {
        println!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}
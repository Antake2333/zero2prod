use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new subcriber.",
        %request_id,
        subscriber_email=%form.email,
        subscriber_name=%form.name,
    );
    let _request_span_guard = request_span.enter();
    tracing::info!(
        "request_id:{} - Adding '{}' '{}' as a new subscriber.",
        request_id,
        form.email,
        form.name
    );
    let query_span = tracing::info_span!("Saving new subsciber details in database");
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email, name, subscribed_at)VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id:{} - New subsciber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id:{} - Failed to save new subsciber details in database: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

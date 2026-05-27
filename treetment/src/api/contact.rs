use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct ContactPayload {
    pub name: String,
    pub phone: Option<String>,
    pub email: String,
    pub service: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ContactResponse {
    pub success: bool,
    pub message: String,
}

pub async fn contact_handler(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<ContactPayload>,
) -> Json<ContactResponse> {
    if payload.name.trim().is_empty() {
        return Json(ContactResponse {
            success: false,
            message: "Name ist erforderlich".to_string(),
        });
    }
    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return Json(ContactResponse {
            success: false,
            message: "Gültige E-Mail-Adresse erforderlich".to_string(),
        });
    }

    match crate::db::insert_contact_request(
        &pool,
        payload.name,
        payload.phone.unwrap_or_default(),
        payload.email,
        payload.service.unwrap_or_default(),
        payload.message.unwrap_or_default(),
    )
    .await
    {
        Ok(_) => Json(ContactResponse {
            success: true,
            message: "Vielen Dank! Wir melden uns innerhalb von 24 Stunden.".to_string(),
        }),
        Err(e) => {
            tracing::error!("DB error saving contact: {e}");
            Json(ContactResponse {
                success: false,
                message: "Fehler beim Speichern. Bitte versuchen Sie es erneut.".to_string(),
            })
        }
    }
}

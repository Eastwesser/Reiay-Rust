// Этот файл содержит мидлварь для обработки заголовков Accept-Language и установки текущего языка на основе запроса.

use axum::{
    extract::RequestParts,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::i18n::translate;
use std::sync::RwLock;

lazy_static::lazy_static! {
    static ref CURRENT_LANGUAGE: RwLock<String> = RwLock::new("en".to_string());
}

pub async fn locale_middleware<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let mut parts = RequestParts::new(req);

    if let Some(language) = parts.headers().and_then(|h| h.get("Accept-Language")) {
        let language = language.to_str().unwrap().split(',').next().unwrap();
        *CURRENT_LANGUAGE.write().unwrap() = language.to_string();
    }

    Ok(next.run(parts.reassemble()).await)
}

pub fn get_current_language() -> String {
    CURRENT_LANGUAGE.read().unwrap().clone()
}

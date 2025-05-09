pub mod home {
    pub async fn get() -> &'static str {
        "Hi"
    }
}

pub mod favicon {
    use axum::http::{HeaderName, header};

    pub async fn get() -> [(HeaderName, &'static str); 1] {
        [(header::CONTENT_TYPE, "image/x-icon")]
    }
}

use actix_web::Error;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;

pub async fn logger<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse<B>, Error> {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let rsp = next.call(req).await?;
    let status = rsp.status();
    tracing::info!("{} - {} {}", status, method, uri);
    Ok(rsp)
}

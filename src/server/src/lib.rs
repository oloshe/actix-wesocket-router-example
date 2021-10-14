use actix_web::{App, HttpRequest, HttpResponse, HttpServer, get, web};

use crate::ws_conn::WsConn;
#[macro_use]
mod codegen;
mod ws_conn;
mod cmd;
mod router;
mod handler;

/// 启动服务器
pub async fn create_app() {
    let (addr, port) = ("0.0.0.0", "8080");

    let _ = HttpServer::new(move || {
        App::new()
            .service(index)
    })
        .bind(format!("{}:{}", addr, port))
        .expect(&format!("Can't bind to port {}", port))
        .run()
        .await;
}

#[get("/ws/{nick}")]
async fn index(
    params:  web::Path<String>,
    req: HttpRequest,
	stream: web::Payload,
) -> HttpResponse {
    let conn = WsConn {
        nick: params.to_string(),
    };
    let resp = actix_web_actors::ws::start(conn, &req, stream);
    match resp {
		Ok(ret) => ret,
		Err(e) => e.error_response(),
	}
}
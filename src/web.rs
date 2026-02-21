//! Web server for browser-based Scheme REPL.

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::eval::{eval_str, make_global_env};
use crate::repl::load_stdlib;
use crate::types::{write_val, Val};

/// Maximum code length accepted by the /api/eval endpoint (100 KiB).
const MAX_CODE_BYTES: usize = 100 * 1024;

#[derive(Deserialize)]
struct EvalRequest {
    code: String,
}

#[derive(Serialize)]
struct EvalResponse {
    result: Option<String>,
    error: Option<String>,
    output: String,
}

/// Evaluate Scheme code via HTTP POST.
async fn eval_handler(req: web::Json<EvalRequest>) -> impl Responder {
    let code = req.code.trim().to_string();

    // Reject excessively large inputs to prevent DoS.
    if code.len() > MAX_CODE_BYTES {
        return HttpResponse::PayloadTooLarge().json(EvalResponse {
            result: None,
            error: Some(format!(
                "code too large: {} bytes (limit: {} bytes)",
                code.len(),
                MAX_CODE_BYTES
            )),
            output: String::new(),
        });
    }

    // Run evaluation in a blocking thread pool so the event loop is not stalled.
    let response = web::block(move || -> EvalResponse {
        let env = make_global_env();
        load_stdlib(&env);
        match eval_str(&code, &env) {
            Ok(Val::Void) => EvalResponse {
                result: None,
                error: None,
                output: String::new(),
            },
            Ok(val) => EvalResponse {
                result: Some(write_val(&val)),
                error: None,
                output: String::new(),
            },
            Err(e) => EvalResponse {
                result: None,
                error: Some(e.to_string()),
                output: String::new(),
            },
        }
    })
    .await;

    match response {
        Ok(resp) => HttpResponse::Ok().json(resp),
        Err(e) => HttpResponse::InternalServerError().json(EvalResponse {
            result: None,
            error: Some(format!("internal error: {e}")),
            output: String::new(),
        }),
    }
}

/// Health check endpoint.
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

/// Start the web server.
///
/// `bind_addr` defaults to `"0.0.0.0:<port>"` for production deployment.
/// Pass `"127.0.0.1:<port>"` to restrict to localhost during development.
pub async fn run_web_server(port: u16, bind_addr: Option<String>) -> std::io::Result<()> {
    let addr = bind_addr.unwrap_or_else(|| format!("0.0.0.0:{}", port));
    println!("Starting web server at http://{}", addr);

    HttpServer::new(move || {
        App::new()
            .route("/api/eval", web::post().to(eval_handler))
            .route("/health", web::get().to(health))
            .service(
                fs::Files::new("/", "./web/static")
                    .index_file("index.html")
                    .show_files_listing(),
            )
    })
    .bind(&addr)?
    .run()
    .await
}

//! Web server for browser-based Scheme REPL.

use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

use crate::eval::{eval_str, make_global_env};
use crate::repl::load_stdlib;
use crate::types::{write_val, Val};

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
    // Capture stdout
    let code = req.code.trim().to_string();

    // Each request gets a fresh evaluation context per call
    // In a real app you'd want session-based environments
    let env = make_global_env();
    load_stdlib(&env);

    let response = match eval_str(&code, &env) {
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
    };

    HttpResponse::Ok().json(response)
}

/// Health check endpoint.
async fn health() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

/// Start the web server.
pub async fn run_web_server(port: u16) -> std::io::Result<()> {
    let addr = format!("0.0.0.0:{}", port);
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

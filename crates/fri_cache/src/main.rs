use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;

use serde_json::{Value, json};

#[derive(Deserialize)]
struct SendFRIParams {
    public_input: String,
    bytecode_hash: String,
    payload: String,
}

#[derive(Deserialize)]
struct GetFRIParams {
    public_input: String,
    bytecode_hash: String,
}

#[derive(Serialize)]
struct StoredEntry {
    public_input: String,
    bytecode_hash: String,
    payload: String,
}

struct AppState {
    db: Arc<Db>,
}

fn make_key(public_input: &str, bytecode_hash: &str) -> String {
    format!("{}:{}", public_input, bytecode_hash)
}

#[post("/rpc")]
async fn rpc_handler(req: web::Json<Value>, state: web::Data<AppState>) -> impl Responder {
    let jsonrpc = req.get("jsonrpc").and_then(Value::as_str).unwrap_or("");
    let id = req.get("id").cloned().unwrap_or(json!(null));
    let method = req.get("method").and_then(Value::as_str).unwrap_or("");
    let params = req.get("params");

    if jsonrpc != "2.0" {
        let err = json!({"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid JSON-RPC version"},"id":id});
        return HttpResponse::BadRequest().json(err);
    }

    match method {
        "sendFRI" => {
            if let Some(p) = params {
                if let Ok(params) = serde_json::from_value::<SendFRIParams>(p.clone()) {
                    let key = make_key(&params.public_input, &params.bytecode_hash);
                    let _ = state.db.insert(key.as_bytes(), params.payload.as_bytes());
                    let _ = state.db.flush();
                    let res = json!({"jsonrpc":"2.0","result":"Stored","id":id});
                    HttpResponse::Ok().json(res)
                } else {
                    let err = json!({"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid params"},"id":id});
                    HttpResponse::BadRequest().json(err)
                }
            } else {
                let err = json!({"jsonrpc":"2.0","error":{"code":-32602,"message":"Missing params"},"id":id});
                HttpResponse::BadRequest().json(err)
            }
        }
        "getFRI" => {
            if let Some(p) = params {
                if let Ok(params) = serde_json::from_value::<GetFRIParams>(p.clone()) {
                    let key = make_key(&params.public_input, &params.bytecode_hash);
                    match state.db.get(key.as_bytes()) {
                        Ok(Some(value)) => {
                            let payload = String::from_utf8_lossy(&value).to_string();
                            let res = json!({"jsonrpc":"2.0","result":payload,"id":id});
                            HttpResponse::Ok().json(res)
                        }
                        _ => {
                            let err = json!({"jsonrpc":"2.0","error":{"code":-32001,"message":"Not found"},"id":id});
                            HttpResponse::Ok().json(err)
                        }
                    }
                } else {
                    let err = json!({"jsonrpc":"2.0","error":{"code":-32602,"message":"Invalid params"},"id":id});
                    HttpResponse::BadRequest().json(err)
                }
            } else {
                let err = json!({"jsonrpc":"2.0","error":{"code":-32602,"message":"Missing params"},"id":id});
                HttpResponse::BadRequest().json(err)
            }
        }
        _ => {
            let err = json!({"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":id});
            HttpResponse::BadRequest().json(err)
        }
    }
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> impl Responder {
    let mut entries = Vec::new();
    for kv in state.db.iter() {
        if let Ok((k, v)) = kv {
            if let (Ok(key_s), Ok(val_s)) =
                (String::from_utf8(k.to_vec()), String::from_utf8(v.to_vec()))
            {
                if let Some((pi, bh)) = key_s.split_once(':') {
                    entries.push(StoredEntry {
                        public_input: pi.to_string(),
                        bytecode_hash: bh.to_string(),
                        payload: val_s.to_string(),
                    });
                }
            }
        }
    }

    let mut html = String::from(
        "<html><head><title>FRI Store</title></head><body><h1>Stored FRI Payloads</h1><ul>",
    );
    for e in entries {
        html.push_str(&format!("<li><strong>public_input</strong>: {} | <strong>bytecode_hash</strong>: {}<br/><pre>{}</pre></li>",
            e.public_input, e.bytecode_hash, e.payload));
    }
    html.push_str("</ul></body></html>");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = sled::open("fri_db").expect("open sled");
    let state = web::Data::new(AppState { db: Arc::new(db) });

    println!("Starting server at http://127.0.0.1:8085");
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(rpc_handler)
            .service(index)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;

#[derive(Deserialize)]
struct SendFRIRequest {
    public_input: String,
    bytecode_hash: String,
    payload: String,
}

#[derive(Deserialize)]
struct GetFRIQuery {
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

#[post("/sendFRI")]
async fn send_fri(data: web::Json<SendFRIRequest>, state: web::Data<AppState>) -> impl Responder {
    let key = make_key(&data.public_input, &data.bytecode_hash);
    let _ = state.db.insert(key.as_bytes(), data.payload.as_bytes());
    let _ = state.db.flush();
    HttpResponse::Ok().body("Stored")
}

#[get("/getFRI")]
async fn get_fri(query: web::Query<GetFRIQuery>, state: web::Data<AppState>) -> impl Responder {
    let key = make_key(&query.public_input, &query.bytecode_hash);
    match state.db.get(key.as_bytes()) {
        Ok(Some(value)) => {
            let val = value.clone();
            let payload = String::from_utf8_lossy(&val).to_string();
            HttpResponse::Ok().body(payload)
        }
        _ => HttpResponse::NotFound().body("Not found"),
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
            .service(send_fri)
            .service(get_fri)
            .service(index)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}

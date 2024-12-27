use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_web::http::header;
use actix_cors::Cors;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use ::std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct News {
    id: u64,
    title: String,
    content: String,
    viral: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Database {
    news: HashMap<u64, News>,
}

impl Database {
    fn new() -> Self {
        Self {
            news: HashMap::new(),
        }
    }
    fn insert(&mut self, news: News) {
        self.news.insert(news.id, news);
    }
    fn get(&self, id: &u64) -> Option<&News> {
        self.news.get(id)
    }
    fn get_all(&self) -> Vec<&News> {
        self.news.values().collect()
    }
    fn remove(&mut self, id: &u64) -> Option<News> {
        self.news.remove(id)
    }
    fn update(&mut self, news: News) {
        self.news.insert(news.id, news);
    }
    fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    fn load_from_file() -> std::io::Result<Self> {
        let file = fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file)?;
        Ok(db)
    }
}

struct AppState {
    db: Mutex<Database>,
}

async fn create_news(app_state: web::Data<AppState>, news: web::Json<News>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(news.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Created().finish()
}

async fn read_news(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    match db.get(&id.into_inner()) {
        Some(news) => HttpResponse::Ok().json(news),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn update_news(app_state: web::Data<AppState>, news: web::Json<News>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.insert(news.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

async fn read_all_news(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let news = db.get_all();
    HttpResponse::Ok().json(news)
}

async fn delete_news(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db = app_state.db.lock().unwrap();
    db.remove(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = Database::load_from_file().unwrap_or_else(|_| Database::new());
    let data: web::Data<AppState> = web::Data::new(AppState {
        db: Mutex::new(db)
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .route("/news", web::post().to(create_news))
            .route("/news", web::get().to(read_all_news))
            .route("/news", web::put().to(update_news))
            .route("/news/{id}", web::get().to(read_news))
            .route("/news/{id}", web::delete().to(delete_news))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
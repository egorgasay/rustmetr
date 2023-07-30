//use std::env;
//use std::net::TcpListener;
//
//use animal_facts_api::run;
//
//#[actix_web::main]
//async fn main() -> std::io::Result<()> {
//    let environment_file;
//    if let Ok(e) = env::var("ENV") {
//        environment_file = format!(".env.{}", e);
//    } else {
//        environment_file = String::from(".env");
//    }
//
//    dotenv::from_filename(environment_file).ok();
//
//    let listener = TcpListener::bind("0.0.0.0:8888").expect("Failed to bind random port");
//    let database_name = dotenv::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
//
//    run(listener, &database_name)?.await
//}


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

// Структура для хранения данных
struct Storage {
    data: Mutex<HashMap<String, i32>>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            data: Mutex::new(HashMap::new()),
        }
    }

    fn get(&self, metric: String) -> Result<String, String> {
        match self.data.lock().unwrap().get(&metric.to_owned()) {
            Some(value) => Ok(value.to_string()),
            None => Err("metric not found".to_string()),
        }
    }

    fn set(&self, metric: String, value: i32) -> Result<String, String> {
        self.data.lock().unwrap().insert(metric, value);
        Ok("".to_string())
    }
}

struct UseCase {
    storage: Storage,
}

impl UseCase {
    fn new(st: Storage) -> Self {
        UseCase {
            storage: st,
        }
    }

    fn get_metric(&self, metric: String) -> Result<String, String> {
        match self.storage.get(metric) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }

    fn update_gauge(&self, metric: String, value: i32) -> Result<String, String> {
        match self.storage.set(metric, value) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}

// Контроллерные функции для обработки запросов
#[get("/get/{key}")]
async fn get_metric(logic: web::Data<UseCase>, k: web::Path<(String,)>) -> impl Responder {
    let kk = k.into_inner();
    match logic.get_metric(kk.0) {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

#[post("/update/gauge/{key}/{value}")]
async fn update_gauge(logic: web::Data<UseCase>, path: web::Path<(String,String)>) -> impl Responder {
    let p = path.into_inner();
    let key = p.0;
    let value: i32 = p.1.parse().unwrap();
    match logic.update_gauge(key, value) {
        Ok(value) => HttpResponse::Ok().body(value),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let storage = Storage::new();
    let logic = web::Data::new(UseCase::new(storage));

    HttpServer::new(move || {
        App::new()
            .app_data(logic.clone())
            .service(get_metric)
            .service(update_gauge)
    })
    .bind("127.0.0.1:8888")?
    .run()
    .await
}
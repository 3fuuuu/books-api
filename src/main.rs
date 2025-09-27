use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Clone)]
struct Book {
    id: usize,
    title: String,
    author: String,
}

impl Book {
    fn new(id: usize, title: &str, author: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            author: author.to_string(),
        }
    }
}

#[derive(Deserialize)]
struct NewBook {
    title: String,
    author: String,
}

struct AppState {
    books: Mutex<Vec<Book>>,
}

#[post("/books")]
async fn add_book(data: web::Json<NewBook>, state: web::Data<AppState>) -> impl Responder {
    let mut books = state.books.lock().unwrap();
    let book_id = books.len() + 1;

    let new_book = Book::new(book_id, &data.title, &data.author);
    books.push(new_book.clone());

    web::Json(serde_json::json!({
        "message": "本の登録に成功しました。",
        "book": new_book
    }))
}

#[get("/books")]
async fn list_books(state: web::Data<AppState>) -> impl Responder {
    let books = state.books.lock().unwrap();
    web::Json(books.clone()) 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        books: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(add_book)
            .service(list_books) 
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


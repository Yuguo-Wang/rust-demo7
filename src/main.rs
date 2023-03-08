use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[get("/users")]
async fn get_users() -> impl Responder {
    let user1 = User {
        id: 1,
        name: "John".to_string(),
        email: "john@example.com".to_string(),
    };

    let user2 = User {
        id: 2,
        name: "Jane".to_string(),
        email: "jane@example.com".to_string(),
    };

    let users = vec![user1, user2];

    HttpResponse::Ok().json(users)
}

#[derive(Serialize, Deserialize)]
struct NewUser {
    name: String,
    email: String,
}

#[post("/users")]
async fn create_user(user: web::Json<NewUser>) -> impl Responder {
    let user = User {
        id: 3, // generate a unique id here
        name: user.name.clone(),
        email: user.email.clone(),
    };

    HttpResponse::Created().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(create_user)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

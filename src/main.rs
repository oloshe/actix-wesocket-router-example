use server::{create_app};

#[actix_web::main]
async fn main() {
    create_app().await;
}

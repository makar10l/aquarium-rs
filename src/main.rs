use auth_service::auth;
pub mod auth_service;

fn main(){
    println!("Your name: {}", auth::authorize());
}

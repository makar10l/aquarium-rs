use std::io;

pub fn authorize() -> String{
    let mut nickname = String::new();

    println!("\t\tAuthorization:");

    io::stdin()
        .read_line(&mut nickname)
        .expect("error");
    nickname
}
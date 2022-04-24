use std::io;
use figlet_rs::FIGfont;
use rust::{self, Summary, Tweet};
fn main(){
    let shadow_font = FIGfont::from_file("resources/shadow.flf")
        .expect("not found");        
    let figure = shadow_font.convert("Dragon-script");
    assert!(figure.is_some());

    println!("{}", figure.unwrap());
    println!("");
    println!("1) Linux");
    println!("2) Windows");
    println!("Выберите опцию: ");
    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("not read");
    let option: i32 = option
        .trim()
        .parse()
        .expect("not read");   
    match option {  
        1 => linux(),
        2 => windows(),
        _ => main(),
    }
}
fn windows () {
    println!("Hello windows");
}
fn linux () {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
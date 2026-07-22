use std::io::stdin;

fn main() {
    println!("Whats your name?");
    let mut your_name: String = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failled to read line");
    println!("Hello, {}", your_name);
}

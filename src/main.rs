use std::io::stdin;


fn whats_your_name() -> String {
    let mut your_name: String = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failled to read line");
    return your_name
        .trim()
        .to_lowercase();
}

fn main() {
    println!("Whats your name?");
    let name = whats_your_name();
     println!("Hello, {:?}", name);
}

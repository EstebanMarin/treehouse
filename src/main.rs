use std::io::stdin;


fn whats_your_name() -> String {
    let mut your_name: String = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failled to read line");
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    let visitor_list: [&str;4] = ["esteban", "giselle", "jacobo", "julieta"];
    println!("Whats your name?");
    let name: String = whats_your_name();
    let mut allow_then_in: bool = false;
    for visitor in &visitor_list {
	if visitor == &name {
            allow_then_in = true;
	}
    }

    if allow_then_in {
	println!("Welcome to treehouse, {}", name);
    } else {
	println!("Sorry, you arent on the list");
    }
}


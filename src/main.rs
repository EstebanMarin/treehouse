use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
	Self {
	    name: name.to_lowercase(),
	    action,
	    age
	}
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn whats_your_name() -> String {
    let mut your_name: String = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failled to read line");
    your_name
        .trim()
        .to_lowercase()
}

fn basic_example() {
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

fn using_struct() {
    let jacobo= Visitor::new("jacobo", VisitorAction::Accept, 9);
    let giselle = Visitor::new("giselle", VisitorAction::AcceptWithNote { note: String::from("Loves music") }, 42 );
    let julieta = Visitor::new("julieta", VisitorAction::Accept, 6);
    let esteban = Visitor::new("esteban", VisitorAction::Probation, 39);
    println!("{:#?}", jacobo);
    let visitor_list = [
	jacobo,
	giselle,
	julieta,
	esteban,
    ];
    println!("Whats your name?");
    let name: String = whats_your_name();
    let know_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    match know_visitor {
	Some(visitor) =>
	    println!("{}", visitor.name),
	    None => println!("Shooo")
    }
}

fn main() {
    basic_example();
    using_struct();
}


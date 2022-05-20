use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name:&str, greeting:&str ) -> Self{
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }

    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting)
    }
}

fn what_is_your_name() -> String {
    println!("Hello , what's your name ?");
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("failed to read your name");
    your_name
        .trim()
        .to_lowercase()
}    

fn main() {
    // bismi llah
    let name = what_is_your_name();
    println!("your name is {:?}", name);
    let visitors_list = [
        Visitor::new("bert", "Hello bert  , enjoy your treehouse"),
        
        Visitor::new("steve", "Hello steve enjoy your treehouse"),
        
        Visitor::new("fred", "Wow , who invited you"),
    ];


    let know_visitor = visitors_list
        .iter()
        .find(|visitor|  visitor.name == name);

    match know_visitor {
        Some(visitor) => visitor.greet_visitor(),
        None => println!("you are not in the visitors list")
    }

}
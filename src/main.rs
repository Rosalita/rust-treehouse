use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    // impl implements methods for a struct, it is followed the name of the struct to implement.
    // methods can access the struct contents. Associated functions, can't.

    // new is an associated function that is a constructor as it returns Self.
    fn new(name: &str, greeting: &str) -> Self {
        // Self (with capital) refers to struct type.
        Self {
            name: name.to_lowercase(), // to_lowercase() and to_string() convert str to String.
            greeting: greeting.to_string(),
        } // lack of semi-colon here is an implicit return.
    }
    fn greet_visitor(&self) {
        // &self as a parameter means the method has access to the struct contents.
        println!("{}", self.greeting); // self (lowercase) refers to the instance of the struct, not its type.
    }
}

fn main() {
    println!("Hello, what's your name?");
    let name = what_is_your_name();
    println!("Hello {}", name);
    println!("{:?}", name); // this is a debug print, the {} place holder has been change to the debug placeholder

    // let visitor_list = ["bert", "steve", "fred"]; // this is an array of str (string literals)
    // str and String are different types. str are strings entered in code and generally unchanging.
    // String is a dynamic type that stores location, length, capacity and can be appended to and edited.

    // The array has now changed to an array of Visitor structs rather than string literals.
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    let mut allow_them_in = false;

    // When the array was of str this was enough to search for a name.

    // for visitor in &visitor_list {
    //    if visitor == &name {
    //        allow_them_in = true;
    //    }
    // }

    // Now it is an array of struct, need to search it with iterators.
    // Iterators can do a lot, they are designed around function chaining.
    // Each iterator step works as a building block to massage the data from the previous step into what you need.
    // iterators are very fast, often faster than writing loops as the compiler can be certain you arent
    // doing anything dangerous like trying to read beyond the end of an array so it can make many optimisations.

    let known_visitor = visitor_list
        .iter() // create an iterator that contains all the data in visitor_list
        .find(|visitor| visitor.name == name); // find runs a closure. If the statement is true, it returns the matching value.
                                               // Closures are used a lot on Rust. Closures capture data from the scope in which they are called.
                                               // the matching values are stored in known_visitor.
                                               // known_visitor is of type Option because it might contain a visitor or it might not.
                                               // Options are enums that have two possible values Some(x) and None.
                                               // There are lots of ways to interact with options, but for now can use match().

    match known_visitor { // match is given an option
        Some(visitor) => visitor.greet_visitor(), // for some a fat arrow => denotes the code to execute if there is some match 
        None => println!("You are not on the visitor list. Please leave"), // None executes => if the option has no data.
    }

    if allow_them_in {
        println!("Welcome to the treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list");
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    // function chaining
    stdin() // returns an object granting access to the standard input
        .read_line(&mut your_name) // read_line is a method on the stdin object.
        // &mut borrow the variable allowing changes to be made by the function
        .expect("failed to readline"); // expect will unwrap a result object and terminate with the message if there is an error

    // pre-fixing a variable with & creates a reference to the variable.
    // A reference passes access to the variable itself, not a copy.
    // this is called borrowing, the variable is lended to the function.
    // lending with &mut permits the borrowing function to mutate the variable.

    your_name // lines that dont end in ; are returns.
        .trim()
        .to_lowercase()
}

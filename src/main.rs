use std::io::stdin;

// The debug placeholders {:?} for raw printing, and {:#?} for pretty printing
// can be used on any type that supports the Debug trait.
// The Debug trait is added with a derive attribute.
// Deriving requires that every member field in the structure supports the feature being derived.
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    // impl implements functions for a struct, it is followed the name of the struct to implement.
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
    // let visitor_list = ["bert", "steve", "fred"]; // this is an array of str (string literals)
    // str and String are different types. str are strings entered in code and generally unchanging.
    // String is a dynamic type that stores location, length, capacity and can be appended to and edited.

    // The array has now changed to an array of Visitor structs rather than string literals.
    // let visitor_list = [
    //     Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
    //     Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
    //     Visitor::new("fred", "Wow, who invited Fred?"),
    // ];

    // Arrays can't grow beyond their original size, but vectors can. They have a method named push(), their size is limited by memory.
    // instead of turning people away, we want to save their names, so will need to use Vector instead of Array.
    // Rusts vectors are similar to arrays, so its relatively easy to replace one with the other.
    // the vec! macro lets you initialise a vector with similar syntax to array initialisation.

    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    // This could also be expressed as
    // let mut visitor_list = Vec::new();
    // visitor_list.push(
    // Visitor::new("bert", "Hello Bert, enjoy your treehouse.")
    //);

    // however doing this is longer and more unwieldly, so using the macro is the better choice.
    // Vector is a generic type, it's declared as Vec<T> where the T is subsituted for the type specified or inferred.

    loop {
        // this is a loop that runs until it breaks.
        // it will break if there is no input.
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();
        println!("Hello {}", name);
        println!("{:?}", name); // this is a debug print, the {} place holder has been change to the debug placeholder

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

        // note that the following iterator code supports a visitor_list is of type array and of type vector.

        let known_visitor = visitor_list
            .iter() // create an iterator that contains all the data in visitor_list
            .find(|visitor| visitor.name == name); // find runs a closure. If the statement is true, it returns the matching value.
                                                   // Closures are used a lot on Rust. Closures capture data from the scope in which they are called.
                                                   // the matching values are stored in known_visitor.
                                                   // known_visitor is of type Option because it might contain a visitor or it might not.
                                                   // Options are enums that have two possible values Some(x) and None.
                                                   // There are lots of ways to interact with options, but for now can use match().

        match known_visitor {
            // match is given an option
            Some(visitor) => visitor.greet_visitor(), // for some a fat arrow => denotes the code to execute if there is some match
            None => {
                // None executes => if the option has no data.
                if name.is_empty() {
                    // is_empty is a method implemented by String. It returns true if the string is empty, otherwise is false.
                    // is_empty is more efficient than checking name.len() == 0, which also works.
                    break; // break immediately jumps to the end of the loop.
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }
    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
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

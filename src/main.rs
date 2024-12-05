use std::future::pending;

fn main() {
    println!("Hello, Rust!");

    //Variables
    //Note - All variables are immutable here.
    let x: i8 = 5;  //i here means signed integer and if its u8 then it is an unsigned integer.
    let y: u8 = 3;

    //for float
    let a: f32 = 45.34;

    //To print all
    print!("x: {}\n", x);
    print!("y: {}\n", y);
    print!("a: {}\n", a);
    print!("x: {}, y: {}, a: {}\n", x,y,a);

    //Booleans
    let is_male = false;
    let above_18 = true;

    if is_male {
        print!("You are a male.\n");
    } else {
        print!("You are a Female.\n");
    }

    if is_male && above_18 {
        print!("You are a male and above 18.");
    }


    //Strings
    let greeting= String::from("Hello RS.");
    println!("{}", greeting);
    

    //For some Advance stuff in Rust viw russt official docs and Pattern Matching


    //Conditionals and Loops and functions
    for i in 0..11 {
        print!("{} ", i);
    }

    let sentence= String::from("Hello Om!");
    let first_word = get_first_word(sentence);
    println!("First Word is: {}", first_word);

    //Simple iterations
    let x: i8 = 51;
    for i in 0..x {
        println!("{}", i);
    }

}

fn get_first_word(sentence: String) -> String {  //The arrow indicates the return type
    let mut ans: String = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {  //Dont use double quotes here as it is a char and not a string
            break;
        }
    }
    return ans;
}

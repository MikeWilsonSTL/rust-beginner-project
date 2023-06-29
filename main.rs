//Things to study later: variable shadowing
use std::io;

fn main() {
    println!("Hello world!");
    // immutability by default
    // the initialized value is the **only** value
    let _non_mutable = 5;
    // mutable variable defined by the use of 'mut' keyword
    // the initial value can be changed to something else
    let mut mutable = 6;
    println!("The value of mutable before changing is: {}", mutable);
    mutable = 7;
    println!("The value of mutable after changing is: {}", mutable);

    // Create an immutable variable
    let x = 10;
    println!("The value of x is: {}", x);

    // Create a mutable variable
    let mut y = 5;
    println!("The value of y before modification is: {}", y);

    // Modify the mutable variable
    y = 7;
    println!("The value of y after modification is: {}", y);

    { // this block won't compile
        // let a;
        // println!("{}", a); // error on this line
        // reading the value of an **uninitialized** variable is a compile-time error
    }

    { // this block will compile
        let a;
        a = 128;
        println!("{}", a); // no error here
        // variable 'a' has an initial value
    }

    println!();
    println!("Swap two numbers without a third variable:");
    println!();
    let mut a = 101010;
    let mut b = 4599584;
    println!("Variable A = {}", a);
    println!("Variable B = {}", b);
    println!("");
    println!("SWAP!");
    println!("");
    a = a + b;
    b = a - b;
    a = a - b;
    println!("Variable A = {}", a);
    println!("Variable B = {}", b);

    println!("");
    println!("Character Counter");
    let string = "This is a test string. We out here.";
    let mut counter = 0;
    for _char in string.chars() {
        counter = counter + 1;
    }
    println!("'{}' has {} characters", string, counter);

    println!("Enter any string");
    let mut user_string = String::new();
    let mut _user_counter = -1;

    io::stdin()
        .read_line(&mut user_string)
        .expect("Failed to read input");

    for _char in user_string.chars() {
        _user_counter += 1;
    }

    println!("'{}' has {} characters", user_string.trim(), _user_counter);
    println!("");

    println!("Variable shadowing and physical memory locations:");
    println!("");
    let a = 108;
    println!("addr of a: {:p}, value of a: {a}", &a);
    let a = 56;
    println!("addr of a: {:p}, value of a: {a} // post shadowing", &a);

    let mut b = 82;
    println!("\naddr of b: {:p}, value of b: {b}", &b);
    let mut b = 120;
    println!("addr of b: {:p}, value of b: {b} // post shadowing", &b);

    let mut c = 18;
    println!("\naddr of c: {:p}, value of c: {c}", &c);
    c = 29;
    println!("addr of c: {:p}, value of c: {c} // post shadowing", &c);
    println!("");
    println!("Arrays and Tuples");
    println!("Arrays:");
    println!("");
    // without type annotation
    let greeting = ['H', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd', '!'];

    // with type annotation
    let pi: [i32; 10] = [1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    for character in greeting {
        print!("{}", character);
    }

    println!("\nPi: 3.1{}{}{}{}", pi[0], pi[1], pi[2], pi[3]);

    let a = [10; 15];

    for i in a {
        print!("{i} ");
    }
    println!("");
    println!("Tuples:");
    println!("");
    let a = (38, 923.329, true);
    let b: (char, i32, f64, bool) = ('r', 43, 3.14, false);

    println!("a.0: {}, a.1: {}, a.2: {}", a.0, a.1, a.2);
    println!("b.0: {}, b.1: {}, b.2: {}, b.3: {}", b.0, b.1, b.2, b.3);

    // destructuring a tuple
    let pixel = (50, 0, 200);
    let (red, green, blue) = pixel;
    println!("red: {}, green: {}, blue: {}", red, green, blue);

    let mut variable = 5;
    let variable_reference = &variable;
    variable = 0;
    println!("{}", variable_reference);


}

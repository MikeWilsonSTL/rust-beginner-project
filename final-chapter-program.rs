use std::io;

const APPLE_RETAIL_PER_KG: f64 = 60.0;
const APPLE_WHOLESALE_PER_KG: f64 = 45.0;

const BANANA_RETAIL_PER_KG: f64 = 20.0;
const BANANA_WHOLESALE_PER_KG: f64 = 15.0;

const ORANGE_RETAIL_PER_KG: f64 = 100.0;
const ORANGE_WHOLESALE_PER_KG: f64 = 80.0;

const MANGO_RETAIL_PER_KG: f64 = 60.0;
const MANGO_WHOLESALE_PER_KG: f64 = 55.0;

const GRAPES_RETAIL_PER_KG: f64 = 120.0;
const GRAPES_WHOLESALE_PER_KG: f64 = 100.0;

fn main() {
    println!("Welcome to the fruit mart!");
    println!("Please select a fruit to buy.\n");

    let mut total: f64 = 0.0;
    let valid_inputs = ["apple", "banana", "orange", "mango", "grapes", "quit", "q"];

    'mart: loop {
        let mut user_input = String::new();
        let mut quantity = String::new();

        println!("\nAvailable fruits to buy: Apple, Banana, Orange, Mango, Grapes");
        println!("Once you are done purchasing, type in 'quit' or 'q'.\n");

        // get user input
        io::stdin()
            .read_line(&mut user_input)
            .expect("Unable to read user input.");
        user_input = user_input.trim().to_lowercase();

        // validate user input
        let mut input_error = true;
        for input in valid_inputs {
            if input == user_input {
                input_error = false;
                break;
            }
        }

        // handle invalid input
        if input_error {
            println!("ERROR: please enter a valid input");
            continue 'mart;
        }

        // quit if user wants to
        if user_input == "q" || user_input == "quit" {
            break 'mart;
        }

        // get quantity
        println!(
            "\nYou choose to buy \"{}\". Please enter the quantity in Kilograms.
(Quantity of 1Kg 500g should be entered as '1.5'.)",
            user_input
        );
        io::stdin()
            .read_line(&mut quantity)
            .expect("Unable to read user input.");
        let quantity: f64 = quantity
            .trim()
            .parse()
            .expect("Please enter a valid quantity.");

        total += calc_price(quantity, user_input);
    }

    println!("\n\nYour total is {} Rupees.", total);
}

fn calc_price(quantity: f64, fruit: String) -> f64 {
    if fruit == "apple" {
        price_apple(quantity)
    } else if fruit == "banana" {
        price_banana(quantity)
    } else if fruit == "orange" {
        price_orange(quantity)
    } else if fruit == "mango" {
        price_mango(quantity)
    } else {
        price_grapes(quantity)
    }
}

fn price_apple(quantity: f64) -> f64 {
    if quantity > 7.0 {
        quantity * APPLE_WHOLESALE_PER_KG
    } else {
        quantity * APPLE_RETAIL_PER_KG
    }
}

fn price_banana(quantity: f64) -> f64 {
    if quantity > 4.0 {
        quantity * BANANA_WHOLESALE_PER_KG
    } else {
        quantity * BANANA_RETAIL_PER_KG
    }
}

fn price_orange(quantity: f64) -> f64 {
    if quantity > 3.5 {
        quantity * ORANGE_WHOLESALE_PER_KG
    } else {
        quantity * ORANGE_RETAIL_PER_KG
    }
}

fn price_mango(quantity: f64) -> f64 {
    if quantity > 5.0 {
        quantity * MANGO_WHOLESALE_PER_KG
    } else {
        quantity * MANGO_RETAIL_PER_KG
    }
}

fn price_grapes(quantity: f64) -> f64 {
    if quantity > 2.0 {
        quantity * GRAPES_WHOLESALE_PER_KG
    } else {
        quantity * GRAPES_RETAIL_PER_KG
    }
}

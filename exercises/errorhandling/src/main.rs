use std::{fs, io::Error};

//Error handling when there is a result to send back
fn divide(dividend: f64, divisor: f64) -> Result<f64, Error> {
    if divisor == 0.0 {
        return Err(Error::other("You cannot divide by zero"));
    } else {
        return Ok(dividend / divisor);
    }
}

//Error handling when there is no result to send back
fn check_email_is_valid(email: String) -> Result<(), Error> {
    if email.contains("a") {
        return Ok(());
    } else {
        return Err(Error::other(
            "The email you have input does not contain '@'",
        ));
    }
}

fn main() {
    //Error handling when a success has a sensible thing to return
    let quotient = divide(10.0, 0.0);

    match quotient {
        Ok(value) => {
            println!("This is the result {}", value);
        }
        Err(value) => {
            println!("Err: {}", value);
        }
    }

    //Error handling when the success does not have anything sensible to return
    let email_validity = check_email_is_valid(String::from("lesliedouglas23@gmail.com"));

    match email_validity {
        //you would see ".." used to state that what is returned in the ok is useless
        Ok(..) => println!("Email is valid"),
        Err(error) => println!("Error: {}", error),
    }

    let text = fs::read_to_string("logs.txt");

    match text {
        Ok(t) => println!("{}", t),
        Err(e) => println!("Error: {}", e),
    }

    // println!("{:#?}", text);
}

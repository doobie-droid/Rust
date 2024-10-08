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

fn create_list_of_errors(error: &str) -> Vec<String> {
    let mut string_collection: Vec<String> = vec![];
    let split_strings: std::str::Split<'_, &str> = error.split("\n");
    for sentence in split_strings {
        if sentence.starts_with("ERROR") {
            string_collection.push(sentence.to_string());
        }
    }
    return string_collection;
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

    let text: Result<String, Error> = fs::read_to_string("logs.txt");

    //1.  First way of handling errors using the whole match match way
    match text {
        Ok(value) => {
            let error_list: Vec<String> = create_list_of_errors(&value);
            match fs::write("error_logs.txt", error_list.join("\n")) {
                Ok(..) => println!("The error contents have been written"),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => {
            println!("Error: {}", e)
        }
    }

    //2. Second way of handling errors
    // if the content of what you are dealing with returns a result, you can handle it with an
    // expect, unwrap or unwrap_or
    let second_text: String =
        fs::read_to_string("logs.txt").expect("The file you want to open does not exist");
    let second_error_list: Vec<String> = create_list_of_errors(&second_text);
    fs::write("second_error_logs.txt", second_error_list.join("\n"))
        .expect("There was an error in writing to the file");

    // 3rd way of handling errors
    main1().expect("It could not handle this function");

   
}

fn main1() -> Result<(), Error> {
    //3. Third way of handling errors
    // We use question marks on things or items that could return result enums
    // if an error, is returned, the program panics there and returns the error
    // if not, it gets to the end and returns the ok
    let text = fs::read_to_string("logs.txt")?;
    let third_error_list = create_list_of_errors(&text);
    fs::write("third_error_logs.txt",third_error_list.join("\n") )?;
    Ok(())

}

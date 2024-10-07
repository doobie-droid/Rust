use std::io::Error;

// Todo: add in a return type
fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error> {
    if ingredients.len() > 3 {
        return Err(Error::other("You should not have more than 3 ingredients"));
        // Todo: make it clear that this is an error!
    } else {
        return Ok(());
        // Todo: make it clear that the ingredidents passed
        // validation. Note that we don't really have any value
        // to include in the 'Ok' here...
    }
}

fn main() {
    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        String::from("Olives"),
    ];

    // Todo: validation is an operation that might succeed or fail
    // Print out a success or fail message based on whether
    // it passes validation
    match validate_ingredients(&ingredients) {
        Ok(..) => {
            println!("The validation was successful");
        }
        Err(value) => {
            println!("Error: {}", value)
        }
    }
}

fn main() {
    //Did you know that you can just create infinite loops in rust by simply
    // using the loop command ... it automatically goes forever and ever

    //you can name the outer loop
    'outer: loop {
        println!("Enter the outer loop");

        //you can name the inner loop
        'inner: loop {
            println!("Entered the inner loop");

            //You can select the specific loop
            // that you want to break out of
            break 'outer;
        }
    }
    readable_reference_in_loop();
    move_variable_in_loop();
    mutable_reference_in_loop();
    understanding_references();

     let array = [1, -2, 6];

     //in the example below, you are dealing with an array of primitive values so when you assign a new variable containing any of the primitive value, that primitive value would be copied
     match array {
        // in the example below, you have binded the first element to the first variable
        // in the example below, you have binded all the elements in the middle to the "middle" variable
        // in the example below, you have binded the last element to the 'last' element in the array

        
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

fn readable_reference_in_loop() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
}

fn move_variable_in_loop() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}

fn mutable_reference_in_loop() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}

fn understanding_references() {

    let integer_value = 5;
    //There are 2 ways of CREATING A REFERENCE  in rust, you can:

    //1. Use the ref keyword
    let ref reference1 = 4;
    //2. You can also use the & stuff like other programming languages do
    let reference = &3;

    
    // There are 2 ways of DEREFERENCING A REFERENCE IN rust, you can:

    // 1. you can dereference a binding by putting "*" in front of the reference.
    let dereferenced_number = *reference;
    println!("The number is {}", dereferenced_number);
    // 2. You can dereference by putting the ampersand in front of the variable
    let &dereferenced_number1 = reference;
    println!("The number is {}", dereferenced_number1);
    //
    

    // depending on where &  or &mut is , it could mean referencing or de-referencing, 
    // the ref  or ref mut keyword always creates a reference though
    // the * keyword always dereferences though 
}

fn matching_numbers_in_guards() {
    let number: i8 = 4;

    //when using enums, you have to match all possible enums
    // when matching numbers, you must account for all possible scenarios, 
    // so that being said, you should have an underscore to account for things that you did not consider
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        i if i < 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen."),
        // TODO ^ uncomment to fix compilation
    }
}


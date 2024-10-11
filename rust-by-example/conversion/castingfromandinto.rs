use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// The From trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types. 
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// The Into trait is simply the reciprocal of the From trait. It defines how to convert a type into another type.
impl Into<Number> for i64 {
    fn into(self) -> Number {
    
        Number { value: self as i32 }
    }
}


fn main() {
    let number_from = Number::from(30);
    // the code below would still work because there is a From method for i32, the into method checks if there is a from if no into is found
    // however, the from method does not check for an into
    let into_number: Number = 32.into();
    println!("My number is {:?}", into_number);
    let into_number: Number = 32i64.into();
    println!("My number is {:?}", number_from);
    println!("My number is {:?}", into_number);
}
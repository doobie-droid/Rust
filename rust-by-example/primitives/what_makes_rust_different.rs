use std::mem;

#[allow(unused_variables)]
fn main() {
    // BINDINGS
    // Variables can be type annotated by declaring the type annotation in front of
    // or BEHIND THE variable in question

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    // Variables are immutable by default and as such are called bindings instead of variables
    // to make a variable mutable, you have to use the mut keyword

    let mut inferred_type = 12; // Type i64 is inferred from another line

    // A variable can be redeclared to a different type in a process known as shadowing
    // What is shadowing and when should we use it
    let mutable = true;

    // BITWISE OPERATORS
    // Bitwise operations are present and are used frequently for systems programming
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    //TUPLES
    // You can insert any datatype in a tuple including another tuple
    // and rust would not complain
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // tuples are printable until the elements they have exceed 12,
    // at that point, they are no longer printable

    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("Too long tuple: {:?}", too_long_tuple);

    // The tuple sample above cannot be printed like other bindings are

    // To create a tuple with one element, add a comma to the end of the
    // tuple to differentiate it from a regular ass binding
    println!("One element tuple: {:?}", (5u32,));

    //DESTRUCTURING
    // You can destructure in rust by doing what you would have done when you were assigning the variable, in reverse
    // let matrix = Matrix(2.3, 3.4, 2, 6i32);
    // let Matrix(a, b, c, d) = matrix;


    //ARRAYS
    // You can initialize an array and assign values to it immediately
    let ys: [i32; 5] = [4; 5];
    //The code sample would create an array with 5 elements which are all the letter 4

    //Arrays are allocated on the stack so you can have a max with 
    // about 1 million elements because the stack can have a max of 8 mb on it and there would be some other memory set aside for other stack stuff
    // the array  has 5 elements which all occupy 4 bytes each for a total of 20 bytes
    println!("Array occupies {} bytes", mem::size_of_val(&ys));


    // you can get a slice of strings, vector and arrays by simply referencing them
    // a slice of vectors and a slice of arrays look exactly the same when defining them in function argument

  
    // you can use .get() to safely get an element from an array, it would return some if it exists and none if it does not exist
    match ys.get(5) {
        Some(val) => println!("value was gotten"),
        None => println!("value was not gotten"),
    }


    // All elements can be initialized to the same value.
}

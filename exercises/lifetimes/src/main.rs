//Rules around lifetimes

//1. Each parameter that is a reference gets its own lifetime parameter
//2. If there is exactly one input lifetime parameter, that lifetime is assigned to all the output lifetime parameters... because all the output lifetimes definitely have to come from that one input
//3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

//Lifetimes explained
//Notice how string1 and string2 have different lifetimes in the function
// below,
// String1 is active for the entire function and string2 is active for only the inner block.
// Assuming there is a function that takes multiple references as parameters and returns a reference, how is rust supposed to know the lifetime of what is returned
// when you mark up the function like it was done in longest, you are instructing rust to check between x and y and make sure that the returned string slice has the same lifetime as the shortest of the two
// For this reason, the lifetime of the solution does not extend beyond
// the code block because the shorter of the 2 strings, string2 has its lifetime only existing in that code block
// check main 2 for how the function reacts when the lifetimes of the 2 variables are in the outer block
fn main() {
    let string1: String = String::from("abcd");
    {
        let string2: String = String::from("inner");
        //solution takes the lifetime of the shorter variable
        let solution = longest(&string1, &string2);
        println!("The solution is {}", solution)
    }

    main_when_they_both_have_long_lifetimes();
    main_when_they_both_have_short_lifetimes();
    println!("Hello, world!");
}

// since the 2 strings are initialized in the outer code block,
// their lifetimes exist for that code block and the returned string also has a lifetime that exists for the outer code block
fn main_when_they_both_have_long_lifetimes() {
    let string1: String = String::from("abcd");
    let string2: String = String::from("inner");
    let result: &str;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result);

    println!("Hello, world!");
}

// the code below would fail compilation because the string1 and string2
// do not live long enough to be used in the result variable which has a lifetime that exceeds the both of them
fn main_when_they_both_have_short_lifetimes() {
    {
        let string1: String = String::from("abcd");
        let string2: String = String::from("inner");
        let solution = longest(&string1, &string2);
        println!("The longest string is {}", solution);
    }

    println!("Hello, world!");
}
// the code below would fail compilation because the string1 and string2
// do not live long enough to be used in the result variable which has a lifetime that exceeds the both of them
// fn main_when_they_both_have_short_lifetimes_but_wont_compile() {
//     let result: &str;
//     {
//         let string1: String = String::from("abcd");
//         let string2: String = String::from("inner");
//         result = longest(&string1, &string2);
//     }
//     println!("The longest string is {}", result);

//     println!("Hello, world!");
// }

//Taking closures as input parameters
fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}
fn main() {
    

    let greeting = "hello";
    let mut farewell = greeting.to_owned();

    let once = || {
        //If this line was the only line in the closure, you could declare the closure as a Fn
        println!("The greeting is referenced {} ", greeting);

        // Mutation has been done, this closure has to be declared as an FnMut at least at this point
        farewell.push_str(" !!!");
        println!("The greeting has been mutated {}", farewell);

        // Dropping the variable has happened here, this requires ownership of the variable. This closure
        // needs to be an FnOnce at this point
        std::mem::drop(farewell);

        //Note that if you say you are expecting a FnOnce closure, and the closure you pass only does 
        // referencing, the code will compile as expected because FnOnce implies ownership, so of course
        // the parent function can reference values
    };

    apply(once);

    //Since the three_closure only references the value, you can easily use it in the
    //apply_to_3 function
    let three_closure = |x: i32| x * 2;
    apply_to_3(three_closure);
}

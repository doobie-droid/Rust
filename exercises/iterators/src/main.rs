fn print_elements(colors: &[String], function_name: &str) {
    println!("The function name is: {}", function_name);
    colors
        .iter()
        .for_each(|color: &String| println!("Color: {}", color));
}

fn shorten_strings(colors: &mut [String]) {
    colors
        .iter_mut()
        .map(|color| color.truncate(1))
        .for_each(|color| color);
}

//Note that when you use a vector slice [String]... you can pass full vectors
// and still pass vector slices as parameters to the functions
fn to_upper(colors: Vec<String>) -> Vec<String> {
    // realize that changing strings to uppercase is literally changing the underlying string there
    colors
        .into_iter()
        .map(|color| color.to_uppercase())
        .collect()
}

fn move_element(vector_a: Vec<String>, vector_b: &mut Vec<String>) {
    vector_a
        .into_iter()
        .for_each(|element| vector_b.push(element));
}

fn explode(colors: &[String]) -> Vec<Vec<String>> {
    colors
        .iter()
        .map(|color| color.chars().map(|char| char.to_string()).collect())
        .collect()
}

fn find_color_or(haystack: &[String], needle: &str, fallback: &str) -> String {
    match haystack.iter().find(|element| element.contains(needle)) {
        Some(value) => {
            return String::from(value);
        }
        None => {
            return String::from(fallback);
        }
    }
}
fn main() {
    let colors: Vec<String> = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elements(&colors, "PRINT");

    let mut colors1: Vec<String> = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    shorten_strings(&mut colors1);
    print_elements(&colors1, "SHORTEN");

    let colors2: Vec<String> = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let new_colors2 = to_upper(colors2);
    print_elements(&new_colors2, "TO UPPER");

    let colors3: Vec<String> = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let mut destination_colors: Vec<String> = vec![];
    move_element(colors3, &mut destination_colors);
    print_elements(&destination_colors, "MOVE");

    let colors4: Vec<String> = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    let exploded_colors = explode(&colors4);
    exploded_colors
        .iter()
        .for_each(|element| print_elements(&element, "EXPLODE"));

    let colors5: Vec<String> = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    println!("Hello, world!");
    println!("The color is {}", find_color_or(&colors5, "re", "Orange"));
}

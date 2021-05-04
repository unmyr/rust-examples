fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = first_str(string1.as_str(), string2);
    println!("The first string is {}", result);
}

fn first_str(x: &str, y: &str) -> &str{
    x
}
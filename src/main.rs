fn main() {
    // Prompt the user to enter a number
    //cvevievev
    println!("Enter a number:");

    // Read the number from the user
    let mut num = String::new();
    std::io::stdin().read_line(&mut num)
        .expect("Failed to read line");

    // Parse the number into a u32 value
    let num: u32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number");
            return;
        },
    };

    // Generate the multiplication table for the given number
    for i in 1..=10 {
        println!("{} x {} = {}", num, i, num * i);
    }
}
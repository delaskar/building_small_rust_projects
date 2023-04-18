use std::io;

// prompt user
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Create a zodiac signs function
fn generate_zodiac_signs() -> Vec<&'static str> {
    vec![
        "Aries", "Tauro", "Geminis", "Cacncer", "Leo", "Virgo", "Libra", "Escorpio", "Sagitario",
        "Capricornio", "Acuario", "Piscis",
    ]
}


fn main() {
    // Call the function generate_zodiac_signs
    let zodiac_signs = generate_zodiac_signs();

    loop {
        // Username:
        let user_name = get_user_input("Enter your name please:");

        // Birth month:
        let month = get_user_input("Write your birth month:");

        // Convert month brithday to real number:
        let month: i32 = month.parse().expect("Please type a month number.");

        // Birthday
        let day = get_user_input("Enter your birthday please:");

        // Convert user bitrthday to real number
        let day: i32 = day.parse().expect("Please type a number");

        // Match Expression
        let zodiac_sign: &str = match (day, month) {
            (21..=31, 3) | (1..=19, 4) => zodiac_signs[0],
            (20..=30, 4) | (1..=20, 5) => zodiac_signs[1],
            (21..=31, 5) | (1..=20, 6) => zodiac_signs[2],
            (21..=30, 6) | (1..=22, 7) => zodiac_signs[3],
            (23..=31, 7) | (1..=22, 8) => zodiac_signs[4],
            (23..=31, 8) | (1..=22, 9) => zodiac_signs[5],
            (23..=30, 9) | (1..=22, 10) => zodiac_signs[6],
            (23..=31, 10) | (1..=21, 11) => zodiac_signs[7],
            (22..=30, 11) | (1..=21, 12) => zodiac_signs[8],
            (22..=31, 12) | (1..=19, 1) => zodiac_signs[9],
            (20..=31, 1) | (1..=18, 2) => zodiac_signs[10],
            (19..=29, 2) | (1..=20, 3) => zodiac_signs[11],
            _ => "Unknown",
        };

        // Print the zodiac sign
        println!("{}, your zodiac sign is: {}", user_name, zodiac_sign);
        println!("");

        // another sign ?
        println!("Do you want to see another zodiac sign? Type 'y' or 'n':");
        let mut another_sign: String = String::new();
        io::stdin()
            .read_line(&mut another_sign)
            .expect("Failed to read your answer!");

        // Remove white space
        let another_sign: &str = another_sign.trim();

        if another_sign == "y" {
            println!("");
        } else if another_sign == "n" {
            println!("");
            println!("Thanks for your query");
            break;
        } else {
            println!("Wrong type. Try again!");
            println!("");
            break;
        }
    }
}

use std::io;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u32,
    civil_status: CivilStatus,
    account: bool,
}

#[derive(PartialEq, Debug)]
enum CivilStatus {
    Married,
    Single,
    Divorced,
    Widowed,
}

fn data_collection(name: &str, email: &str, age: u32, civil_status: CivilStatus) -> User {
    User {
        name: String::from(name),
        email: String::from(email),
        age,
        civil_status,
        account: true
    }
}

fn main() {
    println!("Hello, Budy!");
    println!("Answer the following questions for this survey:");

    let mut all_data_collected: Vec<User> = Vec::new();

    loop {
        let mut data_user: Vec<String> = Vec::new();
        
        for question in 1..=4 {
            println!("Questio number ({})", question);
            if question == 1 {
                println!("What is your name?: ");
                data_user.push(prompts_user_text());
            } else if question == 2 {
                println!("What is your email?: ");
                data_user.push(prompts_user_text());
            } else if question == 3 {
                println!("How old are you?: ");
                data_user.push(prompts_user_number().to_string());
            } else if question == 4 {
                println!("What is your civil_status: ");
                data_user.push(prompts_user_text());
            }
        }

        let status = match data_user[3].to_lowercase().as_str() {
            "married" => CivilStatus::Married,
            "single" => CivilStatus::Single,
            "divorced" => CivilStatus::Divorced,
            "widowed" => CivilStatus::Widowed,
            _ => panic!("Invalid civil status")
        };
        
        let data: User = data_collection(&data_user[0], &data_user[1], data_user[2].parse().unwrap(), status);

        println!("");
        println!("Your name is: {:?}", data.name);
        println!("Your email is: {:?}", data.email);
        println!("Your age is: {:?}", data.age);
        println!("Your civil status is: {:?}", data.civil_status);
        println!("Your mode account is: {:?}", data.account);

        all_data_collected.push(data);

        println!("");
        println!("Are there more people to survey? Write 'y' or off: ");
        let decision = prompts_user_text();
        if decision.to_lowercase() == "off" {
            break;
        }
    }
    write_data_to_file(&all_data_collected, "users.txt").expect("Failed to write to file");
}

fn prompts_user_text() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let trimmed_input = input.trim();
    if !trimmed_input.is_empty() {
        trimmed_input.to_string()
    } else {
        String::from("Input cannot be empty")
    }
}

fn prompts_user_number() -> u32 {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line. Enter a number.");
    input.trim().parse().unwrap()
}

fn write_data_to_file(data: &[User], file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    for user in data {
        writeln!(
            file,
            "Name: {}\nEmail: {}\nAge: {}\nCivil status: {:?}\nAccount: {}\n",
            user.name, user.email, user.age, user.civil_status, user.account
        )?;
    }
    Ok(())
}

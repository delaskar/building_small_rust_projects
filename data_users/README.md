# **Data Users**

This Rust survey project collects data from users through a series of questions and saves the responses to a file. The project includes basic input/output functionality and data structuring.

## **Requirements**

- Rust compiler
- Cargo package manager

## **Installation**
1. Clone the repository: 
    - `git clone https://github.com/exampleuser/data_users.git`

2. Navigate to the project directory: `cd data_users`
3. Build the project: `cargo build`
4. Run the project: `cargo run`

## **Usage**

When the project is run, the user is prompted to answer a series of questions. The questions are as follows:

- What is your name?
- What is your email?
- How old are you?
- What is your civil status?

After answering the questions, the user's data is collected and displayed on the console. The user is then prompted to continue with the survey for additional users or to end the survey.

At the end of the survey, the collected data is saved to a text file named users.txt in the same directory as the project.

## **Code Overview**

The code defines two structs, User and CivilStatus, which are used to represent the collected data. The data_collection function takes in user input and creates a new User struct. The prompts_user_text and prompts_user_number functions handle user input for text and numerical data, respectively.

The main function handles the overall program flow, calling the various functions to collect and save user data. The collected data is stored in a vector of User structs, which is then passed to the write_data_to_file function to save the data to a file.

## **Future Improvements**

Possible future improvements to the project include:

- Adding validation for user input to ensure that data is entered correctly
- Adding support for different file formats for saving the collected data
- Adding more detailed output to the console and file for easier analysis of collected data.

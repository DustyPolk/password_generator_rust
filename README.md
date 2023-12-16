#### Overview

This project is a Rust-based application designed to generate secure passwords. It consists of three main components: the main application logic, a password generator module (`password_generator.rs`), and a utility module (`utils.rs`). The application allows users to specify the length of the password and whether to include special characters and uppercase letters.

#### Main Application File

-   Functionality: The main file is responsible for interfacing with the user. It prompts users for their preferences regarding the password to be generated and displays the result or any errors.

-   Key Functions:

    1.  `main`: Initializes the application, handles user input, and orchestrates password generation.
    2.  `prompt_for_input`: Prompts the user for input and returns the input as a string.
    3.  `prompt_for_usize`: Continuously prompts the user until a valid positive integer is provided.
    4.  `prompt_for_yes_no`: Asks the user a yes/no question and returns a boolean based on the answer.
-   External Libraries:

    -   `colored`: Used for colored output to the console.
    -   `std::io`: Standard input/output handling in Rust.

#### Password Generator Module (`src/password_generator.rs`)

-   Purpose: This module contains the core logic for generating the password based on the specified criteria.

-   Function `generate_password`:

    -   Inputs:
        1.  `length`: The desired length of the password.
        2.  `use_special_chars`: Boolean indicating whether to include special characters.
        3.  `use_uppercase`: Boolean indicating whether to include uppercase letters.
    -   Output: Returns a `Result<String, &'static str>`, which is either the generated password or an error message.
    -   Logic: Constructs a character pool based on the user's choices and randomly selects characters to form the password.
-   External Libraries:

    -   `rand`: Used for generating random numbers.

#### Utility Module (`src/utils.rs`)

-   Description: This module defines constants used across the application, specifically character sets.

-   Constants:

    -   `LOWERCASE_LETTERS`: Contains all lowercase alphabet characters.
    -   `UPPERCASE_LETTERS`: Contains all uppercase alphabet characters.
    -   `NUMERALS`: Contains digit characters from 0 to 9.
    -   `SPECIAL_CHARACTERS`: Contains various special characters for password complexity.

#### Compilation and Execution

To compile and run this project, ensure you have Rust installed. Use `cargo run` in the project directory. The program will compile and then start, prompting the user for input.

#### Additional Notes

-   The program emphasizes user input validation and error handling, ensuring robustness.
-   Styling and user experience are enhanced through colored console output.
-   Consideration for future improvement: Implement a configuration file for customizable character sets and default settings.

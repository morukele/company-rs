mod company;
mod employee;

use company::Company;
use employee::Employee;
use std::io::{self, Write};

fn main() {
    let mut company = Company::new();

    println!("Welcome, please input the name of employee and department you want to add.");

    // Initalize a loop to handle user input
    loop {
        // Print cursor
        print!("> ");
        io::stdout().flush().unwrap();

        // Read input from the terminal
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");

        // Trim whitespace from the input
        let input = input.trim();

        // Check if the command is quit, then break loop
        if input == "exit" {
            break;
        }

        // split the input into a vector of words
        let words: Vec<&str> = input.split_whitespace().collect();

        // Handle the user input with match
        match words[0] {
            "Add" => {
                let name = words[1].to_string();
                let department = words[3].to_string();
                let employee = Employee { name, department };
                company.add_employee(employee);
            }
            "Get" => {
                let employee_by_department = company.get_employee_by_department();

                for (department, employees) in employee_by_department {
                    println!("{}: {}", department, employees.join(", "));
                }
            }
            _ => println!("Unrecognized input, please try again."),
        }

        println!("Thank you for using the system");
    }
}

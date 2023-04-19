use crate::employee::Employee;
use std::collections::HashMap;

pub struct Company {
    /*
    First string holds department name
    Vector holds names employees in the department
    */
    pub register: HashMap<String, Vec<String>>,
}

impl Company {
    // A function to create a new company
    pub fn new() -> Self {
        Company {
            register: HashMap::new(),
        }
    }

    /*
    A function to add an employee to the company
    */
    pub fn add_employee(&mut self, employee: Employee) {
        // get the department associated with employee
        let department = self
            .register
            .entry(employee.department.clone())
            .or_insert(Vec::new());

        // Add employee to the department
        department.push(employee.name.clone());

        // Display success message
        println!(
            "Added {} to {} department",
            employee.name, employee.department
        )
    }

    /*
    A function to return the list of employees and department
    sorted alphabetically
    The output is a vector with a string for department and
    another vector of strings to contain employee name
    */
    pub fn get_employee_by_department(&self) -> Vec<(String, Vec<String>)> {
        // Create a new vector to hold the result
        let mut result = Vec::new();

        // Iterate through the company register to get the department
        for (department, employees) in &self.register {
            // Add department and employees to the result
            result.push((department.clone(), employees.clone()));
        }

        // sort the result according to department name
        result.sort_by(|a, b| a.0.cmp(&b.0));

        result
    }
}

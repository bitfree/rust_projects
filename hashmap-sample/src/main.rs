use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter a command:");
        println!("1. Add [Name] to [Department]");
        println!("2. List all employees alphabetically");
        println!("3. List all employees by department");
        println!("4. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();

        if input == "4" {
            break;
        } else if input.starts_with("Add ") && input.contains(" to ") {
            add_employee(&mut company, input);
        } else if input == "2" {
            list_all_employees_alphabetically(&company);
        } else if input == "3" {
            list_employees_by_department(&company);
        } else {
            println!("Invalid command. Please try again.");
        }
    }
}

fn add_employee(company: &mut HashMap<String, Vec<String>>, input: &str) {
    // Split input to extract name and department
    if let Some((name_part, department)) = input[4..].split_once(" to ") {
        let name = name_part.trim().to_string();
        let department = department.trim().to_string();

        company
            .entry(department.clone())
            .or_insert_with(Vec::new)
            .push(name.clone());

        println!("Added {} to {}", name, department);
    } else {
        println!("Invalid input format.");
    }
}

fn list_all_employees_alphabetically(company: &HashMap<String, Vec<String>>) {
    let mut all_employees: Vec<String> = company
        .values()
        .flat_map(|employees| employees.clone())
        .collect();

    all_employees.sort();

    if all_employees.is_empty() {
        println!("No employees found.");
    } else {
        println!("All employees (sorted alphabetically):");
        for employee in all_employees {
            println!("{}", employee);
        }
    }
}

fn list_employees_by_department(company: &HashMap<String, Vec<String>>) {
    if company.is_empty() {
        println!("No departments or employees found.");
    } else {
        for (department, employees) in company {
            println!("\nDepartment: {}", department);
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            for employee in sorted_employees {
                println!("- {}", employee);
            }
        }
    }
}

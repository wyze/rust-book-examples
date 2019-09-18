use std::collections::HashMap;
use std::io;

type Directory = HashMap<String, Vec<String>>;

pub fn run() {
    println!("Create your company directory");

    let mut directory: Directory = HashMap::new();

    loop {
        show_help();

        println!("Enter your selection:");

        let input = get_input();

        match input.chars().nth(0) {
            Some('a') => add_employee(&mut directory),
            Some('c') => list_company(&directory),
            Some('d') => list_department(&directory),
            Some('q') => break,
            Some(_) | None => continue,
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("An error occurred while reading the input: {}", error);
    }

    input
}

fn show_help() {
    println!();
    println!("a\tAdd an employee");
    println!("d\tList employees in a department");
    println!("c\tList employees in the company");
    println!("q\tQuit the program");
}

fn add_employee(directory: &mut Directory) {
    println!("Use the following format: Add <name> to <department>");
    println!("E.g. Add Neil to Engineering");

    let input = get_input();
    let tokens: Vec<&str> = input.split_whitespace().collect();

    match &tokens[..] {
        ["Add", employee, "to", department] => {
            if employee.is_empty() {
                println!("Employee name cannot be empty.");

                return;
            }

            if department.is_empty() {
                println!("Department name cannot be empty.");

                return;
            }

            directory
                .entry(department.to_string())
                .or_insert_with(|| vec![])
                .push(employee.to_string());
        }
        _ => println!("Not enough or correct arguments!"),
    };
}

fn list_department(directory: &Directory) {
    println!("Which department would you like to see?");

    let mut departments: Vec<_> = directory.keys().collect();

    departments.sort();

    for (index, department) in departments.iter().enumerate() {
        println!("\t{}. {}", index + 1, department);
    }

    let input = get_input();
    let index = match input.trim().parse::<usize>() {
        Ok(num) if num <= departments.len() => num - 1,
        Ok(_) | Err(_) => departments.len(),
    };

    if index < departments.len() {
        let department = *departments.get(index).unwrap();
        let employees = directory.get(department).unwrap();

        print_department((department, employees));
    } else {
        println!("Department not found.");
    }
}

fn print_department((department, employees): (&String, &Vec<String>)) {
    println!("\nEmployees in {}:", department);

    let mut sorted_employees = employees.clone();

    sorted_employees.sort();

    for employee in sorted_employees {
        println!("\t{}", employee);
    }
}

fn list_company(directory: &Directory) {
    directory.iter().for_each(print_department);
}

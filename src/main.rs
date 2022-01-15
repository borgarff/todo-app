use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use std::string::ToString;

fn main() {
    let mut input = String::new();
    let mut todo_list: Vec<String> = read_from_file();

    println!("Todo terminal application written in Rust");
    meny();

    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line\n");

    while input.to_uppercase().trim() != "Q" {
        match input.to_uppercase().trim() {
            "N" => new_task(&mut todo_list),
            "D" => delete_task(&mut todo_list),
            "L" => read_list(&mut todo_list),
            "U" => update_task(&mut todo_list),
            _=> meny(),
        }
        print!("\n");
        input = "".to_string(); // Resets the string
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line\n");
    }
    write_to_file(&mut todo_list);
}

// What the different keypresses do
fn meny() {
    println!("
Press N for new task\n
Press D to delete or complete a task\n
Press L to list all tasks\n
Press U to update a task\n
Press Q to quit the program\n");
}

// Returns an Iterator that can iterate thrue all lines in file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Returns all the tasks from tasks.txt in a list  
fn read_from_file() -> Vec<String>{
    let mut list: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines("./tasks.txt") {
        for line in lines {
            if let Ok(ip) = line {
                list.push(ip.trim().to_string());
            }
        }
    }
    return list;
}

// Writes a list to a file
fn write_to_file(list: &mut Vec<String>) {
    let path = Path::new("tasks.txt");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Writes the the todo-list to the tasks.txt file
    for line in list {
        file.write(line.as_bytes())
        .expect("Failed to write line");
        file.write("\n".as_bytes())
        .expect("Failed to create new line");
    }
}

// List out all tasks in the todo-list
fn read_list(list: &mut Vec<String>) {
    let mut nr = 1;
    for element in list {
        print!("{}", nr);
        println!(" {}", element);
        nr += 1;
    }
}

// Adds a new task to the todo-list
fn new_task(list: &mut Vec<String>) {
    let mut task = String::new();

    println!("Name the new task: "); 
    io::stdin()
        .read_line(&mut task)
        .expect("Failed to read line");

    if list.contains(&task.trim().to_string()) { 
        println!("The task {:?} allready exist", task.trim());
    }
    else {
        list.push(task.trim().to_string());
    }
}

// Removes a task from the todo-list if it exist
fn remove_task(list: &mut Vec<String>, task: String) -> bool {
    // Checks if task exists and removes it
    if list.contains(&task.trim().to_string()) { 
        list.retain(|x| task.trim() != x);
        return true;
    }
    println!("This task do not exist");
    return false;
}

// Deletes a task if it exists 
fn delete_task(list: &mut Vec<String>) {
    let mut task = String::new();
    println!("Name of the task you want to delete: ");
    
    io::stdin()
    .read_line(&mut task)
    .expect("Failed to read line");

    if remove_task(list, task) {
        println!("The task is deleted from the list");
    }
}

// Updates a task in the todo-list
fn update_task(list: &mut Vec<String>) {
    let mut task = String::new();
    println!("Name of the task you want to update: ");

    io::stdin()
    .read_line(&mut task)
    .expect("Failed to read line");

    if remove_task(list, task) {
        println!("Give new name to task");
        new_task(list);
    }
}   
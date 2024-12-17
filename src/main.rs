use std::process::Command;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// Francescos Work
const FILENAME: &str = "todos.json";
fn get_todos() -> Vec<Todo> {
    let todos = std::fs::read_to_string(FILENAME).unwrap_or("".to_string());
    let todos: Value = serde_json::from_str(&todos).unwrap_or(Value::Null);

    let mut todos_list = Vec::new();

    if todos.is_array() {
        for todo in todos.as_array().unwrap() {
            let todo = todo.as_object().unwrap();
            let id = todo.get("id").unwrap().as_u64().unwrap() as u32;
            let title = todo.get("title").unwrap().as_str().unwrap().to_string();
            let completed = todo.get("completed").unwrap().as_bool().unwrap();

            todos_list.push(Todo { id, title, completed });
        }
    }

    todos_list
}

fn list_all_todos() {
    let todos = get_todos();

    if todos.is_empty() {
        println!("No todos found!");
    } else {
        for todo in todos {
            println!("{} [{}] {}", todo.id, if todo.completed { "X" } else { " " }, todo.title);
        }
    }

    pause_before_main_menu();
}

fn add_new_todo() {
    println!("Enter the title of the new todo: ");
    let title = get_input();

    let todos = get_todos();
    let id = if todos.is_empty() {
        1
    } else {
        todos.last().unwrap().id + 1
    };

    let new_todo = Todo { id, title, completed: false };

    let mut todos = get_todos();
    todos.push(new_todo);

    let todos = serde_json::to_string(&todos).unwrap();
    std::fs::write(FILENAME, todos).unwrap();

    println!("Todo added successfully!");

    pause_before_main_menu();
}

// End of Francescos Work

// Morgans Work

fn mark_todo_as_done() {
    println!("Enter the id of the todo you want to mark as done: ");
    let id: u32 = get_input().parse().unwrap();

    let mut todos = get_todos();
    let todo = todos.iter_mut().find(|todo| todo.id == id);

    match todo {
        Some(todo) => {
            todo.completed = true;

            let todos = serde_json::to_string(&todos).unwrap();
            std::fs::write(FILENAME, todos).unwrap();

            println!("Todo marked as done successfully!");
        }
        None => {
            println!("Todo not found!");
        }
    }

    pause_before_main_menu();
}

fn delete_todo() {
    println!("Enter the id of the todo you want to delete: ");
    let id: u32 = get_input().parse().unwrap();

    let mut todos = get_todos();
    let initial_length = todos.len();
    todos.retain(|todo| todo.id != id);

    if todos.len() < initial_length {
        let todos_json = serde_json::to_string(&todos).unwrap();
        std::fs::write(FILENAME, todos_json).unwrap();
        println!("Todo deleted successfully!");
    } else {
        println!("Todo not found!");
    }

    pause_before_main_menu();
}

fn delete_completed_todos() {
    let mut todos = get_todos();
    let initial_length = todos.len();
    todos.retain(|todo| !todo.completed);

    if todos.len() < initial_length {
        let todos_json = serde_json::to_string(&todos).unwrap();
        std::fs::write(FILENAME, todos_json).unwrap();
        println!("Completed todos deleted successfully!");
    } else {
        println!("No completed todos found!");
    }

    pause_before_main_menu();
}

// End of Morgans Work

// Cadens Work

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn pause_before_main_menu() {
    println!("Press Enter to go back to main menu...");
    get_input();
    main_menu();
}

fn clear_console() {
    Command::new("clear").status().unwrap();
}

fn exit() {
    println!("Goodbye!");
    std::process::exit(0);
}

// End of Cadens Work

// Morgan/Francesco/Caden's Work

fn main_menu() {
    clear_console();
    println!("Welcome User!");
    println!("1) LIST ALL TODOS");
    println!("2) ADD NEW TODO");
    println!("3) MARK TODO AS DONE");
    println!("4) DELETE TODO");
    println!("5) DELETE COMPLETED TODOS");
    println!("6) EXIT");
    println!("Enter your choice: ");

    let choice = get_input();

    clear_console();

    match choice.as_str() {
        "1" => list_all_todos(),
        "2" => add_new_todo(),
        "3" => mark_todo_as_done(),
        "4" => delete_todo(),
        "5" => delete_completed_todos(),
        "6" => exit(),
        _ => {
            println!("Invalid choice! Please try again.");
            main_menu();
        }
    }
}

fn main() {
    main_menu();
}

// End of Morgan/Francesco/Caden's Work
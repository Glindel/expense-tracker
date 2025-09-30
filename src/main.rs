mod models;
mod expense_repository;

use crate::models::{Action, ActionError};

use comfy_table::{Table};
use std::{env, vec};

fn main() {
   let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a action")
    }

    let action = match map_action(&args) {
        Ok(a) => a,
        Err(error) => panic!("{error}")
    };

    match action {
        Action::Add {description, amount} => {
            match expense_repository::create_expense(description, amount) {
                Ok(_) => {}
                Err(error) => { println!("{error}") }
            }
        },
        Action::List { month } => show_expense_list(month),
        Action::Summary => show_summary(),
        Action::Delete { id } => delete_expense(id)
    }
}

fn map_action(args: &Vec<String>) -> Result<Action, ActionError> {
    let action = args.get(1).unwrap();

    match action.as_str() {
        "add" => map_add_action(args),
        "list" => map_list_action(args),
        "summary" => Ok(Action::Summary),
        "delete" => map_delete_action(args),
        _ => Err(ActionError::UnknowAction)
    }
}

fn map_add_action(args: &Vec<String>) -> Result<Action, ActionError> {
    let description_index = match args.iter().position(|x| x == "--description") {
        Some(index) => index + 1,
        None => Err(ActionError::ArgumentNotFound)?
    };

    let Some(description) = args.iter().nth(description_index) else {
        Err(ActionError::ArgumentNotFound)?
    };

    let amount_index = match args.iter().position(|x| x == "--amount") {
        Some(index) => index + 1,
        None => Err(ActionError::ArgumentNotFound)?
    };

    let Some(amount) = args.get(amount_index) else {
        Err(ActionError::ArgumentNotFound)?
    };

    let amount = match amount.parse::<i32>() {
        Ok(amount) => amount,
        Err(_e) => Err(ActionError::ArgumentInvalid)?
    };

   Ok(Action::Add { description: description.to_string(), amount})
}

fn map_delete_action(args: &Vec<String>) -> Result<Action, ActionError> {
    let id_index = match args.iter().position( |x| x == "--id") {
        Some(index) => index+1,
        None => Err(ActionError::ArgumentNotFound)?
    };

    let Some(id) = args.get(id_index) else {
        Err(ActionError::ArgumentNotFound)?
    };

    match id.parse::<usize>() {
        Ok(id) => Ok(Action::Delete { id }),
        Err(_e) => Err(ActionError::ArgumentInvalid)?
    }
}

fn map_list_action(args: &Vec<String>) -> Result<Action, ActionError> {
    let month_index = match args.iter().position(|x| x == "--month") {
        Some(index) => index+1,
        None => return Ok(Action::List { month: None })
    };

    let Some(month) = args.get(month_index) else {
        Err(ActionError::ArgumentNotFound)?
    };

    match month.parse::<u32>() {
        Ok(month) => Ok(Action::List {month: Some(month)}),
        Err(_e) => Err(ActionError::ArgumentInvalid)?
    }
}

fn show_expense_list(month: Option<u32>) {
    match expense_repository::read_expenses() {
        Ok(list) => {
            if list.is_empty() {
                println!("No expenses found");
                return;
            }

            let mut table = Table::new();
            table.set_header(vec![
                "ID",
                "Date",
                "Description",
                "Amount"
            ]);

            if let Some(month) = month {
                let filtered_list = list.filter_for(month);
                if filtered_list.is_empty() {
                    println!("No expenses found");
                    return;
                }

                for index in 0..filtered_list.len() {
                    match filtered_list.get(index) {
                        Some(expense) => {
                            table.add_row(vec![
                                expense.id().to_string(),
                                expense.date().format("%d/%m/%Y %H:%M:%S").to_string(),
                                expense.description().to_string(),
                                format!("{}€", expense.amount())
                                ]);
                        },
                        None => { panic!("No expense found") }
                    }
                }
            } else {
                for index in 0..list.len() {
                    match list.get(index) {
                        Ok(expense) => {
                            table.add_row(vec![
                                expense.id().to_string(),
                                expense.date().format("%d/%m/%Y %H:%M:%S").to_string(),
                                expense.description().to_string(),
                                format!("{}€", expense.amount())
                            ]);
                        },
                        Err(error) => { panic!("{error}") }
                    }
                }
            }

            println!("{table}");
        },
        Err(error) => { panic!("{error}") }
    };
}

fn show_summary() {
    match expense_repository::read_expenses() {
        Ok(list) => {
            println!("Total expenses: {}€", list.summary());
        },
        Err(error) => { panic!("{error}") }
    }
}

fn delete_expense(index: usize) {
    match expense_repository::read_expenses() {
        Ok(mut list) => {
            list.remove(index);
            match expense_repository::write_expense(&list) {
                Ok(()) => println!("Expense successfully deleted"),
                Err(error) => println!("Error deleting expense: {error}")
            }
        },
        Err(error) => { panic!("{error}") }
    }
}


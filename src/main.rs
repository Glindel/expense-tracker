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
        Action::List => { show_expense_list() },
        _ => panic!("Not implemented yet")
    }
}

fn map_action(args: &Vec<String>) -> Result<Action, ActionError> {
    let action = args.get(1).unwrap();

    match action.as_str() {
        "add" => map_add_action(args),
        "list" => Ok(Action::List),
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

fn show_expense_list() {
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

            println!("{table}");
        },
        Err(error) => { panic!("{error}") }
    };
}


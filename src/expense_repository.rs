use std::fs;
use crate::models::{ExpenseList, ExpenseListError};
use crate::models::errors::LocalizedError;

use std::fs:: { File };
use serde_json;
use crate::expense_repository::ExpenseRepositoryError::FailToCreateExpense;

const FILENAME: &str = "expenses.json";

fn open_or_create_file() -> Result<ExpenseList, ExpenseRepositoryError> {
    match fs::read_to_string(FILENAME) {
        Ok(data) => { decode_expenses(data) },
        Err(_) => {
            println!("Unable to find the expense file");
            println!("------------------------------------");
            println!("Try to create a new one");
            File::create(FILENAME).expect("Unable to create an expense file");
            Ok(ExpenseList::new())
        }
    }
}

fn decode_expenses(data: String) -> Result<ExpenseList, ExpenseRepositoryError> {
    if data.is_empty() {
        return Ok(ExpenseList::new());
    }

    match serde_json::from_str::<ExpenseList>(data.as_str()) {
        Ok(data) => { Ok(data) },
        Err(_) => { Err(ExpenseRepositoryError::FailToDecodeFile) }
    }
}

fn encode_expenses(expenses: &ExpenseList) -> Result<String, ExpenseRepositoryError> {
    match serde_json::to_string(&expenses) {
        Ok(data) => Ok(data),
        Err(_) => { Err(ExpenseRepositoryError::FailToEncodeExpenses) }
    }
}

fn write_expense_in_file(expenses: &ExpenseList) -> Result<(), ExpenseRepositoryError> {
    let string = encode_expenses(expenses)?;
    match fs::write(FILENAME, string) {
        Ok(_) => { Ok(()) },
        Err(_) => { Err(ExpenseRepositoryError::FailToWriteExpensesInFile) }
    }
}

pub fn create_expense(description: String, amount: i32) -> Result<(), ExpenseRepositoryError> {
    let mut expenses = open_or_create_file()?;
    match expenses.append(description, amount) {
        Ok(()) => {
            match expenses.last() {
                Ok(last) => {
                    write_expense_in_file(&expenses)?;
                    println!("Expense successfully created (ID: {})", last.id());
                    Ok(())
                },

                Err(error) => {
                    Err(FailToCreateExpense(error))
                }
            }
        }
        Err(error) => { Err(FailToCreateExpense(error)) }
    }
}

pub enum ExpenseRepositoryError {
    FailToDecodeFile,
    FailToEncodeExpenses,
    FailToWriteExpensesInFile,
    FailToCreateExpense(ExpenseListError)
}

impl LocalizedError for ExpenseRepositoryError {
    fn localized_description(&self) -> &str {
        match self {
            ExpenseRepositoryError::FailToDecodeFile => { "Fail to decode expense file" }
            ExpenseRepositoryError::FailToEncodeExpenses => { "Fail to encode expenses into json string" }
            ExpenseRepositoryError::FailToWriteExpensesInFile => { "Fail to write expenses in the file" }
            FailToCreateExpense( error) => { error.localized_description() }

        }
    }
}
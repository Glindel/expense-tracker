pub mod errors;

use crate::models::errors::LocalizedError;

use chrono::{DateTime, Utc};
use serde::{ Serialize, Deserialize };
use crate::models::ExpenseListError::ExpenseNotFound;

pub enum Action {
    Add { description: String, amount: i32 },
    List ,
    Summary,
    Delete { id: i32 }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Expense {
    id: u32,
    date: DateTime<Utc>,
    description: String,
    amount: i32,
}

impl Expense {
    pub fn new(id: u32, description: String, amount: i32) -> Expense {
        Expense { id, date: Utc::now(), description, amount }
    }

    pub(crate) fn id(&self) -> u32 { self.id }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpenseList {
    list: Vec<Expense>,
    next_id: u32,
}

impl ExpenseList {
    pub fn new() -> ExpenseList {
        ExpenseList { list: Vec::new(), next_id: 0 }
    }

    pub fn append(&mut self, description: String, amount: i32) -> Result<(), ExpenseListError> {
        if  amount < 0 {
            return Err(ExpenseListError::NegativeAmount)
        }
        self.list.push(Expense::new(self.next_id, description, amount));
        self.next_id += 1;
        Ok(())
    }

    pub fn last(&self) -> Result<&Expense, ExpenseListError> {
        match self.list.last() {
            Some(last) => Ok(last),
            None => Err(ExpenseNotFound)
        }
    }
}

pub enum ExpenseListError {
    NegativeAmount,
    ExpenseNotFound
}

impl LocalizedError for ExpenseListError {
    fn localized_description(&self) -> &str {
        match self {
            ExpenseListError::NegativeAmount => "Negative amount given",
            ExpenseNotFound => "The expense search was not found at the specified index"
        }
    }
}

pub enum ActionError {
    UnknowAction,
    ArgumentNotFound,
    ArgumentInvalid
}

impl LocalizedError for ActionError {
    fn localized_description(&self) -> &str {
        match self {
            ActionError::UnknowAction => "The action could not be determined",
            ActionError::ArgumentNotFound => "The argument could not be found for this action",
            ActionError::ArgumentInvalid => "The argument could not be parsed"
        }
    }
}
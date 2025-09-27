use std::error::Error;
use std::fmt;
use chrono::{DateTime, Utc};
use serde::{ Serialize, Deserialize };
use crate::models::ExpenseListError::ExpenseNotFound;

#[derive(Debug)]
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

    pub fn id(&self) -> u32 { self.id }
    pub fn date(&self) -> DateTime<Utc> { self.date }
    pub fn description(&self) -> &String { &self.description }
    pub fn amount(&self) -> i32 { self.amount }
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

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn first(&self) -> Result<&Expense, ExpenseListError> {
        match self.list.first() {
            Some(first) => Ok(first),
            None => Err(ExpenseNotFound)
        }
    }

    pub fn last(&self) -> Result<&Expense, ExpenseListError> {
        match self.list.last() {
            Some(last) => Ok(last),
            None => Err(ExpenseNotFound)
        }
    }

    pub fn get(&self, index: usize) -> Result<&Expense, ExpenseListError> {
        match self.list.get(index) {
            Some(expense) => Ok(expense),
            None => Err(ExpenseNotFound)
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

#[derive(Debug)]
pub enum ExpenseListError {
    NegativeAmount,
    ExpenseNotFound
}

impl fmt::Display for ExpenseListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpenseListError::NegativeAmount => write!(f, "Negative amount given"),
            ExpenseNotFound => write!(f, "The expense search was not found at the specified index")
        }
    }
}

impl Error for ExpenseListError {}

pub enum ActionError {
    UnknowAction,
    ArgumentNotFound,
    ArgumentInvalid
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActionError::UnknowAction => write!(f, "The action could not be determined"),
            ActionError::ArgumentNotFound => write!(f, "The argument could not be found for this action"),
            ActionError::ArgumentInvalid => write!(f, "The argument could not be parsed")
        }
    }
}
# 💰 expense-tracker

[roadmap.sh project](https://roadmap.sh/projects/expense-tracker)

`expense-tracker` is a **Rust CLI application** that helps you track and manage your personal expenses directly from the terminal.  
It allows you to add, update, delete, and list expenses, as well as view summaries and statistics.

---

## 🚀 Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/Glindel/expense-tracker.git
cd expense-tracker
cargo build --release
```

The binary will be available in `target/release/expense-tracker`.

---

## 📌 Usage

Run the command followed by one of the available subcommands.

### ➕ Add an expense
```bash
expense-tracker add --description "Lunch" --amount 20
# Expense added successfully (ID: 1)
```

### ✏️ Update an expense
```bash
expense-tracker update --id 1 --description "Business Lunch" --amount 25
# Expense updated successfully
```

### ❌ Delete an expense
```bash
expense-tracker delete --id 2
# Expense deleted successfully
```

### 📋 List all expenses
```bash
expense-tracker list
# ID  Date       Description       Amount
# 1   2024-08-06  Lunch            $20
# 2   2024-08-06  Dinner           $10
```

### 📊 View a summary
Total summary:
```bash
expense-tracker summary
# Total expenses: $30
```

Summary for a specific month:
```bash
expense-tracker summary --month 8
# Total expenses for August: $20
```


## 📊 Example workflow

```bash
$ expense-tracker add --description "Lunch" --amount 20
# Expense added successfully (ID: 1)

$ expense-tracker add --description "Dinner" --amount 10
# Expense added successfully (ID: 2)

$ expense-tracker list
# ID  Date       Description  Amount
# 1   2024-08-06  Lunch        $20
# 2   2024-08-06  Dinner       $10

$ expense-tracker summary
# Total expenses: $30

$ expense-tracker delete --id 2
# Expense deleted successfully

$ expense-tracker summary
# Total expenses: $20

$ expense-tracker summary --month 8
# Total expenses for August: $20
```


// TODO: Budget Manager
// income
// expense
// Fn:
// 1. add income
// 2. add expense
// 3. View Budget
//
struct BudgetManager {
    income: f32,
    expense: f32,
}

impl BudgetManager {
    fn new() -> Self {
        Self {
            income: 0.0,
            expense: 0.0,
        }
    }

    fn add_income(&mut self, amount: f32) {
        self.income = self.income + amount;
    }

    fn add_expense(&mut self, amount: f32) {
        self.expense = self.expense + amount;
    }

    fn view_budget(&self) {
        let budget = format!(
            "Income: R{:.2}\nExpense: R{:.2}\nBalance: R{:.2}",
            self.income,
            self.expense,
            self.income - self.expense
        );

        println!("{budget}")
    }
}
fn main() {
    let mut budget = BudgetManager::new();

    loop {
        println!("Budget Manager");
        println!("1. Add Income");
        println!("2. Add Expense");
        println!("3. View Budget");
        println!("4. Exit");

        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter the income amount:");
                let mut amount = String::new();
                std::io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");
                let amount: f32 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                budget.add_income(amount);
            }
            2 => {
                println!("Enter the expense amount:");
                let mut amount = String::new();
                std::io::stdin()
                    .read_line(&mut amount)
                    .expect("Failed to read line");
                let amount: f32 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                budget.add_expense(amount);
            }
            3 => budget.view_budget(),
            4 => {
                std::process::exit(0);
            }
            _ => {
                println!("lease enter a number between 1 and 4.");
            }
        }
    }
}

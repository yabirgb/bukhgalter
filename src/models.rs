struct Debtor{
    name: String,
    paid_amount: f32,
    fraction: f32,
    paid: bool
}

struct Item{
    price: f32,
    date: u32,
}

struct Account{
    items: Vec[Item],
    debtors: Vec[Debtor]
}
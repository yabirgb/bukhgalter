use bukhgalter::db::models;

#[test]
fn check_testing_module(){
    assert!(true);
}

#[test]
fn test_modifiers_debtor(){
    let mut new_debtor = models::Debtor{
        name: "Debtor 1".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    new_debtor.update_paid_amount(20.0);
    new_debtor.rename_debtor("Debtor_1".to_string());
    new_debtor.set_fraction(0.3);
    new_debtor.toggle_paid();

    assert_eq!(new_debtor.paid_amount, 20.0, "Failed to modify paid amount");
    assert_eq!(new_debtor.name, "Debtor_1".to_string(), "Failed to rename debtor");
    assert_eq!(new_debtor.fraction, 0.3, "Failed to modify fraction");
    assert_eq!(new_debtor.paid, true, "Failed to toggle paid false to true");

    new_debtor.toggle_paid();

    assert_eq!(new_debtor.paid, false, "Failed to toggle paid true to false");
}

#[test]
fn test_account_add_debtor(){
    let mut acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    let debtor = models::Debtor{
        name: "Debtor 1".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    acc.add_debtor(debtor);

    assert_eq!(acc.debtors.len(), 1);
}

#[test]
fn test_account_add_item(){
    let mut acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    let item = models::Item{
        name: "Item 1".to_string(),
        date: 12312321,
        price: 23.0
    };

    acc.add_item(item.clone());
    acc.add_item(item.clone());

    assert_eq!(acc.items.len(), 2);
}

#[test]
fn test_total_debt_no_debtors(){
    let mut acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    let item = models::Item{
        name: "Item 1".to_string(),
        date: 12312321,
        price: 23.0
    };

    acc.add_item(item.clone());
    acc.add_item(item.clone());

    assert_eq!(acc.total_debt(), 46.0);
}

#[test]
fn test_total_price_debtor(){
    let mut acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    let item = models::Item{
        name: "Item 1".to_string(),
        date: 12312321,
        price: 23.0
    };

    let debtor = models::Debtor{
        name: "Debtor 1".to_string(),
        id: "21323".to_string(),
        paid_amount: 22.0,
        fraction: 0.5,
        paid: false
    };

    acc.add_item(item.clone());
    acc.add_item(item.clone());
    acc.add_debtor(debtor.clone());

    assert_eq!(acc.total_debt(), 24.0);
}

#[test]
fn test_total_price_empty(){
    let acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    assert_eq!(acc.total_debt(), 0.0);
}
use bukhgalter::db::models;
use bukhgalter::db::errors;

// This module tests basic functionalities of methods in 
// the models file.
// This tests are related to the following UH:
// HU1 HU2 HU3 HU4

#[test]
fn check_testing_module(){
    assert!(true);
}

#[test]
fn test_modifiers_debtor(){
    // test the different modifires available for the Debtor
    // struct

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

    // Testing the functionality to add a debtor to an account

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
fn test_account_add_multiple_debtors(){

    // Testing the functionality to add a debtor to an account

    let mut acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    let debtor1 = models::Debtor{
        name: "Debtor 1".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    let debtor2 = models::Debtor{
        name: "Debtor 2".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    acc.add_debtor(debtor1);
    acc.add_debtor(debtor2);

    assert_eq!(acc.debtors.len(), 2);
    assert_eq!(acc.debtors[0].fraction, 0.5);
    assert_eq!(acc.debtors[1].fraction, 0.5);
}

#[test]
fn test_account_add_multiple_debtors_with_fraction(){

    // Testing the functionality to add a debtor to an account

    let mut acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    let debtor1 = models::Debtor{
        name: "Debtor 1".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    let debtor2 = models::Debtor{
        name: "Debtor 2".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    acc.add_debtor(debtor1);
    acc.add_debtor_with_fractions(debtor2, vec![0.3, 0.7]);

    assert_eq!(acc.debtors.len(), 2);
    assert_eq!(acc.debtors[0].fraction, 0.3);
    assert_eq!(acc.debtors[1].fraction, 0.7);
}

#[test]
fn test_account_add_multiple_debtors_with_fraction_invalid(){

    // Testing the functionality to add a debtor to an account

    let mut acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    let debtor1 = models::Debtor{
        name: "Debtor 1".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    let debtor2 = models::Debtor{
        name: "Debtor 2".to_string(),
        id: "21323".to_string(),
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    acc.add_debtor(debtor1);
    let result = acc.add_debtor_with_fractions(debtor2, vec![0.3, 0.8]);

    assert_eq!(result.unwrap_err(), errors::AccountError::InvalidProportions );

}

#[test]
fn test_account_add_item(){

    // test what happens when a new item is added to the list

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

    // Chech that when we add multiple items the 
    // total debt is correctly calculated

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

    // Chech that when a debtor pays one portion the total
    // debt is adjusted correctly

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

    // compute total debt with an empty list of debtors

    let acc = models::Account{
        items: Vec::new(),
        debtors: Vec::new(),
    };

    assert_eq!(acc.total_debt(), 0.0);
}

#[test]
fn test_pay_by_debtor(){
    
    // setup conditions
    
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
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    acc.add_item(item.clone());
    acc.add_debtor(debtor.clone());

    // make the payment

    let payment = acc.pay_by_debtor("Debtor 1".to_string(), 20.0);

    assert_eq!(payment.unwrap(), 0 );

}

#[test]
fn test_pay_by_debtor_not_found(){
    
    // setup conditions
    
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
        paid_amount: 0.0,
        fraction: 0.5,
        paid: false
    };

    acc.add_item(item.clone());
    acc.add_debtor(debtor.clone());

    // make the payment

    let payment = acc.pay_by_debtor("Debtor 2".to_string(), 20.0);

    assert_eq!(payment.unwrap_err(), errors::AccountError::DebtorNotFound );

}
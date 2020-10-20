use rstest::*;

use bukhgalter::models::models::{Debtor, Item, Account};
use bukhgalter::models::errors;

// This module tests basic functionalities of methods in 
// the models file.
// This tests are related to the following UH:
// HU1 HU2 HU3 HU4 HU5


// Const for debtor FIXTURE
const DEFAULT_NAME: &str = "Debtor 1"; // Un nombre cualquiera
const ALTERNATIVE_NAME: &str = "Debtor 2"; // name when there are two fixtures involved
const DEFAULT_ID: &str = "21323"; // Un valor cualquiera
const PAID_AMOUNT: f64 = 0.0; // By default the fixture objects have paid 0.0 units
const FRACTION: f64 = 0.5; // By default debtors are assigned half of the debt in fixtures
const PAID: bool = false; // By default debotrs are not eximed from paying the debt in fixtures

// const for items

const ITEM_NAME: &str = "Item 1";
const DEFAULT_DATE: u32 = 12312332;
const ITEM_PRICE: f64 = 20.3;

#[test]
fn check_testing_module(){
    assert!(true);
}

#[fixture(name=DEFAULT_NAME, id=DEFAULT_ID, paid_amount=PAID_AMOUNT, fraction=FRACTION, paid=PAID)]
fn debtor(name: &str, id:&str, paid_amount:f64, fraction:f64, paid:bool) -> Debtor{
    Debtor{
        name: name.to_string(),
        id: id.to_string(),
        paid_amount: paid_amount,
        fraction: fraction,
        paid: paid
    }
}

#[fixture(name=ALTERNATIVE_NAME, id=DEFAULT_ID, paid_amount=PAID_AMOUNT, fraction=FRACTION, paid=PAID)]
fn debtor_party(name: &str, id:&str, paid_amount:f64, fraction:f64, paid:bool) -> Debtor{
    Debtor{
        name: name.to_string(),
        id: id.to_string(),
        paid_amount: paid_amount,
        fraction: fraction,
        paid: paid
    }
}

#[fixture]
fn account() -> Account{
    Account{
        items: Vec::new(),
        debtors: Vec::new(),
    }
}

#[fixture(name=ITEM_NAME, date=DEFAULT_DATE, price=ITEM_PRICE)]
fn item(name: &str, date: u32, price:f64) -> Item{
    Item{
        name: name.to_string(),
        date: date,
        price: price
    }
}

#[rstest]
fn test_modifiers_debtor(mut debtor: Debtor){

    debtor.update_paid_amount(20.0);
    debtor.rename_debtor("Debtor_1".to_string());
    debtor.set_fraction(0.3);
    debtor.toggle_paid();

    assert_eq!(debtor.paid_amount, 20.0, "Failed to modify paid amount");
    assert_eq!(debtor.name, "Debtor_1".to_string(), "Failed to rename debtor");
    assert_eq!(debtor.fraction, 0.3, "Failed to modify fraction");
    assert_eq!(debtor.paid, true, "Failed to toggle paid false to true");

    debtor.toggle_paid();

    assert_eq!(debtor.paid, false, "Failed to toggle paid true to false");
}

#[rstest]
fn test_account_add_debtor(mut account: Account, debtor: Debtor){
    /*
    This test is related to HU2. In this test we take a paarameters
    an empty account and a debtor and we try to add the debtor to the list
    of debtor.

    We check:

    - The list of debtors has increased in size
    - The debtor in the list of debtors is the one we have added
    */
    assert_eq!(account.debtors.len(),0);
    account.add_debtor(debtor);
    assert_eq!(account.debtors.len(), 1);
    assert_eq!(account.debtors[0].id,DEFAULT_ID);
}

#[rstest]
fn test_account_add_multiple_debtors(mut account: Account, debtor: Debtor, debtor_party: Debtor){

    // Testing the functionality to add a debtor to an account

    account.add_debtor(debtor);
    account.add_debtor(debtor_party);

    assert_eq!(account.debtors.len(), 2);
    assert_eq!(account.debtors[0].fraction, FRACTION);
    assert_eq!(account.debtors[1].fraction, FRACTION);
}

#[rstest]
fn test_account_add_multiple_debtors_with_fraction(mut account: Account, debtor: Debtor, debtor_party: Debtor){

    // Testing the functionality to add a debtor to an account

    account.add_debtor(debtor);
    account.add_debtor_with_fractions(debtor_party, vec![0.3, 0.7]);

    assert_eq!(account.debtors.len(), 2);
    assert_eq!(account.debtors[0].fraction, 0.3);
    assert_eq!(account.debtors[1].fraction, 0.7);
}

#[rstest]
fn test_account_add_multiple_debtors_with_fraction_invalid(mut account: Account, debtor: Debtor, debtor_party: Debtor){

    // Testing the functionality to add a debtor to an account

    account.add_debtor(debtor);
    let result = account.add_debtor_with_fractions(debtor_party, vec![0.3, 0.8]);
    assert_eq!(result.unwrap_err(), errors::AccountError::InvalidProportions );

}

#[rstest]
fn test_account_add_item(mut account: Account, item: Item){

    // test what happens when a new item is added to the list

    account.add_item(item.clone());
    account.add_item(item.clone());

    assert_eq!(account.items.len(), 2);
}

#[rstest]
fn test_total_debt_no_debtors(mut account: Account, item: Item){

    // Chech that when we add multiple items the 
    // total debt is correctly calculated


    account.add_item(item.clone());
    account.add_item(item.clone());

    assert_eq!(account.total_debt(), ITEM_PRICE*2.0);
}

#[rstest]
fn test_total_price_debtor(mut account: Account, debtor: Debtor, item: Item){

    // Chech that when a debtor pays one portion the total
    // debt is adjusted correctly

    account.add_item(item.clone());
    account.add_item(item.clone());
    account.add_debtor(debtor.clone());

    assert_eq!(account.total_debt(), ITEM_PRICE*2.0);
}

#[rstest]
fn test_total_price_empty(account: Account){

    // compute total debt with an empty list of debtors

    assert_eq!(account.total_debt(), 0.0);
}

#[rstest]
fn test_pay_by_debtor(mut account: Account, debtor: Debtor, item: Item){
    
    // setup conditions

    account.add_item(item.clone());
    account.add_debtor(debtor.clone());

    // make the payment

    let payment = account.pay_by_debtor(DEFAULT_NAME.to_string(), ITEM_PRICE);

    assert_eq!(payment.unwrap(), 0 );

}

#[rstest]
fn test_pay_by_debtor_not_found(mut account: Account, debtor: Debtor, item: Item){
    
    // setup conditions
    account.add_item(item.clone());
    account.add_debtor(debtor.clone());

    // make the payment

    let payment = account.pay_by_debtor(ALTERNATIVE_NAME.to_string(), ITEM_PRICE);

    assert_eq!(payment.unwrap_err(), errors::AccountError::DebtorNotFound );

}
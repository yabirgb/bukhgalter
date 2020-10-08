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
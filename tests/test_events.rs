use warp::http::StatusCode;
use bukhgalter::{filters, models};
use rstest::*;
use bukhgalter::models::DataManager;

const ACC2:&str =  r#"
{
    "debtors": [
        {
            "name": "Raton",
            "paid_amount": 0.0,
            "fraction": 0.4,
            "paid": false,
            "id": "1"
        },
        {
            "name": "Gustavo",
            "paid_amount": 0.0,
            "fraction": 0.6,
            "paid": false,
            "id": "2"
        }
    ],
    "items":[
        {
        "date": 5,
        "price": 30,
        "name": "mas chocolate"
        },
        {
        "date": 5,
        "price": 25,
        "name": "queso"
        }
    ],
    "name": "Lidl",
    "id": "1231dfsf1"
}
"#;

const ACC1:&str =  r#"
{
    "debtors": [
        {
            "name": "Gustavo",
            "paid_amount": 0.0,
            "fraction": 1.0,
            "paid": false,
            "id": "1"
        }
    ],
    "items":[
        {
        "date": 23,
        "price": 33.3,
        "name": "choco"
        }
    ],
    "name": "Mercadona",
    "id": "1231dfsf2"
}
"#;


#[fixture]
fn empty_memory_db() -> models::MemoryDataManager{
    models::blank_db()
}

#[fixture]
fn filled_memory_db() -> models::MemoryDataManager{
    let db = models::blank_db();

    let acc1: models::models::Account = serde_json::from_str(&ACC1).unwrap();
    let acc2: models::models::Account = serde_json::from_str(&ACC2).unwrap();

    db.store(acc1);
    db.store(acc2);

    return db
}

#[rstest]
#[tokio::test]
async fn test_create_event(empty_memory_db: impl DataManager){
    let filter = filters::events::event_create(empty_memory_db);


    let res = warp::test::request()
        .method("POST")
        .path("/api/v1/events")
        .header("content-type", "application/json")
        .body(&ACC1)
        .reply(&filter).await;

    let account:models::models::Account = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(res.status(), StatusCode::CREATED );
    assert_eq!(account.name, "Mercadona");
    assert_eq!(account.items.len(), 1);
    assert_eq!(account.debtors.len(), 1)
}

#[rstest]
#[tokio::test]
async fn test_get_event(filled_memory_db: impl DataManager){
    let filter_get = filters::events::event_get(filled_memory_db);

    let res = warp::test::request()
        .path("/api/v1/events/1231dfsf1").reply(&filter_get).await;

    let acc2: models::models::Account = serde_json::from_str(&ACC2).unwrap();

    //let account_response:models::models::Account = serde_json::from_slice(res.body()).unwrap();
    let acapi:models::models::Account = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(acc2, acapi);
}

#[rstest]
#[tokio::test]
async fn make_payment(filled_memory_db: impl DataManager){

    let filter = filters::events::event_make_payment(filled_memory_db);


    // make a payment example. This amount doesnt represent anything special.
    // The account_id and debtor name match the ones in the db

    let payment = r#"{
        "debtor": "Gustavo",
        "account_id": "1231dfsf1",
        "amount": 20}
    "#;

    let res = warp::test::request()
    .method("PATCH")
    .path("/api/v1/events/pay")
    .header("content-type", "application/json")
    .body(&payment)
    .reply(&filter).await;
    
    // comprobamos que se busca bien
    assert_eq!(res.status(), StatusCode::OK);

    // Generamos el estado inicial
    let acc2: models::models::Account = serde_json::from_str(&ACC2).unwrap();
    // obtenemos el estado final
    let acapi:models::models::Account = serde_json::from_slice(res.body()).unwrap();
    // comprobamos la diferencia entre estado inicial y final
    assert_eq!(acc2.total_debt()-20.0, acapi.total_debt());
}

#[rstest]
#[tokio::test]
async fn find_events(filled_memory_db: impl DataManager){

    let filter = filters::events::event_get_by_user(filled_memory_db);

    let res = warp::test::request()
    .path("/api/v1/users/Gustavo")
    .reply(&filter).await;
    
    // comprobamos que se busca bien
    assert_eq!(res.status(), StatusCode::OK);
    let acc2: Vec<models::models::Account> = serde_json::from_slice(res.body()).unwrap();
    assert_eq!(acc2.len(),2);
    assert_eq!(acc2[0].id, "1231dfsf2");
    assert_eq!(acc2[1].id, "1231dfsf1");
}

#[rstest]
#[tokio::test]
async fn test_update_event(filled_memory_db: impl DataManager){
    let filter = filters::events::event_update(filled_memory_db);

    let res = warp::test::request()
        .method("PUT")
        .path("/api/v1/events/1231dfsf2")
        .header("content-type", "application/json")
        .body(&ACC2)
        .reply(&filter).await;

    assert_eq!(res.status(), StatusCode::ACCEPTED );

    let account:models::models::Account = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(account.name, "Lidl");
    assert_eq!(account.items.len(), 2);
    assert_eq!(account.debtors.len(), 2);
}
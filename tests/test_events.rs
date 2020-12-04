use warp::{Filter, http::{Response, StatusCode}};
use bukhgalter::{handlers, models};

#[tokio::test]
async fn test_create_event(){
    let db = models::blank_db();
    let filter = handlers::events::event_create(db);
    let body = r#"
    {
        "debtors": [
            {
                "name": "Yabir",
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
        "name": "Mercadona"
    }
    "#;

    let res = warp::test::request()
        .method("POST")
        .path("/api/v1/events")
        .header("content-type", "application/json")
        .body(body)
        .reply(&filter).await;

    assert_eq!(res.status(), StatusCode::CREATED );
}
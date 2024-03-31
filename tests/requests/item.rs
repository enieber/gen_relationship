use gen_relationship::app::App;
use loco_rs::testing;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_create_item() {
    testing::request::<App, _, _>(|request, _ctx| async move {
        let classification = serde_json::json!({
            "name": "bacia",
        });

        let _res_class = request.post("/api/classifications").json(&classification).await;
        let roles = serde_json::json!({
            "name": "Code_128",
        });

        let _res_roles = request.post("/api/roles").json(&roles).await;
        let payload = serde_json::json!({
            "code": "789014",
            "role_id": 1,
            "classification_id": 1,
        });

        let res = request.post("/api/items").json(&payload).await;
        assert_eq!(res.status_code(), 200);
    })
    .await;
}



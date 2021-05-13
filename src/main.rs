#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::response::status;
use rocket_contrib::json::JsonValue;


#[get("/records")]
fn get_records() -> JsonValue {
    json!([{"id": 1, "name":"Jane Smith"}, {"id": 2, "name":"Jose Brinas"}])
}

#[get("/records/<id>")]
fn get_records_by_id(id: i32) -> JsonValue {
    json!({"id": id, "name":"Jane Smith"})
}

#[post("/records", format = "json")]
fn create_record() -> JsonValue {
    json!({"id": 1, "name":"Jane Smith"})
}

#[put("/records/<id>", format = "json")]
fn update_record(id: i32) -> JsonValue {
    json!({"id": id, "name":"Jane Smith"})
}

#[delete("/records/<_id>")]
fn delete_record(_id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main(){
    let _ = rocket::build()
        .mount("/",routes![
            get_records,
            get_records_by_id,
            create_record,
            update_record,
            delete_record
        ])
        .launch()
        .await;
}
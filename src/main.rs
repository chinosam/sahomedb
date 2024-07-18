use sahomedb::api::*;
use sahomedb::db::database::*;
use sahomedb::*;

use dotenv::dotenv;
use rocket::http::Status;

#[macro_use]
extern crate rocket;

#[catch(404)]
fn catch_404() -> (Status, Response) {
    let message = "Invalid endpoint or method.";
    (Status::NotFound, Response::error(message))
}

#[catch(401)]
fn catch_401() -> (Status, Response) {
    let message = "Invalid x-oasysdb-token header.";
    (Status::Unauthorized, Response::error(message))
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let _token = get_env("SAHOMEDB_TOKEN");
    let dimension = env_get_dimension();

    let config = {
        let path = "data".to_string();
        Config { path, dimension }
    };

    let db = Database::new(config);

    println!("SahomeDB is running on port 3141.");
    println!("SahomeDB accepts embeddings of {} dimension.", dimension);

    rocket::build()
        .manage(db)
        .mount("/", routes![get_status, get_version])
        .mount("/values", routes![set_value, get_value, delete_value])
        .mount("/graphs", routes![create_graph, delete_graph, query_graph])
        .register("/", catchers![catch_401, catch_404])
}

fn env_get_dimension() -> usize {
    let not_int = "variable 'SAHOMEDB_DIMENSION' must be an integer";
    get_env("SAHOMEDB_DIMENSION").parse::<usize>().expect(not_int)
}

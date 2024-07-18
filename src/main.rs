use sahomedb::api::*;
use sahomedb::db::database::*;
use sahomedb::*;

use dotenv::dotenv;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let dimension = env_get_dimension();

    let config = {
        let path = "data".to_string();
        Config { path, dimension }
    };

    // Initialize shared database state.
    let db = Database::new(config);

    println!("SahomeDB is running on port 3141.");
    println!("SahomeDB accepts embeddings of {} dimension.", dimension);

    rocket::build()
        .manage(db)
        .mount("/", routes![get_status])
        .mount("/", routes![get_version])
        .mount("/", routes![set_value])
        .mount("/", routes![get_value])
}

fn env_get_dimension() -> usize {
    let not_int = "variable 'SAHOMEDB_DIMENSION' must be an integer";
    get_env("SAHOMEDB_DIMENSION").parse::<usize>().expect(not_int)
}

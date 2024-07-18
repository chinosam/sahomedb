use sahomedb::db::database::*;
use sahomedb::*;

use dotenv::dotenv;

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let _token = get_env("SAHOMEDB_TOKEN");
    let dimension = env_get_dimension();

    let config = {
        let path = "data".to_string();
        Config { path, dimension }
    };

    println!("SahomeDB is running on port 3141.");
    println!("SahomeDB accepts embeddings of {} dimension.", dimension);

    let db = Database::new(config);
    create_server(db)
}

fn env_get_dimension() -> usize {
    let not_int = "variable 'SAHOMEDB_DIMENSION' must be an integer";
    get_env("SAHOMEDB_DIMENSION").parse::<usize>().expect(not_int)
}

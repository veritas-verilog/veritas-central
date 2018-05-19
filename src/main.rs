#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate postgres;

use rocket::http::RawStr;
use postgres::{Connection, TlsMode};

struct RegistryItem {
    package_name: String,
    package_version: String,
    archive_url: String,
}

/*
 * We can leave it up to veritas client to parse the text
 * into TOML. It should be valid anyways.
 */
#[get("/package/<name>/<version>")]
fn get_package(name: &RawStr, version: &RawStr) -> String {
    let dsn = "postgresql://postgres:postgres@localhost/veritas-central";
    let conn = match Connection::connect(dsn, TlsMode::None) {
        Ok(conn) => conn,
        Err(e) => {
            println!("Connection Error: {}", e);
            return format!("Error with database connection on package {} - {}", name.as_str(), version.as_str());
        }
    };
    let mut pkg_toml: String = "".to_string();
    println!("Searching for: {} -  {}", name.as_str(), version.as_str());
    for row in &conn.query("SELECT * FROM public.packages WHERE package_name = $1 AND package_version = $2", &[&name.as_str(), &version.as_str()]).unwrap() {
        pkg_toml = row.get("package_toml");
        let reg_item = RegistryItem {
            package_name: row.get("package_name"),
            package_version: row.get("package_version"),
            archive_url: row.get("archive_url"),
        };
        println!("Found this: \n\tName: {}\n\tVersion: {}\n\tURL: {}", reg_item.package_name, reg_item.package_version, reg_item.archive_url);
    }
    format!("{}", pkg_toml)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_package])
        .launch();
}

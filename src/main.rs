#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use actix_web::{App, get, HttpResponse, HttpServer, web};
use async_std::task;
use rand::Rng;
use tokio::time::Duration;
use urlencoding::decode;

const NANOS_TO_MS: u32 = 1000000;

#[derive(Eq, Hash, PartialEq)]
struct Package {
    name: String,
    version: String,
}

lazy_static! {
    static ref PACKAGE_MAP: HashMap<Package, String> = {
        let mut packages: HashMap<Package, String> = HashMap::new();
        let mut rdr = csv::Reader::from_path("packages.csv").unwrap();
        for line in rdr.records() {
            let line = line.unwrap();
            let key = Package {
                        name: String::from(line.get(0).unwrap()),
                        version: String::from(line.get(1).unwrap())
                    };
            let value = String::from(line.get(2).unwrap());
            packages.insert(key, value);
        }
        packages
    };
}

#[get("/rest/v1/checksums/{package_name}/{version}")]
async fn check_sum(web::Path((package_name, v)): web::Path<(String, String)>) -> HttpResponse {
    let version = decode(&v).expect("UTF-8").to_string();
    sleep_time().await;
    if package_name.is_empty() || version.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    println!("Received request for package {} with version {}", package_name, version);
    let requested_package = Package { name: package_name, version };
    let pkg = PACKAGE_MAP.get(&requested_package);
    if pkg.is_none() {
        println!("Package {} with version {} does not exist.",
                 requested_package.name, requested_package.version);
        return HttpResponse::NotFound().finish();
    }
    let hash = pkg.unwrap();
    println!("Package {} with version {} has hash {}.",
             requested_package.name, requested_package.version, hash);
    HttpResponse::Ok().body(hash)
}

async fn sleep_time() {
    let mut rng = rand::rngs::OsRng::default();
    let sleep_time = rng.gen_range(25..250);
    task::sleep(Duration::new(0, sleep_time * NANOS_TO_MS)).await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting: {} packages!", PACKAGE_MAP.len());
    HttpServer::new(|| {
        App::new()
            .service(check_sum)
    })
        .bind("0.0.0.0:4590")?
        .run()
        .await
}

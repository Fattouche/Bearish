#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::fmt::format;
use rand::Rng;

#[get("/<symbol>")]
fn price(symbol: String) -> String {
    let price = hit_api(&symbol);
    format!("Price for {} is {}", symbol,price)
}


#[get("/buy/<symbol>/<amount>")]
fn buy(symbol: String, amount: i32) -> String {
    format!("Bought {} shares of {}", amount, symbol)
}

// TODO: https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html use reqwest to make client calls to stock api
fn hit_api(symbol: &String) -> f64{
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0, 10.0)
}



fn main() {
    rocket::ignite().mount("/", routes![price,buy]).launch();
}
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<&'static str> {
    Html(r#"<html><head></head><body><img src="https://disk.yandex.net/qr/?clean=1&amp;text=https://clck.ru/3vyXS"></body></html>"#)
}


fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
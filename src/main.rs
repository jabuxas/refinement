#[macro_use]
extern crate rocket;

use std::{borrow::Cow, ffi::OsStr, path::PathBuf};

use rocket::{http::ContentType, response::content::RawHtml};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "static/"]
struct Asset;

#[get("/")]
fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
    let asset = Asset::get("index.html")?;
    Some(RawHtml(asset.data))
}

#[get("/dist/<file..>")]
fn dist(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
    let filename = file.display().to_string();
    let asset = Asset::get(&filename)?;
    let content_type = file
        .extension()
        .and_then(OsStr::to_str)
        .and_then(ContentType::from_extension)
        .unwrap_or(ContentType::Bytes);

    Some((content_type, asset.data))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .mount("/", FileServer::from("static"))
        .mount("/", routes![index, dist])
}

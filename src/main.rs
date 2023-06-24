use std::env;

use rocket::{build, form::Form, launch, post, routes, FromForm, Responder, State};

type AllowedKeys = Vec<String>;

#[derive(FromForm, Debug)]
#[allow(unused)]
struct Request<'r> {
    app: &'r str,
    #[field(name = "flashver")]
    flash_version: &'r str,
    #[field(name = "swfurl")]
    swf_url: &'r str,
    #[field(name = "tcurl")]
    tc_url: &'r str,
    #[field(name = "pageurl")]
    page_url: &'r str,
    addr: &'r str,
    #[field(name = "clientid")]
    client_id: &'r str,
    call: &'r str,
    #[field(name = "name")]
    key: &'r str,
    r#type: &'r str,
}
#[derive(Responder)]
enum Response {
    #[response(status = 401)]
    Unauthorized(&'static str),
    #[response(status = 200)]
    Ok(&'static str),
}

#[post("/", data = "<req>")]
fn index(req: Form<Request<'_>>, allowed_keys: &State<AllowedKeys>) -> Response {
    println!("{:?}", req);

    assert_eq!(req.app, "stream");
    assert_eq!(req.call, "publish");
    assert_eq!(req.r#type, "live");

    if allowed_keys.iter().any(|k| k == req.key) {
        Response::Ok("authenticated successfully")
    } else {
        Response::Unauthorized("invalid key")
    }
}

#[launch]
fn rocket() -> _ {
    let allowed_keys = env::var("STREAM_KEYS")
        .expect("STREAM_KEYS environment variable must be set")
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    build().manage(allowed_keys).mount("/", routes![index])
}

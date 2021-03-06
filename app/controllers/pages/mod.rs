use iron::{Request, Response, IronResult};
use iron::mime::Mime;
use iron::status;
use layouts;

mod views;

pub fn index(_: &mut Request) -> IronResult<Response> {
  Ok(Response::with((
    status::Ok,
    "text/html".parse::<Mime>().unwrap(),
    layouts::application(views::index::index())
  )))
}
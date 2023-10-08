#[macro_use]
extern crate rocket;

mod bit;

use bit::Message;
use rocket::response::status;

#[post("/<message>")]
fn show(message: &str) -> status::Accepted<String> {
    let message: Message = serde_json::from_str(message).unwrap();
    println!("{}", message);
    status::Accepted(None)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![show])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;

    #[test]
    fn test_show() {
        use rocket::local::blocking::Client;

        let client = Client::tracked(rocket()).unwrap();
        let response = client.post("/[[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]]").dispatch();
        assert_eq!(response.status(), Status::Accepted);
    }
}

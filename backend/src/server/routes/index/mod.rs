use rocket::Route;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

pub fn build() -> Vec<Route> {
    routes![index]
}

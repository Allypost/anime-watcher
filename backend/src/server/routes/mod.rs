use rocket::Route;

mod index;

pub fn build() -> Vec<Route> {
    let routes = vec![index::build()];

    routes.into_iter().flatten().collect()
}

pub mod response{
    use hyper::{Response, Body};

    pub fn internal_server_error()->Response<Body>{
        Response::builder()
        .status(500)
        .body(Body::from("Internal Server Error"))
        .unwrap()
    }
    pub fn not_found()->Response<Body>{
        Response::builder()
        .status(404)
        .body(Body::empty())
        .unwrap()
    }
}
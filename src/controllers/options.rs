use crate::models::response::CorsResponder;

#[options("/<_..>")]
pub async fn get_options<'r>() -> CorsResponder {
    CorsResponder
}

use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response
    }
};

pub enum JournalError {
    SomethingWentWrong,
}

impl IntoResponse for JournalError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong"
        )
            .into_response()
    }
}
        


use crate::http::problem::SimpleProblemType;
use actix_http::http::StatusCode;

pub const UNAUTHORIZED: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:nrl/2020:problems/unauthorized",
    problem_title: "Authorization was not present or not valid",
    status_code: StatusCode::UNAUTHORIZED,
};

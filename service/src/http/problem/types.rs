use super::{Problem, ProblemType, ProblemTypeStatus};
use actix_http::http::StatusCode;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct SimpleProblemType {
    pub problem_type: &'static str,
    pub problem_title: &'static str,
    pub status_code: StatusCode,
}

impl ProblemType for SimpleProblemType {
    /// A URI Reference that identifies the problem type.
    fn problem_type(&self) -> &'static str {
        self.problem_type
    }
}

impl ProblemTypeStatus for SimpleProblemType {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl Display for SimpleProblemType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.problem_title)
    }
}

impl From<SimpleProblemType> for Problem {
    fn from(problem_type: SimpleProblemType) -> Self {
        Self::new(problem_type)
    }
}

pub const NOT_FOUND: SimpleProblemType = SimpleProblemType {
    problem_type: "tag:nrl/2020:problems/not_found",
    problem_title: "The requested resource was not found",
    status_code: StatusCode::NOT_FOUND,
};

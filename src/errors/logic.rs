pub enum UpdateError {
    UnknownMetric,
    NotFound,
    BadFormat,
    ProblemStorage
}

pub enum GetMetricError {
    NotFound,
    ProblemStorage
}
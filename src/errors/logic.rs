pub enum UpdateError {
    UnknownMetric,
    NotFound,
    BadFormat,
    ProblemStorage
}

#[derive(Debug)]
pub enum GetMetricError {
    NotFound,
    ProblemStorage
}
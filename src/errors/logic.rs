pub enum UpdateError {
    UnknownMetric,
    BadFormat,
    ProblemStorage
}

#[derive(Debug)]
pub enum GetMetricError {
    NotFound,
    ProblemStorage,
    UnknownMetric
}
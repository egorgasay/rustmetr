use crate:: application::service::metric::MetricService;

pub struct AppState<'a> {
    pub app_name: String,
    pub logic: MetricService<'a>,
}

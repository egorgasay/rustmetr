use crate:: application::usecases::usecase::UseCase;

pub struct AppState<'a> {
    pub app_name: String,
    pub logic: UseCase<'a>,
}

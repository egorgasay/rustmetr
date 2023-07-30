use crate:: application::usecases::usecase::UseCase;
use std::sync::{Mutex, RwLock};

pub struct AppState<'a> {
    pub app_name: String,
    pub logic: Mutex<UseCase<'a>>,
}

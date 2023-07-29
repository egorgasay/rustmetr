use crate::adapters::spi::{http::http_cat_facts_repository::CatFactsRepository};
use crate:: application::usecases::get_all_cat_facts_usecase::UseCase;

pub struct AppState<'a> {
    pub app_name: String,
    pub logic: UseCase<'a>,
}

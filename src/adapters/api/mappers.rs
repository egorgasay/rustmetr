//use crate::adapters::api::payloads::Payload;
//use crate::adapters::api::presenters::Presenter;
//use crate::application::mappers::api_mapper::ApiMapper;
//use crate::domain::cat_fact_entity::CatFactEntity;
//
//pub struct CatFactPresenterMapper {}
//
//impl ApiMapper<CatFactEntity, Presenter, Payload> for CatFactPresenterMapper {
//    fn to_api(entity: CatFactEntity) -> Presenter {
//        CatFactPresenter {
//            fact: entity.fact_txt,
//            nb_chars: entity.fact_length,
//        }
//    }
//
//    fn to_entity(_payload: Payload) -> CatFactEntity {
//        panic!("not implemented");
//    }
//}

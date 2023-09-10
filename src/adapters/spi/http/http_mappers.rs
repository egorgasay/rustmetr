use actix_web::web;
use crate::adapters::spi::http::http_models::{MetricAPI};
use crate::domain::entity::{Metric};

pub struct HTTPMapper {}

impl HTTPMapper {
   pub(crate) fn to_http(entity: Metric) -> MetricAPI {
       MetricAPI {
           id: entity.name.to_string(),
           mtype: entity.mtype,
           value: entity.value,
           delta: entity.delta,
       }
   }

    pub(crate) fn to_entity(http_obj: web::Json<MetricAPI>) -> Metric {
        Metric {
            name: http_obj.id.to_string(),
            mtype: http_obj.mtype.to_string(),
            value: http_obj.value,
            delta: http_obj.delta,
        }
    }
}

// impl HttpMapper<GaugeMetric<'_>, MetricAPI> for HTTPMapper {
//     fn to_http(entity: GaugeMetric) -> MetricAPI {
//         MetricAPI {
//             id: entity.name.to_string(),
//             mtype: String::from("gauge"),
//             value: Some(entity.value),
//             delta: None,
//         }
//     }
//
//     fn to_entity(http_obj: MetricAPI) -> GaugeMetric<'static> {
//         GaugeMetric {
//             name: &*http_obj.id,
//             value: http_obj.value.unwrap_or_default(),
//         }
//     }
// }


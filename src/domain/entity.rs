
pub struct Metric {
    pub(crate) name: String,
    pub(crate) mtype: String,
    pub(crate) value: Option<f64>,
    pub(crate) delta: Option<i64>,
}

impl Metric {
    pub fn new(name: String, mtype: String, value: Option<f64>, delta: Option<i64>) -> Self {
        Metric {
            name,
            mtype,
            value,
            delta,
        }
    }
}

// pub trait Metric {
//     fn name(&self) -> &str;
//
//     fn is_gauge(&self) -> bool;
//     fn value(&self) -> Option<f64>;
//     fn set_value(&mut self, value: f64);
//
//     fn is_counter(&self) -> bool;
//     fn delta(&self) -> Option<i64>;
//     fn set_delta(&mut self, delta: i64);
// }
//
// #[derive(Debug, Clone)]
// pub struct GaugeMetric<'a> {
//     pub name: &'a str,
//     pub value: f64,
// }
//
// impl GaugeMetric<'_> {
//     pub fn new(name: &str, value: f64) -> Self {
//         GaugeMetric { name, value }
//     }
// }
//
// impl Metric for GaugeMetric<'_> {
//     fn name(&self) -> &str {
//         self.name
//     }
//
//     fn is_gauge(&self) -> bool {
//         true
//     }
//
//     fn value(&self) -> Option<f64> {
//         Some(self.value)
//     }
//
//     fn set_value(&mut self, value: f64) {
//         self.value = value
//     }
//
//     fn is_counter(&self) -> bool {
//         false
//     }
//
//     fn delta(&self) -> Option<i64> {
//         //unimplemented!()
//         None
//     }
//
//     fn set_delta(&mut self, _: i64) {
//         //unimplemented!()
//     }
// }
//
// #[derive(Debug, Clone)]
// pub struct CounterMetric<'a> {
//     pub name: &'a str,
//     pub delta: i64,
// }
//
// impl CounterMetric<'_> {
//     pub fn new(name: &str, delta: i64) -> Self {
//         CounterMetric { name, delta }
//     }
// }
//
// impl Metric for CounterMetric<'_> {
//     fn name(&self) -> &str {
//         self.name
//     }
//
//     fn is_gauge(&self) -> bool {
//         false
//     }
//
//     fn value(&self) -> Option<f64> {
//         //unimplemented!()
//         None
//     }
//
//     fn set_value(&mut self, value: f64) {
//         // unimplemented!()
//     }
//
//     fn is_counter(&self) -> bool {
//         true
//     }
//
//     fn delta(&self) -> Option<i64> {
//         Some(self.delta)
//     }
//
//     fn set_delta(&mut self, delta: i64) {
//         self.delta = delta
//     }
// }
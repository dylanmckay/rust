use crate::spec::{TargetResult};

pub fn target() -> TargetResult {
    super::avr_unknown_unknown::target().map(|mut avr_unknown_unknown| {
        avr_unknown_unknown.options.cpu = "atmega328".to_owned();
        avr_unknown_unknown
    })
}

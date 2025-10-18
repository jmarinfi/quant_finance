#![doc = include_str!("../README.md")]

mod common;
mod time_value;

pub use common::{FinanceError, FinanceResult};
pub use time_value::{future_value, present_value};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_test_time_value() {
        // Test de integración básico
        let pv = 1000.0;
        let rate = 0.08;
        let periods = 5;

        let fv = future_value(pv, rate, periods).unwrap();
        let back_to_pv = present_value(fv, rate, periods).unwrap();

        assert!((pv - back_to_pv).abs() < 1e-10);
    }
}

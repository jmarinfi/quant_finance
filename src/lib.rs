#![doc = include_str!("../README.md")]

pub mod common;
pub mod time_value;

// Re-exportar los tipos más comunes
pub use common::{FinanceError, FinanceResult};

// Re-exportar funciones principales de time_value
pub use time_value::{
    future_value, present_value,
    simple_interest, simple_interest_amount,
    compound_amount, compound_interest, continuous_compound_amount
};

// Mantener la función de ejemplo hasta que tengas más funcionalidad
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn integration_test_simple_vs_compound() {
        let principal = 1000.0;
        let rate = 0.06;
        let time = 3.0;

        let simple = simple_interest_amount(principal, rate, time).unwrap();
        let compound = compound_amount(principal, rate, 1, time).unwrap();

        // El interés compuesto debe generar más que el simple
        assert!(compound > simple);

        // Valores esperados aproximados
        assert!((simple - 1180.0).abs() < 0.1);
        assert!((compound - 1191.02).abs() < 0.1);
    }
}

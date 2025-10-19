#![doc = include_str!("../README.md")]

pub mod common;
pub mod time_value;
pub mod options;

// Re-exportar los tipos más comunes
pub use common::{FinanceError, FinanceResult, OptionType};

// Re-exportar funciones principales de time_value
pub use time_value::{
    future_value, present_value,
    simple_interest, simple_interest_amount,
    compound_amount, compound_interest, continuous_compound_amount
};

// Re-exportar funciones principales de options
pub use options::{
    call_price, put_price, option_price, d1_d2
};


#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn integration_test_black_scholes_call() {
        // Parámetros típicos: S0=100, K=100, T=1 año, r=5%, σ=20%
        let call = call_price(100.0, 100.0, 1.0, 0.05, 0.20).unwrap();
        
        // El precio de la call ATM con estos parámetros debe estar entre 8 y 12
        assert!(call > 8.0 && call < 12.0);
        
        // Valor aproximado conocido
        assert!((call - 10.45).abs() < 0.5);
    }

    #[test]
    fn integration_test_put_call_parity() {
        let s0 = 100.0;
        let k = 105.0;
        let t = 0.5;
        let r = 0.03;
        let sigma = 0.25;

        let call = call_price(s0, k, t, r, sigma).unwrap();
        let put = put_price(s0, k, t, r, sigma).unwrap();
        
        // Put-Call Parity: C - P = S0 - K*e^(-rT)
        let df = (-r * t).exp();
        let parity_lhs = call - put;
        let parity_rhs = s0 - k * df;
        
        assert!((parity_lhs - parity_rhs).abs() < 1e-10);
    }

    #[test]
    fn integration_test_option_dispatch() {
        let params = (100.0, 100.0, 1.0, 0.05, 0.20);
        
        let call_direct = call_price(params.0, params.1, params.2, params.3, params.4).unwrap();
        let call_dispatch = option_price(params.0, params.1, params.2, params.3, params.4, OptionType::Call).unwrap();
        
        let put_direct = put_price(params.0, params.1, params.2, params.3, params.4).unwrap();
        let put_dispatch = option_price(params.0, params.1, params.2, params.3, params.4, OptionType::Put).unwrap();
        
        assert_eq!(call_direct, call_dispatch);
        assert_eq!(put_direct, put_dispatch);
    }
}

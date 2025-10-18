//! Fírmulas básicas de valor temporal del dinero
//!
//! Implementa las fórmulas fundamentales para calcular el valor futuro y presente del dinero
//! basadas en interés compuesto simple.

use crate::common::{FinanceResult, FinanceError, InterestRate, MonetaryValue, Periods};

/// Calcula el valor futuro dado un valor presente
///
/// # Fórmula
/// FV = PV * (1 + r)^n
///
/// # Argumentos
/// * `present_value` - Valor presente (PV)
/// * `interest_rate` - Tasa de interés por período como decimal (ej: 0.05 para 5%)
/// * `periods` - Número de períodos (n)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::future_value;
///
/// let fv = future_value(1000.0, 0.05, 10).unwrap();
/// assert!((fv - 1628.89).abs() < 0.01);
/// ```
///
/// # Errores
/// * `InvalidMonetaryValue` si el valor presente es negativo
/// * `InvalidInterestRate` si la tasa de interés es menor a -1 (lo que haría que 1 + r sea negativo)
pub fn future_value(
    present_value: MonetaryValue,
    interest_rate: InterestRate,
    periods: Periods
) -> FinanceResult<MonetaryValue> {
    if present_value < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }

    if interest_rate <= -1.0 {
        return Err(FinanceError::InvalidInterestRate);
    }

    let compound_factor = (1.0 + interest_rate).powi(periods as i32);
    Ok(present_value * compound_factor)
}

/// Calcula el valor presente dado un valor futuro
///
/// # Fórmula
/// PV = FV / (1 + r)^n
///
/// # Argumentos
/// * `future_value` - Valor futuro (FV)
/// * `interest_rate` - Tasa de interés por período como decimal (ej: 0.05 para 5%)
/// * `periods` - Número de períodos (n)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::present_value;
///
/// let pv = present_value(1628.89, 0.05, 10).unwrap();
/// assert!((pv - 1000.0)abs() < 0.01);
/// ```
///
/// # Errores
/// * `InvalidMonetaryValue` - Si el valor futuro es negativo
/// * `InvalidInterestRate` - Si la tasa de interés es menor a -1
/// * `DivisionByZero` - Si (1 + r)^n = 0
pub fn present_value(
    future_value: MonetaryValue,
    interest_rate: InterestRate,
    periods: Periods
) -> FinanceResult<MonetaryValue> {
    if future_value < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }

    if interest_rate <= -1.0 {
        return Err(FinanceError::InvalidInterestRate);
    }

    let compound_factor = (1.0 + interest_rate).powi(periods as i32);

    if compound_factor == 0.0 {
        return Err(FinanceError::DivisionByZero);
    }

    Ok(future_value / compound_factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_future_value_basic() {
        let result = future_value(1000.0, 0.05, 5).unwrap();
        assert!((result - 1276.2815625).abs() < 1e-6);
    }

    #[test]
    fn test_present_value_basic() {
        let result = present_value(1276.2815625, 0.05, 5).unwrap();
        assert!((result - 1000.0).abs() < 1e-6);
    }

    #[test]
    fn test_reciprocal_relationship() {
        let pv = 1000.0;
        let rate = 0.07;
        let periods = 5;

        let fv = future_value(pv, rate, periods).unwrap();
        let calculated_pv = present_value(fv, rate, periods).unwrap();

        assert!((pv - calculated_pv).abs() < 1e-10);
    }

    #[test]
    fn test_zero_interest_rate() {
        assert_eq!(future_value(1000.0, 0.0, 10).unwrap(), 1000.0);
        assert_eq!(present_value(1000.0, 0.0, 10).unwrap(), 1000.0);
    }

    #[test]
    fn test_invalid_present_value() {
        assert_eq!(
            future_value(-100.0, 0.05, 10),
            Err(FinanceError::InvalidMonetaryValue)
        );
    }

    #[test]
    fn test_invalid_interest_rate() {
        assert_eq!(
            future_value(1000.0, -1.5, 10),
            Err(FinanceError::InvalidInterestRate)
        );
    }
}
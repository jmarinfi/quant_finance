//! Fórmulas de interés simple
//!
//! El interés simple se calcula únicamente sobre el principal inicial,
//! sin capitalización de intereses.

use crate::common::{
    FinanceError, FinanceResult, Interest, InterestRate,
    MonetaryValue, Principal, TimeInYears
};

/// Calcula el interés simple ganado
///
/// # Fórmula
/// I = P * r * t
///
/// # Argumentos
/// * `principal` - Capital inicial (P)
/// * `interest_rate` - Tasa de interés anual como decimal (r)
/// * `time_years` - Tiempo en años (t)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::simple_interest;
///
/// let interest = simple_interest(1000.0, 0.05, 2.0).unwrap();
/// assert_eq!(interest, 100.0);
/// ```
///
/// # Errores
/// * `InvalidMonetaryValue` - Si el principal es negativo
/// * `InvalidInterestRate` - Si la tasa de interés es negativa
/// * `InvalidPeriods` - Si el tiempo es negativo
pub fn simple_interest(
    principal: Principal,
    interest_rate: InterestRate,
    time_in_years: TimeInYears
) -> FinanceResult<Interest> {
    validate_simple_interest_inputs(principal, interest_rate, time_in_years)?;

    Ok(principal * interest_rate * time_in_years)
}

/// Calcula el monto total con interés simple
///
/// # Fórmula
/// A = P + I = P + (P * r * t) = P * (1 + r * t)
///
/// # Argumentos
/// * `principal` - Capital inicial (P)
/// * `interest_rate` - Tasa de interés anual como decimal (r)
/// * `time_years` - Tiempo en años (t)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::simple_interest_amount;
///
/// let amount = simple_interest_amount(1000.0, 0.05, 2.0).unwrap();
/// assert_eq!(amount, 1100.0);
/// ```
pub fn simple_interest_amount(
    principal: Principal,
    interest_rate: InterestRate,
    time_years: TimeInYears
) -> FinanceResult<MonetaryValue> {
    validate_simple_interest_inputs(principal, interest_rate, time_years)?;

    Ok(principal * (1.0 + interest_rate * time_years))
}

/// Calcula el principal necesario para obtener una cantidad específica con interés simple
///
/// # Fórmula
/// P = A / (1 + r * t)
///
/// # Argumentos
/// * `target_amount` - Monto objetivo (A)
/// * `interest_rate` - Tasa de interés anual como decimal (r)
/// * `time_years` - Tiempo en años (t)
pub fn simple_interest_principal(
    target_amount: MonetaryValue,
    interest_rate: InterestRate,
    time_years: TimeInYears
) -> FinanceResult<Principal> {
    if target_amount < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }
    if interest_rate < 0.0 {
        return Err(FinanceError::InvalidInterestRate);
    }
    if time_years < 0.0 {
        return Err(FinanceError::InvalidPeriods);
    }

    let denominator = 1.0 + interest_rate * time_years;
    if denominator == 0.0 {
        return Err(FinanceError::DivisionByZero);
    }

    Ok(target_amount / denominator)
}

/// Calcula la tasa de interés necesaria para alcanzar una cantidad con interés simple
///
/// # Fórmula
/// r = (A - P) / (P * t) = (A/P - 1) / t
///
/// # Argumentos
/// * `principal` - Capital inicial (P)
/// * `target_amount` - Monto objetivo (A)
/// * `time_years` - Tiempo en años (t)
pub fn simple_interest_rate(
    principal: Principal,
    target_amount: MonetaryValue,
    time_years: TimeInYears
) -> FinanceResult<InterestRate> {
    if principal <= 0.0 || target_amount < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }
    if time_years <= 0.0 {
        return Err(FinanceError::InvalidPeriods);
    }

    Ok((target_amount / principal - 1.0) / time_years)
}

/// Validación común para parámetros de interés simple
fn validate_simple_interest_inputs(
    principal: Principal,
    interest_rate: InterestRate,
    time_years: TimeInYears
) -> FinanceResult<()> {
    if principal < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }
    if interest_rate < 0.0 {
        return Err(FinanceError::InvalidInterestRate);
    }
    if time_years < 0.0 {
        return Err(FinanceError::InvalidPeriods);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interest_basic() {
        let interest = simple_interest(1000.0, 0.05, 2.0).unwrap();
        assert_eq!(interest, 100.0);
    }

    #[test]
    fn test_simple_interest_amount() {
        let amount = simple_interest_amount(1000.0, 0.05, 2.0).unwrap();
        assert_eq!(amount, 1100.0);
    }

    #[test]
    fn test_simple_interest_principal() {
        let principal = simple_interest_principal(1100.0, 0.05, 2.0).unwrap();
        assert!((principal - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_simple_interest_rate() {
        let rate = simple_interest_rate(1000.0, 1100.0, 2.0).unwrap();
        assert!((rate - 0.05).abs() < 1e-10);
    }

    #[test]
    fn test_fractional_time() {
        let interest = simple_interest(1000.0, 0.06, 0.5).unwrap(); // 6 meses
        assert_eq!(interest, 30.0);
    }

    #[test]
    fn test_zero_interest_rate() {
        let amount = simple_interest_amount(1000.0, 0.0, 5.0).unwrap();
        assert_eq!(amount, 1000.0);
    }

    #[test]
    fn test_invalid_inputs() {
        assert_eq!(
            simple_interest(-100.0, 0.05, 1.0),
            Err(FinanceError::InvalidMonetaryValue)
        );
        assert_eq!(
            simple_interest(1000.0, -0.05, 1.0),
            Err(FinanceError::InvalidInterestRate)
        );
        assert_eq!(
            simple_interest(1000.0, 0.05, -1.0),
            Err(FinanceError::InvalidPeriods)
        );
    }
}
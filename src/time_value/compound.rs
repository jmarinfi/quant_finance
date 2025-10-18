//! Fórmulas de interés compuesto
//!
//! El interés compuesto incluye capitalización de intereses,
//! permitiendo diferentes frecuencias de capitalización.

use crate::common::{
    FinanceResult, FinanceError, InterestRate, Principal,
    CompoundingFrequency, TimeInYears, MonetaryValue
};

/// Calcula el monto con interés compuesto
///
/// # Fórmula
/// A = P * (1 + r/n)^(n*t)
///
/// # Argumentos
/// * `principal` - Capital inicial (P)
/// * `annual_rate` - Tasa de interés anual como decimal (r)
/// * `compounding_frequency` - Veces que se capitaliza por año (n)
/// * `time_years` - Tiempo en años (t)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::compound_amount;
///
/// // Capitalización trimestral (4 veces por año)
/// let amount = compound_amount(1000.0, 0.08, 4, 2.0).unwrap();
/// assert!((amount - 1171.66).abs() < 0.01);
///```
///
/// # Errores
/// * `InvalidMonetaryValue` - Si el principal es negativo
/// * `InvalidInterestRate` - Si la tasa anual es negativa
/// * `InvalidPeriods` - Si el tiempo es negativo o la frecuencia es 0
pub fn compound_amount(
    principal: Principal,
    annual_rate: InterestRate,
    compounding_frequency: CompoundingFrequency,
    time_years: TimeInYears
) -> FinanceResult<MonetaryValue> {
    validate_compound_inputs(principal, annual_rate, compounding_frequency, time_years)?;

    let rate_per_period = annual_rate / (compounding_frequency as f64);
    let total_periods = (compounding_frequency as f64) * time_years;

    Ok(principal * (1.0 + rate_per_period).powf(total_periods))
}

/// Calcula el interés ganado con capitalización compuesta
///
/// # Fórmula
/// I = A - P = P * [(1 + r/n)^(n*t) - 1]
///
/// # Argumentos
/// * `principal` - Capital inicial (P)
/// * `annual_rate` - Tasa de interés anual como decimal (r)
/// * `compounding_frequency` - Veces que se capitaliza por año (n)
/// * `time_years` - Tiempo en años (t)
pub fn compound_interest(
    principal: Principal,
    annual_rate: InterestRate,
    compounding_frequency: CompoundingFrequency,
    time_years: TimeInYears
) -> FinanceResult<MonetaryValue> {
    let amount = compound_amount(principal, annual_rate, compounding_frequency, time_years)?;
    Ok(amount - principal)
}

/// Calcula el principal necesario para obtener una cantidad específica con interés compuesto
///
/// # Fórmula
/// P = A / (1 + r/n)^(n*t)
///
/// # Argumentos
/// * `target_amount` - Monto objetivo (A)
/// * `annual_rate` - Tasa de interés anual como decimal (r)
/// * `compounding_frequency` - Veces que se capitaliza por año (n)
/// * `time_years` - Tiempo en años (t)
pub fn compound_principal(
    target_amount: MonetaryValue,
    annual_rate: InterestRate,
    compounding_frequency: CompoundingFrequency,
    time_years: TimeInYears
) -> FinanceResult<Principal> {
    if target_amount < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }

    validate_compound_params(annual_rate, compounding_frequency, time_years)?;

    let rate_per_period = annual_rate / (compounding_frequency as f64);
    let total_periods = (compounding_frequency as f64) * time_years;
    let compound_factor = (1.0 + rate_per_period).powf(total_periods);

    if compound_factor == 0.0 {
        return Err(FinanceError::DivisionByZero)
    }

    Ok(target_amount / compound_factor)
}

/// Capitalización continua usando e^(rt)
///
/// # Fórmula
/// A = P * e^(r*t)
///
/// # Argumentos
/// * `principal` - Capital inicial (P)
/// * `annual_rate` - Tasa de interés anual como decimal (r)
/// * `time_years` - Tiempo en años (t)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::continuous_compound_amount;
///
/// let amount = continuous_compound_amount(1000.0, 0.05, 2.0).unwrap();
/// assert!((amount - 1105.17).abs() < 0.01);
/// ```
pub fn continuous_compound_amount(
    principal: Principal,
    annual_rate: InterestRate,
    time_years: TimeInYears
) -> FinanceResult<MonetaryValue> {
    if principal < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }
    if annual_rate < 0.0 {
        return Err(FinanceError::InvalidInterestRate);
    }
    if time_years < 0.0 {
        return Err(FinanceError::InvalidPeriods);
    }

    Ok(principal * (annual_rate * time_years).exp())
}

/// Tasa efectiva anual (EAR) dada una tasa nominal y frecuencia de capitalización
///
/// # Fórmula
/// EAR = (1 + r/n)^n - 1
///
/// # Argumentos
/// * `nominal_rate` - Tasa nominal anual (r)
/// * `compounding_frequency` - Veces que se capitaliza por año (n)
pub fn effective_annual_rate(
    nominal_rate: InterestRate,
    compounding_frequency: CompoundingFrequency
) -> FinanceResult<InterestRate> {
    if nominal_rate < 0.0 {
        return Err(FinanceError::InvalidInterestRate);
    }
    if compounding_frequency == 0 {
        return Err(FinanceError::InvalidPeriods);
    }

    let rate_per_period = nominal_rate / (compounding_frequency as f64);
    Ok((1.0 + rate_per_period).powi(compounding_frequency as i32) - 1.0)
}

/// Validación para parámetros de interés compuesto
fn validate_compound_inputs(
    principal: Principal,
    annual_rate: InterestRate,
    compounding_frequency: CompoundingFrequency,
    time_years: TimeInYears
) -> FinanceResult<()> {
    if principal < 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }
    validate_compound_params(annual_rate, compounding_frequency, time_years)
}

fn validate_compound_params(
    annual_rate: InterestRate,
    compounding_frequency: CompoundingFrequency,
    time_years: TimeInYears
) -> FinanceResult<()> {
    if annual_rate < 0.0 {
        return Err(FinanceError::InvalidInterestRate);
    }
    if compounding_frequency == 0 {
        return Err(FinanceError::InvalidPeriods);
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
    fn test_compound_amount_annual() {
        // Capitalización anual
        let amount = compound_amount(1000.0, 0.05, 1, 10.0).unwrap();
        assert!((amount - 1628.8946267).abs() < 1e-6);
    }

    #[test]
    fn test_compound_amount_quarterly() {
        // Capitalización trimestral
        let amount = compound_amount(1000.0, 0.08, 4, 2.0).unwrap();
        assert!((amount - 1171.659).abs() < 0.001);
    }

    #[test]
    fn test_compound_interest() {
        let interest = compound_interest(1000.0, 0.05, 1, 2.0).unwrap();
        assert!((interest - 102.5).abs() < 0.001);
    }

    #[test]
    fn test_compound_principal() {
        let principal = compound_principal(1102.5, 0.05, 1, 2.0).unwrap();
        assert!((principal - 1000.0).abs() < 0.001);
    }

    #[test]
    fn test_continuous_compound() {
        let amount = continuous_compound_amount(1000.0, 0.05, 2.0).unwrap();
        assert!((amount - 1105.170918).abs() < 1e-6);
    }

    #[test]
    fn test_effective_annual_rate() {
        // 8% nominal capitalizado trimestralmente
        let ear = effective_annual_rate(0.08, 4).unwrap();
        assert!((ear - 0.08243216).abs() < 1e-8);
    }

    #[test]
    fn test_compound_vs_simple() {
        let principal = 1000.0;
        let rate = 0.05;
        let time = 2.0;

        // Interés simple
        let simple_amount = principal * (1.0 + rate * time);

        // Interés compuesto anual
        let compound_amount_val = compound_amount(principal, rate, 1, time).unwrap();

        // El compuesto debe ser mayor que el simple
        assert!(compound_amount_val > simple_amount);
    }

    #[test]
    fn test_higher_frequency_higher_amount() {
        let principal = 1000.0;
        let rate = 0.08;
        let time = 1.0;

        let annual = compound_amount(principal, rate, 1, time).unwrap();
        let quarterly = compound_amount(principal, rate, 4, time).unwrap();
        let monthly = compound_amount(principal, rate, 12, time).unwrap();
        let continuous = continuous_compound_amount(principal, rate, time).unwrap();

        assert!(annual < quarterly);
        assert!(quarterly < monthly);
        assert!(monthly < continuous);
    }

    #[test]
    fn test_invalid_inputs() {
        assert_eq!(
            compound_amount(-100.0, 0.05, 4, 1.0),
            Err(FinanceError::InvalidMonetaryValue)
        );
        assert_eq!(
            compound_amount(1000.0, -0.05, 4, 1.0),
            Err(FinanceError::InvalidInterestRate)
        );
        assert_eq!(
            compound_amount(1000.0, 0.05, 0, 1.0),
            Err(FinanceError::InvalidPeriods)
        );
    }
}
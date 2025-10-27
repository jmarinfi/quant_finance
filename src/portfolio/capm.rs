//! Capital Asset Pricing Model (CAPM)
//!
//! Implementación del modelo CAPM que establece la relación entre
//! el rendimiento esperado de un activo y su riesgo sistemático.
//!
//! Fórmula principal:
//! E(Ri) = Rf + βi * [E(Rm) - Rf]
//!
//! Donde:
//! - E(Ri) = Rentabilidad esperada de la inversión
//! - Rf = Tasa libre de riesgo
//! - βi = Beta de la inversión
//! - E(Rm) = Rentabilidad esperada del mercado
//! - [E(Rm) - Rf] = Prima de riesgo de mercado

use crate::common::{
    Beta, ExpectedReturn, FinanceError, FinanceResult, MarketReturn, MarketRiskPremium,
    RiskFreeRate,
};

/// Calcula la rentabilidad esperada de una inversión usando CAPM.
///
/// # Argumentos
/// * `risk_free_rate` - Tasa libre de riesgo (Rf)
/// * `beta` - Beta de la inversión (βi)
/// * `market_risk_premium` - Prima de riesgo de mercado [E(Rm) - Rf]
///
/// # Fórmula
/// E(Ri) = Rf + βi * [E(Rm) - Rf]
///
/// # Ejemplo
/// ```
/// use quant_finance::portfolio::expected_return;
///
/// let rf = 0.03;  // 3% tasa libre de riesgo
/// let beta = 1.2; // Beta de 1.2
/// let market_premium = 0.08; // 8% prima de riesgo
///
/// let expected = expected_return(rf, beta, market_premium).unwrap();
/// assert!((expected - 0.126).abs() < 1e-10); // 12.6%
/// ```
pub fn expected_return(
    risk_free_rate: RiskFreeRate,
    beta: Beta,
    market_risk_premium: MarketRiskPremium,
) -> FinanceResult<ExpectedReturn> {
    validate_capm_inputs(risk_free_rate, beta, market_risk_premium)?;
    Ok(risk_free_rate + beta * market_risk_premium)
}

/// Calcula la prima de riesgo de mercado.
///
/// # Argumentos
/// * `market_return` - Rentabilidad esperada del mercado E(Rm)
/// * `risk_free_rate` - Tasa libre de riesgo (Rf)
///
/// # Fórmula
/// Prima de riesgo = E(Rm) - Rf
pub fn market_risk_premium(
    market_return: MarketReturn,
    risk_free_rate: RiskFreeRate,
) -> FinanceResult<MarketRiskPremium> {
    if !market_return.is_finite() || !risk_free_rate.is_finite() {
        return Err(FinanceError::InvalidInterestRate);
    }
    Ok(market_return - risk_free_rate)
}

/// Calcula el alpha de Jensen (rentabilidad anormal).
///
/// # Argumentos
/// * `actual_return` - Rentabilidad real observada
/// * `expected_return` - Rentabilidad esperada según CAPM
///
/// # Fórmula
/// α = Ri - E(Ri)
///
/// Un alpha positivo indica sobreperformance relativa al riesgo asumido
pub fn capm_alpha(
    actual_return: f64,
    expected_return: ExpectedReturn
) -> FinanceResult<f64> {
    if !actual_return.is_finite() || !expected_return.is_finite() {
        return Err(FinanceError::InvalidInterestRate);
    }
    Ok(actual_return - expected_return)
}

/// Validación de inputs para CAPM
fn validate_capm_inputs(
    risk_free_rate: RiskFreeRate,
    beta: Beta,
    market_risk_premium: MarketRiskPremium,
) -> FinanceResult<()> {
    if !risk_free_rate.is_finite() || !beta.is_finite() || !market_risk_premium.is_finite() {
        return  Err(FinanceError::InvalidInterestRate);
    }
    // Típicamente Rf >= 0, pero beta puede ser negativo
    if risk_free_rate < 0.0 {
        return Err(FinanceError::InvalidInterestRate);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expected_return_basic() {
        let rf = 0.03;
        let beta = 1.0;
        let market_premium = 0.07;

        let result = expected_return(rf, beta, market_premium).unwrap();
        assert!((result - 0.10).abs() < 1e-10);
    }

    #[test]
    fn test_expected_return_high_beta() {
        let rf = 0.02;
        let beta = 1.5;
        let market_premium = 0.08;

        let result = expected_return(rf, beta, market_premium).unwrap();
        assert!((result - 0.14).abs() < 1e-10); // 2% + 1.5 * 8% = 14%
    }

    #[test]
    fn test_expected_return_negative_beta() {
        let rf = 0.03;
        let beta = -0.5;
        let market_premium = 0.06;

        let result = expected_return(rf, beta, market_premium).unwrap();
        assert!((result - 0.0).abs() < 1e-10); // 3% + (-0.5) * 6% = 0%
    }

    #[test]
    fn test_market_risk_premium() {
        let market_return = 0.12;
        let risk_free = 0.04;

        let premium = market_risk_premium(market_return, risk_free).unwrap();
        assert!((premium - 0.08).abs() < 1e-10);
    }

    #[test]
    fn test_capm_alpha_positive() {
        let actual = 0.15;
        let expected = 0.12;

        let alpha = capm_alpha(actual, expected).unwrap();
        assert!((alpha - 0.03).abs() < 1e-10);
    }

    #[test]
    fn test_capm_alpha_negative() {
        let actual = 0.08;
        let expected = 0.11;

        let alpha = capm_alpha(actual, expected).unwrap();
        assert!((alpha - (-0.03)).abs() < 1e-10);
    }

    #[test]
    fn test_validate_negative_risk_free() {
        let result = expected_return(-0.01, 1.0, 0.05);
        assert!(matches!(result.unwrap_err(), FinanceError::InvalidInterestRate));
    }

    #[test]
    fn test_validate_infinite_inputs() {
        let result = expected_return(f64::INFINITY, 1.0, 0.05);
        assert!(matches!(result.unwrap_err(), FinanceError::InvalidInterestRate));
    }
}

//! Modelo Black-Scholes-Merton para opciones europeas (sin dividendos).
//!
//! Fórmulas implementadas
//! C = S0 N(d1) - K e^{-rT} N(d2)
//! P = K e^{-rT} N(-d2) - S0 N(-d1)
//! d1 = [ln(S0/K) + (r + σ^2/2) T] / (σ √T)
//! d2 = d1 - σ √T

use std::f64::consts::PI;
use crate::common::{
    FinanceError, FinanceResult,
    SpotPrice, StrikePrice, TimeToMaturity, InterestRate, Volatility,
    OptionType
};

const EPS_TIME: f64 = 1e-12;
const EPS_VOL: f64 = 1e-12;

/// PDF de la normal estándar φ(x)
#[inline]
fn normal_pdf(x: f64) -> f64 {
    (-0.5 * x * x).exp() / (2.0 * PI).sqrt()
}

/// CDF de la normal estándar N(x) (aprox. Abramowitz & Stegun 7.1.26).
#[inline]
fn normal_cdf(x: f64) -> f64 {
    let b1 = 0.319381530;
    let b2 = -0.356563782;
    let b3 = 1.781477937;
    let b4 = -1.821255978;
    let b5 = 1.330274429;
    let p = 0.2316419;

    let t = 1.0 / (1.0 + p * x.abs());
    let poly = ((((b5 * t + b4) * t + b3) * t + b2) * t + b1) * t;
    let approx = 1.0 - normal_pdf(x.abs()) * poly;
    if x >= 0.0 { approx } else { 1.0 - approx }
}

/// Validación básica de entradas con errores ya existentes.
#[inline]
fn validate_inputs(s0: SpotPrice, k: StrikePrice, t: TimeToMaturity, r: InterestRate, sigma: Volatility) -> FinanceResult<()> {
    if !s0.is_finite() || !k.is_finite() || !t.is_finite() || !r.is_finite() || !sigma.is_finite() {
        return Err(FinanceError::InvalidMonetaryValue);
    }
    if s0 <= 0.0 || k <= 0.0 {
        return Err(FinanceError::InvalidMonetaryValue);
    }
    if t < 0.0 {
        return Err(FinanceError::InvalidPeriods);
    }
    if sigma < 0.0 {
        return Err(FinanceError::InvalidInterestRate);
    }
    Ok(())
}

/// Calcula (d1, d2) de Black_Scholes.
pub fn d1_d2(s0: SpotPrice, k: StrikePrice, t: TimeToMaturity, r: InterestRate, sigma: Volatility) -> FinanceResult<(f64, f64)> {
    validate_inputs(s0, k, t, r, sigma)?;

    if t < EPS_TIME {
        let sign = (s0 - k).signum();
        let v = if sign > 0.0 { f64::INFINITY } else if sign < 0.0 { f64::NEG_INFINITY} else { 0.0 };
        return Ok((v, v));
    }
    if sigma < EPS_VOL {
        let num = (s0 / k).ln() + r * t;
        let v = if num > 0.0 { f64::INFINITY } else if num < 0.0 { f64::NEG_INFINITY } else { 0.0 };
        return Ok((v, v));
    }

    let ln_sk = (s0 / k).ln();
    let sqrt_t = t.sqrt();
    let sigma_sqrt_t = sigma * sqrt_t;
    let d1 = (ln_sk + (r + 0.5 * sigma * sigma) * t) / sigma_sqrt_t;
    let d2 = d1 - sigma_sqrt_t;
    Ok((d1, d2))
}

/// Precio de call europeo (sin dividendos).
pub fn call_price(s0: SpotPrice, k: StrikePrice, t: TimeToMaturity, r: InterestRate, sigma: Volatility) -> FinanceResult<f64> {
    validate_inputs(s0, k, t, r, sigma)?;

    if t < EPS_TIME {
        return Ok((s0 - k).max(0.0));
    }
    let (d1, d2) = d1_d2(s0, k, t, r, sigma)?;
    let df = (-r * t).exp();
    Ok(s0 * normal_cdf(d1) - k * df * normal_cdf(d2))
}

/// Precio de put europeo (sin dividendos).
pub fn put_price(s0: SpotPrice, k: StrikePrice, t: TimeToMaturity, r: InterestRate, sigma: Volatility) -> FinanceResult<f64> {
    validate_inputs(s0, k, t, r, sigma)?;

    if t < EPS_TIME {
        return Ok((k - s0).max(0.0));
    }
    let (d1, d2) = d1_d2(s0, k, t, r, sigma)?;
    let df = (-r * t).exp();
    Ok(k * df * normal_cdf(-d2) - s0 * normal_cdf(-d1))
}

/// Precio genérico según OptionType.
pub fn option_price(s0: SpotPrice, k: StrikePrice, t: TimeToMaturity, r: InterestRate, sigma: Volatility, kind: OptionType) -> FinanceResult<f64> {
    match kind {
        OptionType::Call => call_price(s0, k, t, r, sigma),
        OptionType::Put  => put_price(s0, k, t, r, sigma),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_call_parity() {
        let s0 = 100.0; let k = 110.0; let t = 0.25; let r = 0.05; let sigma = 0.20;
        let c = call_price(s0, k, t, r, sigma).unwrap();
        let p = put_price (s0, k, t, r, sigma).unwrap();
        let df = (-r * t).exp();
        assert!((c - p - (s0 - k * df)).abs() < 1e-10);
    }

    #[test]
    fn intrinsic_at_expiry() {
        let s0 = 120.0; let k = 110.0; let t = 1e-16; let r = 0.05; let sigma = 0.20;
        let c = call_price(s0, k, t, r, sigma).unwrap();
        let p = put_price (s0, k, t, r, sigma).unwrap();
        assert!((c - 10.0).abs() < 1e-10);
        assert!(p.abs() < 1e-10);
    }

    #[test]
    fn normal_pdf_at_zero() {
        let expected = 1.0 / (2.0 * PI).sqrt();
        assert!((normal_pdf(0.0) - expected).abs() < 1e-12);
    }

    #[test]
    fn normal_cdf_symmetry() {
        let value = normal_cdf(1.0);
        assert!((value - (1.0 - normal_cdf(-1.0))).abs() < 1e-9);
        assert!((value - 0.841344746).abs() < 1e-6);
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn d1_d2_regular_case() {
        let (d1, d2) = d1_d2(100.0, 100.0, 1.0, 0.05, 0.20).unwrap();
        assert!((d1 - 0.35).abs() < 1e-12);
        assert!((d2 - 0.15).abs() < 1e-12);
    }

    #[test]
    fn d1_d2_zero_vol_limit() {
        let (d1, d2) = d1_d2(110.0, 100.0, 1.0, 0.05, 0.0).unwrap();
        assert!(d1.is_infinite() && d1.is_sign_positive());
        assert!(d2.is_infinite() && d2.is_sign_positive());
    }

    #[test]
    fn zero_vol_call_put_prices() {
        let call = call_price(110.0, 100.0, 1.0, 0.05, 0.0).unwrap();
        let expected_call = 110.0 - 100.0 * (-0.05f64).exp();
        assert!((call - expected_call).abs() < 1e-12);

        let put = put_price(90.0, 100.0, 1.0, 0.05, 0.0).unwrap();
        let expected_put = 100.0 * (-0.05f64).exp() - 90.0;
        assert!((put - expected_put).abs() < 1e-12);
    }

    #[test]
    fn option_price_dispatch() {
        let call = option_price(100.0, 100.0, 0.5, 0.02, 0.25, OptionType::Call).unwrap();
        let put = option_price(100.0, 100.0, 0.5, 0.02, 0.25, OptionType::Put).unwrap();
        let direct_call = call_price(100.0, 100.0, 0.5, 0.02, 0.25).unwrap();
        let direct_put = put_price(100.0, 100.0, 0.5, 0.02, 0.25).unwrap();
        assert!((call - direct_call).abs() < 1e-12);
        assert!((put - direct_put).abs() < 1e-12);
    }

    #[test]
    fn validate_inputs_errors() {
        let err_money = validate_inputs(-1.0, 100.0, 1.0, 0.01, 0.2).unwrap_err();
        assert!(matches!(err_money, FinanceError::InvalidMonetaryValue));

        let err_time = validate_inputs(100.0, 100.0, -0.5, 0.01, 0.2).unwrap_err();
        assert!(matches!(err_time, FinanceError::InvalidPeriods));

        let err_sigma = validate_inputs(100.0, 100.0, 0.5, 0.01, -0.1).unwrap_err();
        assert!(matches!(err_sigma, FinanceError::InvalidInterestRate));
    }
}
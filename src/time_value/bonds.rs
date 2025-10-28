//! Cálculos de duración y convexidad para bonos
//!
//! Implementa las fórmulas de duración de Macaulay y convexidad para el análisis
//! de sensibilidad de precios de bonos ante cambios ens las tasas de interés

use crate::common::{FinanceResult, FinanceError, InterestRate, MonetaryValue, Periods};

/// Estructura para representar los flujos de cada de un bono
#[derive(Debug, Clone)]
pub struct  BondCashFlow {
    /// Período en el que se recibe el flujo de caja
    pub period: f64,
    /// Monto del flujo de caja
    pub amount: MonetaryValue,
}

/// Estructura para representar un bono
#[derive(Debug, Clone)]
pub struct Bond {
    /// Flujos de caja del bono (cupones y principal)
    pub cash_flows: Vec<BondCashFlow>,
    /// Rendimiento al vencimiento (yield to maturity)
    pub ytm: InterestRate,
}

impl Bond {
    /// Crea un nuevo bono con cupones regulares
    ///
    /// # Argumentos
    /// * `face_value` - Valor nominal del bono
    /// * `coupon_rate` - Tasa de cupón anual
    /// * `periods` - Número de períodods hasta el vencimiento
    /// * `ytm` - Rendimiento al vencimiento
    ///
    /// # Ejemplos
    /// ```
    /// use quant_finance::time_value::bonds::Bond;
    ///
    /// let Bond = Bond::new_coupon(1000.0, 0.05, 10, 0.04).unwrap();
    /// ```
    pub fn new_coupon_bond(
        face_value: MonetaryValue,
        coupon_rate: InterestRate,
        periods: Periods,
        ytm: InterestRate,
    ) -> FinanceResult<Self> {
        if face_value <= 0.0 {
            return Err(FinanceError::InvalidMonetaryValue);
        }

        if coupon_rate < 0.0 || ytm <= -1.0 {
            return Err(FinanceError::InvalidInterestRate);
        }

        let coupon_payment = face_value * coupon_rate;
        let mut cash_flows = Vec::new();

        // Agregar cupones
        for t in 1..=periods {
            cash_flows.push(BondCashFlow {
                period: t as f64,
                amount: coupon_payment,
            });
        }

        // Agregar principal en el último período
        if let Some(last_flow) = cash_flows.last_mut() {
            last_flow.amount += face_value;
        }

        Ok(Bond { cash_flows, ytm })
    }
}

/// Calcula el precio de mercado del bono
///
/// # Argumentos
/// * `bond` - Referencia al bono
///
/// # Fórmula
/// P = Σ(CFₜ / (1 + y)ᵗ)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::bonds::{Bond, bond_price};
///
/// let bond = Bond::new_coupon_bond(1000.0, 0.05, 5, 0.04).unwrap();
/// let price = bond_price(&bond).unwrap();
/// ```
pub fn bond_price(bond: &Bond) -> FinanceResult<MonetaryValue> {
    if bond.ytm <= -1.0 {
        return Err(FinanceError::InvalidInterestRate);
    }

    let mut price = 0.0;

    for cash_flow in &bond.cash_flows {
        let discount_factor = (1.0 + bond.ytm).powf(-cash_flow.period);
        price += cash_flow.amount * discount_factor;
    }

    Ok(price)
}

/// Calcula la duración de Macaulay de un bono
///
/// # Fórmula
/// Duración = Σ(PV(CFₜ) * t) / Precio del Bono
///
/// Donde PV(CFₜ) = CFₜ / (1 + Y)ᵗ
///
/// # Argumentos
/// * `bond` - Referencia al bono
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::bonds::{Bond, macaulay_duration};
///
/// let bond = Bond::new_coupon_bond(1000.0, 0.05, 10, 0.04).unwrap();
/// let duration = macaulay_duration(&bond).unwrap();
/// ```
///
/// # Errors
/// * `InvalidInterestRate` - Si el YTM es inválido
/// * `DivisionByZero` - Si el precio del bono es cero
pub fn macaulay_duration(bond: &Bond) -> FinanceResult<f64> {
    let market_price = bond_price(bond)?;

    if market_price == 0.0 {
        return Err(FinanceError::DivisionByZero);
    }

    let mut weighted_time_pv = 0.0;

    for cash_flow in &bond.cash_flows {
        let discount_factor = (1.0 + bond.ytm).powf(-cash_flow.period);
        let present_value = cash_flow.amount * discount_factor;
        weighted_time_pv += present_value * cash_flow.period;
    }

    Ok(weighted_time_pv / market_price)
}

/// Calcula la convexidad de un bono
///
/// # Fórmula
/// Convexidad = [1 / (P × (1 + y)²)] × Σ[CFₜ/(1 + y)ᵗ × (t² + t)]
///
/// # Argumentos
/// * `bond` - Referencia al bono
///
/// * Ejemplos
/// ```
/// use quant_finance::time_value::bonds::{Bond, bond_convexity};
///
/// let bond = Bond::new_coupon_bond(1000.0, 0.05, 10, 0.04).unwrap();
/// let convexity = bond_convexity(&bond).unwrap();
/// ```
///
/// # Errores
/// * `InvalidInterestRate` - Si el YTM es inválido
/// * `DivisionByZero` - Si el precio del bono es cero
pub fn bond_convexity(bond: &Bond) -> FinanceResult<f64> {
    let market_price = bond_price(bond)?;

    if market_price == 0.0 {
        return Err(FinanceError::DivisionByZero);
    }

    let mut convexity_sum = 0.0;
    let yield_factor = 1.0 + bond.ytm;

    for cash_flow in &bond.cash_flows {
        let discount_factor = yield_factor.powf(-cash_flow.period);
        let time_factor = cash_flow.period * cash_flow.period + cash_flow.period;
        convexity_sum += cash_flow.amount * discount_factor * time_factor;
    }

    Ok(convexity_sum / (market_price * yield_factor * yield_factor))
}

/// Calcula el ajuste de convexidad
///
/// # Fórmula
/// Ajuste de Convexidad = Convexidad × 100 × (Δy)²
///
/// # Argumentos
/// * `convexity` - Convexidad del bono
/// * `yield_change` - Cambio en el rendimiento (Δy)
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::bonds::{Bond, bond_convexity, convexity_adjustment};
///
/// let bond = Bond::new_coupon_bond(1000.0, 0.05, 10, 0.04).unwrap();
/// let convexity = bond_convexity(&bond).unwrap();
/// let adjustment = convexity_adjustment(convexity, 0.01).unwrap();
/// ```
pub fn convexity_adjustment(convexity: f64, yield_change: f64) -> FinanceResult<f64> {
    if convexity < 0.0 {
        return Err(FinanceError::InvalidInput);
    }

    Ok(convexity * yield_change * yield_change)
}

/// Calcula la duración modificada
///
/// # Fórmula
/// Duración Modificada = Duración de Macaulay / (1 + YTM)
///
/// # Argumentos
/// * `bond` - Referencia al bono
///
/// # Ejemplos
/// ```
/// use quant_finance::time_value::bonds::{Bond, modified_duration};
///
/// let bond = Bond::new_coupon_bond(1000.0, 0.05, 10, 0.04).unwrap();
/// let mod_duration = modified_duration(&bond).unwrap();
/// ```
pub fn modified_duration(bond: &Bond) -> FinanceResult<f64> {
    let mac_duration = macaulay_duration(bond)?;
    Ok(mac_duration / (1.0 + bond.ytm))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_bond() -> Bond {
        Bond::new_coupon_bond(1000.0, 0.05, 5, 0.04).unwrap()
    }

    #[test]
    fn test_bond_creation() {
        let bond = create_test_bond();
        assert_eq!(bond.cash_flows.len(), 5);
        assert_eq!(bond.ytm, 0.04);

        // Verificar cupones
        for i in 0..4 {
            assert_eq!(bond.cash_flows[i].amount, 50.0);
            assert_eq!(bond.cash_flows[i].period, (i + 1) as f64);
        }

        // Último flujo debe incluir principal
        assert_eq!(bond.cash_flows[4].amount, 1050.0);
    }

    #[test]
    fn test_bond_price() {
        let bond = create_test_bond();
        let price = bond_price(&bond).unwrap();

        // Para un bono con cupón 5% y YTM 4%, el precio debe ser superior al par
        assert!(price > 1000.0);
        assert!((price - 1044.52).abs() < 0.1);
    }

    #[test]
    fn test_macaulay_duration() {
        let bond = create_test_bond();
        let duration = macaulay_duration(&bond).unwrap();

        // La duración debe ser menor al vencimiento para bonos con cupón
        assert!(duration > 0.0);
        assert!(duration < 5.0);
        assert!((duration - 4.557).abs() < 0.01);
    }

    #[test]
    fn test_bond_convexity() {
        let bond = create_test_bond();
        let convexity = bond_convexity(&bond).unwrap();

        // La convexidad debe ser positiva
        assert!(convexity > 0.0);
        // Valor calculado: aproximadamente 24.48
        assert!((convexity - 24.48).abs() < 0.1);
    }

    #[test]
    fn test_modified_duration() {
        let bond = create_test_bond();
        let mac_duration = macaulay_duration(&bond).unwrap();
        let mod_duration = modified_duration(&bond).unwrap();

        // Duración modificada = Duración Macaulay / (1 + YTM)
        let expected = mac_duration / (1.0 + bond.ytm);
        assert!((mod_duration - expected).abs() < 1e-10);
    }

    #[test]
    fn test_convexity_adjustment() {
        // Usando la fórmula sin factor 100: Convexidad × (Δy)²
        let adjustment = convexity_adjustment(20.0, 0.01).unwrap();
        assert!((adjustment - 0.002).abs() < 1e-6);
    }

    #[test]
    fn test_zero_coupon_bond() {
        let bond = Bond::new_coupon_bond(1000.0, 0.0, 10, 0.05).unwrap();

        // Para un bono cupón cero, la duración debe ser igual al vencimiento
        let duration = macaulay_duration(&bond).unwrap();
        assert!((duration - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_inputs() {
        assert!(Bond::new_coupon_bond(-1000.0, 0.05, 5, 0.04).is_err());
        assert!(Bond::new_coupon_bond(1000.0, -0.05, 5, 0.04).is_err());
        assert!(Bond::new_coupon_bond(1000.0, 0.05, 5, -2.0).is_err());
    }

    #[test]
    fn test_convexity_with_different_bond() {
        // Test con un bono diferente para verificar la fórmula
        let bond = Bond::new_coupon_bond(1000.0, 0.08, 3, 0.06).unwrap();
        let convexity = bond_convexity(&bond).unwrap();

        // Este bono debería tener menor convexidad (menor vencimiento)
        assert!(convexity > 0.0);
        assert!(convexity < 15.0); // Esperamos menor convexidad que el bono de 5 años
    }
}

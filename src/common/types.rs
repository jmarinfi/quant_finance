//! Tipos comunes utilizados en toda la biblioteca

/// Representa una tasa de interés como decimal (ej: 0.05 para 5%)
pub type InterestRate = f64;

/// Representa un valor monetario
pub type MonetaryValue = f64;

/// Representa un número de períodos
pub type Periods = u32;

/// Errores comunes en cálculos financieros
#[derive(Debug, PartialEq)]
pub enum FinanceError {
    /// Tasa de interés negativa o inválida
    InvalidInterestRate,
    /// Número de períodos inválido
    InvalidPeriods,
    /// Valor monetario negativo donde no es permitido
    InvalidMonetaryValue,
    /// División por cero en cálculos
    DivisionByZero,
}

impl std::fmt::Display for FinanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FinanceError::InvalidInterestRate => write!(f, "Tasa de interés inválida"),
            FinanceError::InvalidPeriods => write!(f, "Número de períodos inválido"),
            FinanceError::InvalidMonetaryValue => write!(f, "Valor monetario inválido"),
            FinanceError::DivisionByZero => write!(f, "División por cero en cálculo financiero"),
        }
    }
}

impl std::error::Error for FinanceError {}

/// Tipo de resultado para operaciones financieras
pub type FinanceResult<T> = Result<T, FinanceError>;

/// Representa el número de veces que se capitaliza por período
pub type CompoundingFrequency = u32;

/// Representa el tiempo en años (puede ser fraccionario)
pub type TimeInYears = f64;

/// Representa el principal o capital inicial
pub type Principal = f64;

/// Representa el interés ganado
pub type Interest = f64;
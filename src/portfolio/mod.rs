///! Teoría de portafolios y modelos de valoración de activos
pub mod capm;

pub use capm::{
    capm_alpha, expected_return, market_risk_premium,
};

//! Módulo de valor temporal del dinero
//!
//! Contiene funciones para calcular valores presentes, futuros, interés simple,
//! interés compuesto y otros conceptos relacionados con el valor del dinero en el tiempo.

pub mod basic;
pub mod simple;
pub mod compound;

// Re-exportar funciones principales
pub use basic::{future_value, present_value};
pub use simple::{
    simple_interest, simple_interest_amount,
    simple_interest_principal, simple_interest_rate
};
pub use compound::{
    compound_amount, compound_interest, compound_principal,
    continuous_compound_amount, effective_annual_rate
};

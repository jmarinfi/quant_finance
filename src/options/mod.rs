//! Valoración de opciones europeas
//!
//! Este módulo agrupa implementaciones relacionadas con opciones,
//! empezando por el modelo de Black-Scholes-Merton (sin dividendos).

pub mod black_scholes;

pub use black_scholes::{
    call_price,
    put_price,
    option_price,
    d1_d2,
};
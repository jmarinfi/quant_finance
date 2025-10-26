# quant_finance

Biblioteca de Rust para implementar fórmulas y algoritmos del libro "Quantitative Finance" de X. Y. Wang.

Este crate aspira a ser una colección clara, testeada y bien documentada de herramientas cuantitativas: desde estadística aplicada y procesos estocásticos, hasta valoración de derivados, gestión de carteras y medición de riesgo. La meta es ofrecer implementaciones transparentes, reproducibles y orientadas a aprendizaje e investigación.

Estado: inicial (scaffolding). Se añadirán módulos y funciones de forma incremental, junto con documentación y tests.

## Instalación

Aún no publicado en crates.io. Por ahora puedes:

- Usarlo vía ruta local (trabajando dentro de este repo):

```toml
# Cargo.toml de tu proyecto
[dependencies]
quant_finance = { path = "../quant_finance" }
```

- O, cuando se publique, bastará con:

```toml
[dependencies]
quant_finance = "0.1"
```

> Nota: actualiza la versión según corresponda.

## Uso rápido

Ejemplo mínimo con la API actual (de momento solo hay funciones de ejemplo mientras se construye la biblioteca):

```rust
use quant_finance::add;

fn main() {
    assert_eq!(add(2, 2), 4);
}
```

A medida que se incorporen módulos nuevos, este apartado mostrará ejemplos prácticos por tema (estadística, renta fija, opciones, etc.).

## Alcance previsto (roadmap)

La estructura exacta puede variar, pero se prevé algo como:

- estadistica: descriptivos, momentos, estimadores, pruebas, regresión básica.
- procesos: caminatas aleatorias, BM/GBM, OU, simulación Monte Carlo.
- renta_fija: tasas spot/forward, descuento, duración/convexidad, construcción de curvas.
- opciones: Black–Scholes–Merton, griegas, paridad put–call, árboles binomiales/trinómicos.
- tasa_interes: modelos de corto plazo (Vasicek, CIR), HJM, valoración de bonos.
- cartera: media-varianza, fronteras eficientes, CAPM, backtesting sencillo.
- riesgo: VaR/ES paramétrico y simulado, backtesting de riesgo, agregación.
- num: herramientas numéricas (interpolación, integración, optimización).

Cada módulo incluirá:

- Documentación con fórmulas y referencias.
- Tests unitarios y, cuando tenga sentido, tests estocásticos basados en propiedades.
- Ejemplos reproducibles.

## Filosofía de diseño

- Correctitud primero: tests y validaciones junto a cada implementación.
- Claridad didáctica: priorizar legibilidad y referencias sobre micro-optimizaciones prematuras.
- Trazabilidad: enlazar fórmulas y resultados con su origen (capítulo/sección del libro u otras fuentes abiertas).
- Reproducibilidad: fijar semillas y detallar supuestos cuando se use aleatoriedad.

## Desarrollo

- Compilar:

```bash
cargo build
```

- Tests (incluye pruebas de documentación):

```bash
cargo test
```

- Documentación local:

```bash
cargo doc --open
```

- Formato y lint (si instalas rustfmt y clippy):

```bash
cargo fmt
cargo clippy -- -D warnings
```

## Contribuir

Toda contribución es bienvenida. Recomendaciones:

1. Abre un issue para discutir nuevas funciones o discrepancias con las fórmulas del libro.
2. Acompaña cada PR con tests, referencias y notas sobre supuestos.
3. Mantén las APIs pequeñas y bien tipadas; documenta entradas/salidas y unidades.

## Licencia

Por definir. Antes de publicar, se añadirá un archivo de licencia (p. ej., MIT/Apache-2.0). Este proyecto no distribuye contenido con copyright del libro; únicamente implementa fórmulas y algoritmos a partir de referencias académicas y apuntes propios.

## Referencias

- X. Y. Wang, "Quantitative Finance" (referencia principal de alcance y organización del contenido).
- Notas de curso y artículos abiertos que se citarán en cada módulo cuando corresponda.

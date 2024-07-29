pub mod ecliptic_position;
pub mod equatorial_position;
pub mod rectangular_position;

pub use ecliptic_position::*;
pub use equatorial_position::*;
pub use rectangular_position::*;

#[derive(Debug)]
pub enum CalcResult {
    _EclipticPosition(EclipticPosition),
    _EquatorialPosition(EquatorialPosition),
    _RectangularPosition(RectangularPosition),
}


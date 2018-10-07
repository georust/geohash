use ::Coordinate;

#[derive(Debug, Fail)]
pub enum GeohashError {
    #[fail(display = "invalid hash character: {}", character)]
    InvalidHashCharacter { character: char },
    #[fail(display = "invalid coordinate range: {:?}", c)]
    InvalidCoordinateRange { c: Coordinate<f64> },
}

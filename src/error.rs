#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    NoBatteries,
    TooManyBatteries,
    UnknownBatteryId,

    Unknown,

    #[from]
    Battery(battery::Error),
}

impl std::error::Error for Error {}

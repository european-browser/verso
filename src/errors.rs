/// Convenient type alias of Result type for Verso.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by Verso.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The error type for when the OS cannot perform the requested operation.
    #[error(transparent)]
    OsError(#[from] winit::error::OsError),
    /// A general error that may occur while running the Winit event loop.
    #[error(transparent)]
    EventLoopError(#[from] winit::error::EventLoopError),
    /// Error related to OpenGL shader compilation.
    #[error("Failed to compiler shader in painter module.")]
    CompileShader,
}

#[cfg(feature = "lpc802")]
pub mod syscon_lpc802;
#[cfg(feature = "lpc804")]
pub mod syscon_lpc804;
#[cfg(feature = "lpc81x")]
pub mod syscon_lpc81x;
#[cfg(feature = "lpc82x")]
pub mod syscon_lpc82x;
#[cfg(feature = "lpc83x")]
pub mod syscon_lpc83x;
#[cfg(feature = "lpc84x")]
pub mod syscon_lpc84x;

#[cfg(feature = "lpc802")]
pub use self::syscon_lpc802 as syscon_variant;
#[cfg(feature = "lpc804")]
pub use self::syscon_lpc804 as syscon_variant;
#[cfg(feature = "lpc81x")]
pub use self::syscon_lpc81x as syscon_variant;
#[cfg(feature = "lpc82x")]
pub use self::syscon_lpc82x as syscon_variant;
#[cfg(feature = "lpc83x")]
pub use self::syscon_lpc83x as syscon_variant;
#[cfg(feature = "lpc84x")]
pub use self::syscon_lpc84x as syscon_variant;

pub use self::syscon_variant::*;

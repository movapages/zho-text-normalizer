//! Text normalization components

pub mod compatibility_normalizer;
pub mod kangxi_normalizer;
pub mod script_converter;
pub mod script_detector;
pub mod text_normalizer;
pub mod unicode_normalizer;
pub mod variant_normalizer;

pub use compatibility_normalizer::CompatibilityNormalizer;
pub use kangxi_normalizer::KangxiNormalizer;
pub use script_converter::ScriptConverter;
pub use script_detector::ScriptDetector;
pub use text_normalizer::TextNormalizer;
pub use unicode_normalizer::UnicodeNormalizer;
pub use variant_normalizer::VariantNormalizer;

//! OpenCC validator for Traditional ↔ Simplified conversion

use opencc::OpenCC;

/// OpenCC validator for script conversion
pub struct OpenCCValidator {
    trad_to_simp: OpenCC,
    simp_to_trad: OpenCC,
}

impl OpenCCValidator {
    /// Create a new OpenCC validator
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Traditional to Simplified
        let trad_to_simp = OpenCC::new("t2s");

        // Simplified to Traditional
        let simp_to_trad = OpenCC::new("s2t");

        Ok(Self {
            trad_to_simp,
            simp_to_trad,
        })
    }

    /// Convert Traditional to Simplified using OpenCC
    pub fn traditional_to_simplified(
        &self,
        text: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.trad_to_simp.convert(text))
    }

    /// Convert Simplified to Traditional using OpenCC
    pub fn simplified_to_traditional(
        &self,
        text: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        Ok(self.simp_to_trad.convert(text))
    }
}

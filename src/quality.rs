use crate::Error;

/// Quality of the output mesh. Higher quality produces more vertices and takes longer
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Quality {
    /// 10
    Low,
    /// 20
    Medium,
    /// 50
    High,
    /// Custom quality. Clipped between 8 and 128
    Custom(u8),
}

impl Default for Quality {
    fn default() -> Self {
        Quality::Medium
    }
}

impl Quality {
    pub fn from_str(val: &str) -> Result<Self, Error> {
        let val = match u8::from_str_radix(val, 10) {
            Ok(val) => val,
            Err(_) => {
                return Ok(match val {
                    "low" => Quality::Low,
                    "medium" => Quality::Medium,
                    "high" => Quality::High,
                    _ => return Err(Error::QualityParse(val.into())),
                });
            }
        };

        Ok(match val {
            10 => Quality::Low,
            20 => Quality::Medium,
            50 => Quality::High,
            _ => Quality::Custom(val),
        })
    }

    pub(crate) fn as_u8(&self) -> u8 {
        match self {
            Quality::Low => 10,
            Quality::Medium => 20,
            Quality::High => 50,
            Quality::Custom(value) => *value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(Quality::from_str("low").unwrap(), Quality::Low);
        assert_eq!(Quality::from_str("medium").unwrap(), Quality::Medium);
        assert_eq!(Quality::from_str("255").unwrap(), Quality::Custom(255));
        assert!(Quality::from_str("1000").is_err());
        assert!(Quality::from_str("Medium").is_err());
    }
}

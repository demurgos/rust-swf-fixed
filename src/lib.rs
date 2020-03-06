#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

macro_rules! fixed_point_impl {
  ($name:ident, $int_bits:expr, $frac_bits:expr, $epsilons_type:ty, $value_type:ty) => {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct $name {
      pub epsilons: $epsilons_type,
    }

    impl $name {
      pub const ZERO: Self = Self { epsilons: 0 };
      pub const ONE: Self = Self {
        epsilons: 1 << $frac_bits,
      };

      pub const fn from_epsilons(epsilons: $epsilons_type) -> $name {
        $name { epsilons: epsilons }
      }

      pub fn from_value(value: $value_type) -> $name {
        // TODO: Checked cast
        let epsilons: $epsilons_type = (value * ((1 << $frac_bits) as $value_type)) as $epsilons_type;
        $name { epsilons: epsilons }
      }
    }

    impl From<$name> for $value_type {
      fn from(fixed: $name) -> $value_type {
        (fixed.epsilons as $value_type) / ((1 << $frac_bits) as $value_type)
      }
    }

    impl ::std::ops::Add<$name> for $name {
      type Output = $name;

      fn add(self, rhs: $name) -> $name {
        &self + &rhs
      }
    }

    impl<'a, 'b> ::std::ops::Add<&'b $name> for &'a $name {
      type Output = $name;

      fn add(self, rhs: &'b $name) -> $name {
        $name {
          epsilons: self.epsilons + rhs.epsilons,
        }
      }
    }

    impl ::std::ops::AddAssign<$name> for $name {
      fn add_assign(&mut self, rhs: $name) -> () {
        self.add_assign(&rhs)
      }
    }

    impl<'a> ::std::ops::AddAssign<&'a $name> for $name {
      fn add_assign(&mut self, rhs: &'a $name) -> () {
        self.epsilons.add_assign(rhs.epsilons)
      }
    }

    #[cfg(feature = "serde")]
    impl Serialize for $name {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
        S: Serializer,
      {
        self.epsilons.serialize(serializer)
      }
    }

    #[cfg(feature = "serde")]
    impl<'a> Deserialize<'a> for $name {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
      where
        D: Deserializer<'a>,
      {
        Ok($name::from_epsilons(<$epsilons_type>::deserialize(deserializer)?))
      }
    }
  };
}

fixed_point_impl!(Sfixed8P8, 8, 8, i16, f32);
fixed_point_impl!(Sfixed16P16, 16, 16, i32, f64);
fixed_point_impl!(Ufixed8P8, 8, 8, u16, f32);
fixed_point_impl!(Ufixed16P16, 16, 16, u32, f64);

#[cfg(test)]
mod tests {
  #[cfg(feature = "serde")]
  use serde_json;

  use crate::{Sfixed16P16, Sfixed8P8, Ufixed16P16, Ufixed8P8};

  #[test]
  fn test_eq() {
    assert_eq!(Sfixed16P16::from_epsilons(3), Sfixed16P16::from_epsilons(3));
  }

  #[test]
  fn test_default() {
    const DEFAULT_SFIXED16P16: Sfixed16P16 = Sfixed16P16::from_epsilons(0);
    const DEFAULT_SFIXED8P8: Sfixed8P8 = Sfixed8P8::from_epsilons(0);
    const DEFAULT_UFIXED16P16: Ufixed16P16 = Ufixed16P16::from_epsilons(0);
    const DEFAULT_UFIXED8P8: Ufixed8P8 = Ufixed8P8::from_epsilons(0);
    assert_eq!(Sfixed16P16::default(), DEFAULT_SFIXED16P16);
    assert_eq!(Sfixed8P8::default(), DEFAULT_SFIXED8P8);
    assert_eq!(Ufixed16P16::default(), DEFAULT_UFIXED16P16);
    assert_eq!(Ufixed8P8::default(), DEFAULT_UFIXED8P8);
  }

  #[test]
  fn test_zero() {
    assert_eq!(Sfixed16P16::ZERO, Sfixed16P16::from_epsilons(0));
    assert_eq!(Sfixed8P8::ZERO, Sfixed8P8::from_epsilons(0));
    assert_eq!(Ufixed16P16::ZERO, Ufixed16P16::from_epsilons(0));
    assert_eq!(Ufixed8P8::ZERO, Ufixed8P8::from_epsilons(0));
  }

  #[test]
  fn test_one() {
    assert_eq!(Sfixed16P16::ONE, Sfixed16P16::from_epsilons(65536));
    assert_eq!(Sfixed8P8::ONE, Sfixed8P8::from_epsilons(256));
    assert_eq!(Ufixed16P16::ONE, Ufixed16P16::from_epsilons(65536));
    assert_eq!(Ufixed8P8::ONE, Ufixed8P8::from_epsilons(256));
  }

  #[test]
  #[cfg(feature = "serde")]
  fn test_json_serde_serialization() {
    assert_eq!(serde_json::to_string(&Sfixed16P16::from_epsilons(3)).unwrap(), "3");
  }

  #[test]
  #[cfg(feature = "serde")]
  fn test_json_serde_deserialization() {
    assert_eq!(
      serde_json::from_str::<Sfixed16P16>("0").unwrap(),
      Sfixed16P16::from_epsilons(0)
    );
    assert_eq!(
      serde_json::from_str::<Sfixed16P16>("3").unwrap(),
      Sfixed16P16::from_epsilons(3)
    );
    assert_eq!(
      serde_json::from_str::<Sfixed16P16>("65536").unwrap(),
      Sfixed16P16::from_epsilons(65_536)
    );
    assert_eq!(
      serde_json::from_str::<Sfixed16P16>("2147483647").unwrap(),
      Sfixed16P16::from_epsilons(2_147_483_647)
    );
    assert_eq!(
      serde_json::from_str::<Sfixed16P16>("-2147483648").unwrap(),
      Sfixed16P16::from_epsilons(-2_147_483_648)
    );
  }

  #[test]
  fn test_ufixed8p8() {
    assert_eq!(Ufixed8P8::from_value(24f32).epsilons, 6144);
  }

  #[test]
  fn test_ufixed8p8_value() {
    assert_eq!(
      f32::from(Ufixed8P8::from_value(24f32)).to_ne_bytes(),
      24f32.to_ne_bytes()
    );
    assert_eq!(
      f32::from(Ufixed8P8::from_value(255f32)).to_ne_bytes(),
      255f32.to_ne_bytes()
    );
  }

  #[test]
  fn test_sfixed8p8_value() {
    assert_eq!(
      f32::from(Sfixed8P8::from_value(-24f32)).to_ne_bytes(),
      (-24f32).to_ne_bytes()
    );
  }

  #[test]
  fn test_ufixed16p16_value() {
    assert_eq!(
      f64::from(Ufixed16P16::from_value(1000f64)).to_ne_bytes(),
      1000f64.to_ne_bytes()
    );
  }

  #[test]
  fn test_add_ufixed8p8() {
    assert_eq!(
      Ufixed8P8::from_value(0f32) + Ufixed8P8::from_value(0f32),
      Ufixed8P8::from_value(0f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(0f32) + Ufixed8P8::from_value(1f32),
      Ufixed8P8::from_value(1f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(1f32) + Ufixed8P8::from_value(0f32),
      Ufixed8P8::from_value(1f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(1f32) + Ufixed8P8::from_value(1f32),
      Ufixed8P8::from_value(2f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(0.5f32) + Ufixed8P8::from_value(0.5f32),
      Ufixed8P8::from_value(1f32)
    );
  }

  #[test]
  fn test_add_ufixed8p8_ref() {
    assert_eq!(
      Ufixed8P8::from_value(0f32) + Ufixed8P8::from_value(0f32),
      Ufixed8P8::from_value(0f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(0f32) + Ufixed8P8::from_value(1f32),
      Ufixed8P8::from_value(1f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(1f32) + Ufixed8P8::from_value(0f32),
      Ufixed8P8::from_value(1f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(1f32) + Ufixed8P8::from_value(1f32),
      Ufixed8P8::from_value(2f32)
    );
    assert_eq!(
      Ufixed8P8::from_value(0.5f32) + Ufixed8P8::from_value(0.5f32),
      Ufixed8P8::from_value(1f32)
    );
  }
}

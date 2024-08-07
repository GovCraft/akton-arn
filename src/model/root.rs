use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

use derive_more::{AsRef, From, Into};
use type_safe_id::{DynamicType, TypeSafeId};

use crate::{IdType, Timestamp, UnixTime};
use crate::errors::ErnError;

#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, PartialOrd)]
pub struct Root<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash = UnixTime> {
    pub(crate) name: Cow<'static, str>,
    marker: std::marker::PhantomData<T>,
}
impl Ord for Root<Timestamp> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}
impl Ord for Root<UnixTime> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> Root<T> {
    pub fn as_str(&self) -> &str {
        &self.name
    }

    pub fn into_owned(self) -> Root<T> {
        Root {
            name: Cow::Owned(self.name.into_owned()),
            marker: Default::default(),
        }
    }

    pub fn new(value: impl Into<Cow<'static, str>>) -> Result<Self, ErnError> {
        let value = value.into();
        let value = if value.is_empty() {
            let val = ACTON;
            TypeSafeId::from_type_and_uuid(DynamicType::new(val)?, T::generate_id(val)).to_string()
        } else {
            TypeSafeId::from_type_and_uuid(
                DynamicType::new(&value)?,
                T::generate_id(value.as_ref()),
            )
            .to_string()
        };
        Ok(Root {
            name: Cow::from(value),
            marker: Default::default(),
        })
    }
}

impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> Default for Root<T> {
    fn default() -> Self {
        Root::new("").expect("Couldn't create default Acton Ern")
    }
}

impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> fmt::Display for Root<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = &self.name;
        write!(f, "{id}")
    }
}
const ACTON: &str = "acton";

impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> std::str::FromStr for Root<T> {
    type Err = ErnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Root {
            name: Cow::from(s.to_string()),
            marker: Default::default(),
        })
    }
}

impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> From<Root<T>> for String {
    fn from(root: Root<T>) -> Self {
        root.name.into_owned()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_creation() {
        let root: Root<UnixTime> = Root::new("test").unwrap();
        assert!(root.as_str().starts_with("test"));
    }

    #[test]
    fn test_root_default() {
        let root: Root<UnixTime> = Root::default();
        assert!(root.as_str().starts_with("acton"));
    }

    #[test]
    fn test_root_display() {
        let root: Root<UnixTime> = Root::new("example").unwrap();
        assert!(format!("{}", root).starts_with("example"));
    }

    #[test]
    fn test_root_from_str() {
        let root: Root<UnixTime> = "test".parse().unwrap();
        assert!(root.as_str().starts_with("test"));
    }

    #[test]
    fn test_root_equality() -> Result<(), ErnError> {
        let root1: Root<UnixTime> = Root::new("test")?;
        let root2: Root<UnixTime> = Root::new("test")?;
        let root3: Root<UnixTime> = Root::new("other")?;
        assert_ne!(root1, root2);
        assert_ne!(root1, root3);
        Ok(())
    }

    #[test]
    fn test_root_into_string() {
        let root: Root<UnixTime> = Root::new("test").unwrap();
        let string: String = root.into();
        assert!(string.starts_with("test"));
    }
}

use std::borrow::Cow;
use std::hash::Hash;

use crate::{IdType, Root};
use crate::errors::ErnError;
use crate::model::{Account, Category, Domain, Ern, Part, Parts};
use crate::traits::ErnComponent;

/// A builder for constructing ERN (Entity Resource Name) instances using a state-driven approach with type safety.
pub struct ErnBuilder<State, T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> {
    builder: PrivateErnBuilder<T>,
    _marker: std::marker::PhantomData<(State, T)>,
}

/// Implementation of `ErnBuilder` for the initial state, starting with `Domain`.
impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> ErnBuilder<(), T> {
    /// Creates a new ERN (Entity Resource Name) builder initialized to start building from the `Domain` component.
    pub fn new() -> ErnBuilder<Domain, T> {
        ErnBuilder {
            builder: PrivateErnBuilder::new(),
            _marker: std::marker::PhantomData,
        }
    }
}

/// Implementation of `ErnBuilder` for `Part` states, allowing for building the final ERN (Entity Resource Name).
impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> ErnBuilder<Part, T> {
    /// Finalizes the building process and constructs the ERN (Entity Resource Name).
    pub fn build(self) -> Result<Ern<T>, ErnError> {
        self.builder.build()
    }
}

/// Implementation of `ErnBuilder` for handling `Parts` states.
impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> ErnBuilder<Parts, T> {
    /// Finalizes the building process and constructs the ERN (Entity Resource Name) when in the `Parts` state.
    pub fn build(self) -> Result<Ern<T>, ErnError> {
        self.builder.build()
    }
}

/// Generic implementation of `ErnBuilder` for all states that can transition to another state.
impl<Component: ErnComponent, T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> ErnBuilder<Component, T> {
    /// Adds a new part to the ERN (Entity Resource Name), transitioning to the next appropriate state.
    pub fn with<N>(
        self,
        part: impl Into<Cow<'static, str>>,
    ) -> Result<ErnBuilder<N::NextState, T>, ErnError>
    where
        N: ErnComponent<NextState = Component::NextState>,
    {
        Ok(ErnBuilder {
            builder: self.builder.add_part(N::prefix(), part.into())?,
            _marker: std::marker::PhantomData,
        })
    }
}

/// Represents a private, internal structure for building the ERN (Entity Resource Name).
struct PrivateErnBuilder<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> {
    domain: Option<Domain>,
    category: Option<Category>,
    account: Option<Account>,
    root: Option<Root<T>>,
    parts: Parts,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IdType + Clone + PartialEq + Eq + PartialOrd + Hash> PrivateErnBuilder<T> {
    /// Constructs a new private ERN (Entity Resource Name) builder.
    fn new() -> Self {
        Self {
            domain: None,
            category: None,
            account: None,
            root: None,
            parts: Parts::new(Vec::new()),
            _marker: Default::default(),
        }
    }

    fn add_part(mut self, prefix: &'static str, part: Cow<'static, str>) -> Result<Self, ErnError> {
        match prefix {
            p if p == Domain::prefix() => {
                self.domain = Some(Domain::new(part)?);
            }
            "" => {
                if self.domain.is_some() && self.category.is_none() {
                    self.category = Some(Category::new(part));
                } else if self.category.is_some() && self.account.is_none() {
                    self.account = Some(Account::new(part));
                } else if self.account.is_some() && self.root.is_none() {
                    self.root = Some(Root::new(part)?);
                } else {
                    // add the first part
                    self.parts = self.parts.add_part(Part::new(part)?);
                }
            }
            ":" => {
                self.parts = self.parts.add_part(Part::new(part)?);
            }
            _ => return Err(ErnError::InvalidPrefix(prefix.to_string())),
        }
        Ok(self)
    }

    /// Finalizes and builds the ERN (Entity Resource Name).
    fn build(self) -> Result<Ern<T>, ErnError> {
        let domain = self
            .domain
            .ok_or(ErnError::MissingPart("domain".to_string()))?;
        let category = self
            .category
            .ok_or(ErnError::MissingPart("category".to_string()))?;
        let account = self
            .account
            .ok_or(ErnError::MissingPart("account".to_string()))?;
        let root = self.root.ok_or(ErnError::MissingPart("root".to_string()))?;

        Ok(Ern::new(domain, category, account, root, self.parts))
    }
}

#[cfg(test)]
mod tests {
    use crate::errors::ErnError;
    use crate::prelude::*;
    use crate::tests::init_tracing;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        // Create an ERN (Entity Resource Name) using the ErnBuilder with specified components
        let ern: Result<Ern<UnixTime>, ErnError> = ErnBuilder::new()
            .with::<Domain>("acton-internal")?
            .with::<Category>("hr")?
            .with::<Account>("company123")?
            .with::<Root<UnixTime>>("root")?
            .with::<Part>("departmentA")?
            .with::<Part>("team1")?
            .build();

        // Verify the constructed ERN (Entity Resource Name) matches the expected value
        assert!(
            ern.is_ok(),
            "ern:acton-internal:hr:company123:root/departmentA/team1"
        );
        Ok(())
    }
    #[test]
    fn test_ern_builder() -> anyhow::Result<()> {
        let ern: Ern<UnixTime> = ErnBuilder::new()
            .with::<Domain>("custom")?
            .with::<Category>("service")?
            .with::<Account>("account123")?
            .with::<Root<UnixTime>>("resource")?
            .with::<Part>("subresource")?
            .build()?;

        assert!(
            ern.to_string().ends_with("/subresource"),
            "{} did not end with expected string",
            ern
        );

        Ok(())
    }

    #[test]
    fn test_ern_builder_with_default_parts() -> anyhow::Result<(), ErnError> {
        init_tracing();
        let ern: Ern<UnixTime> = Ern::default();
        tracing::debug!("{}", ern);
        let parser: ErnParser<UnixTime> = ErnParser::new(ern.to_string());
        let parsed: Ern<UnixTime> = parser.parse()?;
        assert_eq!(parsed.domain.as_str(), "acton");
        Ok(())
    }

    #[test]
    fn test_ern_builder_with_owned_strings() -> anyhow::Result<(), ErnError> {
        let ern: Ern<UnixTime> = ErnBuilder::new()
            .with::<Domain>(String::from("custom"))?
            .with::<Category>(String::from("service"))?
            .with::<Account>(String::from("account123"))?
            .with::<Root<UnixTime>>(String::from("resource"))?
            .build()?;

        assert!(ern
            .to_string()
            .starts_with("ern:custom:service:account123:resource"));
        Ok(())
    }
}

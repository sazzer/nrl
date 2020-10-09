use async_trait::async_trait;
use std::collections::HashMap;

/// The health of a single component.
#[derive(Debug, PartialEq)]
pub enum ComponentHealth {
    Healthy,
    Unhealthy(String),
}

/// The health of the entire system.
#[derive(Debug)]
pub struct SystemHealth {
    pub components: HashMap<String, ComponentHealth>,
}

impl ComponentHealth {
    /// Check if this component is healthy or not.
    ///
    /// # Returns
    /// True if this component is healthy. False if not.
    pub fn healthy(&self) -> bool {
        self == &Self::Healthy
    }

    /// Get the message from the component, if there is one.
    ///
    /// # Returns
    /// The message, if there is one.
    pub fn message(&self) -> Option<String> {
        match self {
            Self::Healthy => None,
            Self::Unhealthy(err) => Some(err.clone()),
        }
    }
}

impl SystemHealth {
    /// Construct a new instance of the system health.
    ///
    /// # Parameters
    /// - `components` - The health of the individual components
    pub const fn new(components: HashMap<String, ComponentHealth>) -> Self {
        Self { components }
    }

    /// Check if this system is healthy or not.
    ///
    /// # Returns
    /// True if this system is healthy. False if not.
    pub fn healthy(&self) -> bool {
        self.components.values().all(ComponentHealth::healthy)
    }
}

/// Trait that components can implement to indicate that they can report on their own health.
#[async_trait]
pub trait Healthchecker: Send + Sync {
    /// Check the health of the component.
    ///
    /// # Returns
    /// If the component is healthy then returns `Ok(())`.
    /// If the component is unhealthy then returns `Err(String)` where the string is the reason the component is unhealthy.
    async fn check_health(&self) -> Result<(), String>;
}

#[async_trait]
impl Healthchecker for ComponentHealth {
    async fn check_health(&self) -> Result<(), String> {
        match self {
            ComponentHealth::Healthy => Ok(()),
            ComponentHealth::Unhealthy(err) => Err(err.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn test_healthy_component() {
        let sut = ComponentHealth::Healthy;
        check!(sut.healthy());
        check!(sut.message() == None);
    }

    #[test]
    fn test_unhealthy_component() {
        let sut = ComponentHealth::Unhealthy("Oops".to_owned());
        check!(!sut.healthy());
        check!(sut.message() == Some("Oops".to_owned()));
    }

    #[test]
    fn test_empty_system() {
        let components = HashMap::new();
        let sut = SystemHealth::new(components);
        check!(sut.healthy());
    }

    #[test]
    fn test_healthy_system() {
        let mut components = HashMap::new();
        components.insert("healthy".to_owned(), ComponentHealth::Healthy);
        let sut = SystemHealth::new(components);
        check!(sut.healthy());
    }

    #[test]
    fn test_unhealthy_system() {
        let mut components = HashMap::new();
        components.insert(
            "unhealthy".to_owned(),
            ComponentHealth::Unhealthy("Oops".to_owned()),
        );
        let sut = SystemHealth::new(components);
        check!(!sut.healthy());
    }

    #[test]
    fn test_mixed_system() {
        let mut components = HashMap::new();
        components.insert("healthy".to_owned(), ComponentHealth::Healthy);
        components.insert(
            "unhealthy".to_owned(),
            ComponentHealth::Unhealthy("Oops".to_owned()),
        );
        let sut = SystemHealth::new(components);
        check!(!sut.healthy());
    }
}

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// The identity of some persisted API model.
#[derive(Debug)]
pub struct Identity<I> {
    /// The raw ID
    pub id: I,
    /// The version of the model
    pub version: Uuid,
    /// When the model was first created
    pub created: DateTime<Utc>,
    /// When the model was last updated
    pub updated: DateTime<Utc>,
}

/// Some persisted API model
#[derive(Debug)]
pub struct Model<I, D> {
    /// The identity of the model
    pub identity: Identity<I>,
    /// The actual data of the model
    pub data: D,
}

impl<I> Default for Identity<I>
where
    I: Default,
{
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: I::default(),
            version: Uuid::new_v4(),
            created: now,
            updated: now,
        }
    }
}

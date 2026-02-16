// In contracts/src/lib.rs
use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub origin: String,
    pub owner: Address,
    pub created_at: u64,
    pub authorized_actors: Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrackingEvent {
    pub id: String,
    pub product_id: String,
    pub location: String,
    pub actor: Address,
    pub timestamp: u64,
    pub event_type: EventType,
    pub metadata: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventType {
    Harvest,
    Processing,
    Packaging,
    Shipping,
    Receiving,
    QualityCheck,
}

#[contracttype]
pub enum DataKey {
    Product(String),           // Product by ID
    Events(String),            // Events by Product ID
    ProductCount,              // Total products
}
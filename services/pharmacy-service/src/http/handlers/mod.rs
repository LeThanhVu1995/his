pub mod health;
pub mod medications;
pub mod prescriptions;
pub mod dispenses;

// Re-export handler functions
pub use health::healthz;
pub use medications::{list_meds, create_med, update_med};
pub use prescriptions::{list_prescriptions, create_prescription, update_prescription};
pub use dispenses::{list_dispenses, create_dispense, finish_dispense};

pub mod patient_service;
pub mod encounter_service;
pub mod problem_service;
pub mod allergy_service;
pub mod vital_service;
pub mod order_service;

pub use patient_service::PatientService;
pub use encounter_service::EncounterService;
pub use problem_service::ProblemService;
pub use allergy_service::AllergyService;
pub use vital_service::VitalService;
pub use order_service::OrderService;

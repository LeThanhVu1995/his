pub mod patient_repo;
pub mod encounter_repo;
pub mod problem_repo;
pub mod allergy_repo;
pub mod vital_repo;
pub mod order_repo;

pub use patient_repo::PatientRepo;
pub use encounter_repo::EncounterRepo;
pub use problem_repo::ProblemRepo;
pub use allergy_repo::AllergyRepo;
pub use vital_repo::VitalRepo;
pub use order_repo::OrderRepo;

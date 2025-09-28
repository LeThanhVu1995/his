use app_web::prelude::PermissionDef;
use once_cell::sync::Lazy;

// EMR Service Permissions
pub static EMR_PATIENT_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.patient.read", "Read patient data", "patient", "read"));
pub static EMR_PATIENT_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.patient.write", "Write patient data", "patient", "write"));
pub static EMR_PATIENT_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.patient.delete", "Delete patient data", "patient", "delete"));

pub static EMR_ENCOUNTER_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.encounter.read", "Read encounter data", "encounter", "read"));
pub static EMR_ENCOUNTER_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.encounter.write", "Write encounter data", "encounter", "write"));
pub static EMR_ENCOUNTER_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.encounter.delete", "Delete encounter data", "encounter", "delete"));

pub static EMR_NOTE_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.note.read", "Read clinical notes", "note", "read"));
pub static EMR_NOTE_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.note.write", "Write clinical notes", "note", "write"));
pub static EMR_NOTE_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.note.delete", "Delete clinical notes", "note", "delete"));

pub static EMR_PROBLEM_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.problem.read", "Read problem list", "problem", "read"));
pub static EMR_PROBLEM_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.problem.write", "Write problem list", "problem", "write"));
pub static EMR_PROBLEM_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.problem.delete", "Delete problem list", "problem", "delete"));

pub static EMR_ALLERGY_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.allergy.read", "Read allergy data", "allergy", "read"));
pub static EMR_ALLERGY_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.allergy.write", "Write allergy data", "allergy", "write"));
pub static EMR_ALLERGY_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.allergy.delete", "Delete allergy data", "allergy", "delete"));

pub static EMR_MEDICATION_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.medication.read", "Read medication data", "medication", "read"));
pub static EMR_MEDICATION_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.medication.write", "Write medication data", "medication", "write"));
pub static EMR_MEDICATION_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.medication.delete", "Delete medication data", "medication", "delete"));

pub static EMR_VITAL_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.vital.read", "Read vital signs", "vital", "read"));
pub static EMR_VITAL_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.vital.write", "Write vital signs", "vital", "write"));
pub static EMR_VITAL_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.vital.delete", "Delete vital signs", "vital", "delete"));

pub static EMR_OBSERVATION_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.observation.read", "Read observations", "observation", "read"));
pub static EMR_OBSERVATION_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.observation.write", "Write observations", "observation", "write"));
pub static EMR_OBSERVATION_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.observation.delete", "Delete observations", "observation", "delete"));

pub static EMR_ORDER_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.order.read", "Read clinical orders", "order", "read"));
pub static EMR_ORDER_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.order.write", "Write clinical orders", "order", "write"));
pub static EMR_ORDER_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.order.delete", "Delete clinical orders", "order", "delete"));

pub static EMR_EPISODE_READ: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.episode.read", "Read episodes of care", "episode", "read"));
pub static EMR_EPISODE_WRITE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.episode.write", "Write episodes of care", "episode", "write"));
pub static EMR_EPISODE_DELETE: Lazy<PermissionDef> = Lazy::new(|| PermissionDef::new("his.emr.episode.delete", "Delete episodes of care", "episode", "delete"));

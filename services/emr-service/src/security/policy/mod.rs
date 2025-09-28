pub mod perm;

use app_web::prelude::*;

pub fn permission_catalog(_service_name: &str) -> Vec<PermissionDef> {
    vec![
        perm::EMR_PATIENT_READ.clone(),
        perm::EMR_PATIENT_WRITE.clone(),
        perm::EMR_PATIENT_DELETE.clone(),
        perm::EMR_ENCOUNTER_READ.clone(),
        perm::EMR_ENCOUNTER_WRITE.clone(),
        perm::EMR_ENCOUNTER_DELETE.clone(),
        perm::EMR_NOTE_READ.clone(),
        perm::EMR_NOTE_WRITE.clone(),
        perm::EMR_NOTE_DELETE.clone(),
        perm::EMR_PROBLEM_READ.clone(),
        perm::EMR_PROBLEM_WRITE.clone(),
        perm::EMR_PROBLEM_DELETE.clone(),
        perm::EMR_ALLERGY_READ.clone(),
        perm::EMR_ALLERGY_WRITE.clone(),
        perm::EMR_ALLERGY_DELETE.clone(),
        perm::EMR_MEDICATION_READ.clone(),
        perm::EMR_MEDICATION_WRITE.clone(),
        perm::EMR_MEDICATION_DELETE.clone(),
        perm::EMR_VITAL_READ.clone(),
        perm::EMR_VITAL_WRITE.clone(),
        perm::EMR_VITAL_DELETE.clone(),
        perm::EMR_OBSERVATION_READ.clone(),
        perm::EMR_OBSERVATION_WRITE.clone(),
        perm::EMR_OBSERVATION_DELETE.clone(),
        perm::EMR_ORDER_READ.clone(),
        perm::EMR_ORDER_WRITE.clone(),
        perm::EMR_ORDER_DELETE.clone(),
        perm::EMR_EPISODE_READ.clone(),
        perm::EMR_EPISODE_WRITE.clone(),
        perm::EMR_EPISODE_DELETE.clone(),
    ]
}

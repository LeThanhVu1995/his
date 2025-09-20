use uuid::Uuid;
use bigdecimal::BigDecimal;
use crate::domain::entities::{
    medication::Medication,
    prescription::Prescription,
    prescription_item::PrescriptionItem,
    dispense::Dispense,
};
use crate::domain::repositories::{
    medication_repo,
    prescription_repo,
    prescription_item_repo,
    dispense_repo,
};
use crate::error::AppError;
use sqlx::{Pool, Postgres};

pub async fn create_medication(
    db: &Pool<Postgres>,
    code: String,
    name: String,
    strength: Option<String>,
    form: Option<String>,
    route: Option<String>,
) -> Result<Uuid, AppError> {
    let id = Uuid::new_v4();
    let medication = Medication {
        id,
        code,
        name,
        strength,
        form,
        route,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    medication_repo::create(db, &medication).await?;
    Ok(id)
}

pub async fn create_prescription(
    db: &Pool<Postgres>,
    patient_id: Uuid,
    encounter_id: Option<Uuid>,
    note: Option<String>,
    items: Vec<(Uuid, Option<String>, Option<String>, Option<String>, BigDecimal, Option<String>)>,
    ordered_by: Option<String>,
) -> Result<Uuid, AppError> {
    let id = Uuid::new_v4();
    let presc_no = format!("PRC-{}", &id.to_string()[..8]);

    let prescription = Prescription {
        id,
        patient_id,
        encounter_id,
        presc_no,
        status: "NEW".into(),
        ordered_by,
        note,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    prescription_repo::create(db, &prescription).await?;

    let mut prescription_items = Vec::new();
    for (medication_id, dose, freq, duration, qty, instruction) in items {
        prescription_items.push(PrescriptionItem {
            id: Uuid::new_v4(),
            prescription_id: id,
            medication_id,
            dose,
            freq,
            duration,
            qty,
            instruction,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });
    }

    prescription_item_repo::insert_many(db, &prescription_items).await?;
    Ok(id)
}

pub async fn create_dispense(
    db: &Pool<Postgres>,
    prescription_id: Uuid,
    dispensed_by: Option<String>,
) -> Result<Uuid, AppError> {
    let id = Uuid::new_v4();
    let disp_no = format!("DSP-{}", &id.to_string()[..8]);

    let dispense = Dispense {
        id,
        prescription_id,
        disp_no,
        dispensed_by,
        status: "NEW".into(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    dispense_repo::create(db, &dispense).await?;
    Ok(id)
}

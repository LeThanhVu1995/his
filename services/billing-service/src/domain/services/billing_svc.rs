use uuid::Uuid;
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::domain::entities::charge::Charge;
use crate::domain::entities::invoice::Invoice;
use crate::domain::entities::invoice_item::InvoiceItem;
use crate::domain::entities::payment::Payment;
use crate::infra::db::repositories::{charge_repo, invoice_repo, invoice_item_repo, payment_repo};
use crate::infra::db::pool::PgPool;

pub async fn create_charge(
    db: &PgPool,
    patient_id: Uuid,
    encounter_id: Option<Uuid>,
    order_id: Option<Uuid>,
    code: String,
    name: String,
    qty: BigDecimal,
    unit_price: BigDecimal,
    currency: Option<String>,
) -> Result<Uuid, app_error::AppError> {
    let id = Uuid::new_v4();
    let amount = &qty * &unit_price;

    let charge = Charge {
        id,
        patient_id,
        encounter_id,
        order_id,
        code,
        name,
        qty: qty,
        unit_price: unit_price,
        amount,
        currency: currency.unwrap_or_else(|| "VND".into()),
        status: "NEW".into(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    charge_repo::create(db, &charge).await?;
    Ok(id)
}

pub async fn generate_invoice(
    db: &PgPool,
    patient_id: Uuid,
    encounter_id: Option<Uuid>,
    charge_ids: Vec<Uuid>,
    discount: Option<BigDecimal>,
    tax: Option<BigDecimal>,
    note: Option<String>,
) -> Result<Uuid, app_error::AppError> {
    let id = Uuid::new_v4();
    let invoice_no = format!("INV-{}", &id.to_string()[..8]);

    // Load charges and calculate totals
    let mut subtotal = BigDecimal::from(0);
    let mut items = Vec::new();

    for charge_id in charge_ids {
        if let Some(charge) = charge_repo::find_by_id(db, charge_id).await? {
            subtotal += &charge.amount;
            items.push(InvoiceItem {
                id: Uuid::new_v4(),
                invoice_id: id,
                charge_id: Some(charge.id),
                code: charge.code,
                name: charge.name,
                qty: charge.qty,
                unit_price: charge.unit_price,
                amount: charge.amount,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            });
        }
    }

    let discount_decimal = discount.unwrap_or(BigDecimal::from(0));
    let tax_decimal = tax.unwrap_or(BigDecimal::from(0));
    let total = (&subtotal - &discount_decimal + &tax_decimal).max(BigDecimal::from(0));

    let invoice = Invoice {
        id,
        invoice_no,
        patient_id,
        encounter_id,
        subtotal,
        discount: discount_decimal,
        tax: tax_decimal,
        total,
        status: "DRAFT".into(),
        note,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    invoice_repo::create(db, &invoice).await?;
    invoice_item_repo::insert_many(db, &items).await?;

    Ok(id)
}

pub async fn create_payment(
    db: &PgPool,
    invoice_id: Uuid,
    method: String,
    amount: BigDecimal,
    currency: String,
) -> Result<Uuid, app_error::AppError> {
    let id = Uuid::new_v4();
    let pay_no = format!("PAY-{}", &id.to_string()[..8]);

    let payment = Payment {
        id,
        invoice_id,
        pay_no,
        method,
        amount: amount,
        currency,
        received_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    payment_repo::create(db, &payment).await?;
    Ok(id)
}

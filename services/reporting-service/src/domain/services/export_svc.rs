use bytes::Bytes;
use umya_spreadsheet as xlsx;

pub struct ExportSvc<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> ExportSvc<'a> {
    pub async fn export_revenue_xlsx(
        &self,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
    ) -> anyhow::Result<Bytes> {
        let rows = crate::infra::db::views::revenue_range(self.db, start, end).await?;
        let mut book = xlsx::new_file();
        let sheet = book.get_sheet_mut(&0).unwrap();
        sheet.get_cell_mut("A1").set_value("Day");
        sheet.get_cell_mut("B1").set_value("Revenue");
        sheet.get_cell_mut("C1").set_value("Invoices");
        for (i, r) in rows.iter().enumerate() {
            let row = i + 2;
            sheet.get_cell_mut(format!("A{}", row)).set_value(r.day.format("%Y-%m-%d").to_string());
            sheet.get_cell_mut(format!("B{}", row)).set_value(r.revenue.to_string());
            sheet.get_cell_mut(format!("C{}", row)).set_value((r.invoices as f64).to_string());
        }
        // umya-spreadsheet v1.x writes to a Path. Use a temp file then read bytes.
        let dir = std::env::temp_dir();
        let path = dir.join(format!("reporting_export_{}-{}.xlsx", start, end));
        xlsx::writer::xlsx::write(&book, &path)?;
        let bytes = std::fs::read(path)?;
        Ok(Bytes::from(bytes))
    }
}

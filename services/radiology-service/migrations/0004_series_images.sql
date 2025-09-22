CREATE TABLE IF NOT EXISTS rad_series (
  id UUID PRIMARY KEY,
  study_id UUID NOT NULL REFERENCES rad_studies(id) ON DELETE CASCADE,
  series_no INT NOT NULL,
  description VARCHAR(255)
);
CREATE TABLE IF NOT EXISTS rad_images (
  id UUID PRIMARY KEY,
  series_id UUID NOT NULL REFERENCES rad_series(id) ON DELETE CASCADE,
  instance_no INT NOT NULL,
  sop_uid UUID NOT NULL,
  storage_uri TEXT                      -- link PACS/object storage nếu tích hợp
);
CREATE INDEX IF NOT EXISTS idx_rad_series_study ON rad_series(study_id);
CREATE INDEX IF NOT EXISTS idx_rad_images_series ON rad_images(series_id);

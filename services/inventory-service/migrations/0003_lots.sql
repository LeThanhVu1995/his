CREATE TABLE IF NOT EXISTS inv_lots (
  id UUID PRIMARY KEY,
  item_id UUID NOT NULL REFERENCES inv_items(id) ON DELETE CASCADE,
  lot_no VARCHAR(64) NOT NULL,
  exp_date DATE,
  UNIQUE(item_id, lot_no)
);

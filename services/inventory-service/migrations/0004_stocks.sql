CREATE TABLE IF NOT EXISTS inv_stocks (
  warehouse_id UUID NOT NULL REFERENCES warehouses(id) ON DELETE CASCADE,
  item_id UUID NOT NULL REFERENCES inv_items(id) ON DELETE CASCADE,
  lot_id UUID REFERENCES inv_lots(id) ON DELETE SET NULL,
  qty NUMERIC(18,3) NOT NULL DEFAULT 0,
  PRIMARY KEY (warehouse_id, item_id, lot_id)
);

CREATE INDEX IF NOT EXISTS idx_stocks_item ON inv_stocks(item_id);

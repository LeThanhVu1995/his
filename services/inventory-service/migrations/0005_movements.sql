CREATE TABLE IF NOT EXISTS inv_movements (
  id UUID PRIMARY KEY,
  mv_no VARCHAR(64) UNIQUE NOT NULL,
  mv_type VARCHAR(16) NOT NULL, -- RECEIVE/ISSUE/ADJUST/TRANSFER
  src_wh UUID REFERENCES warehouses(id),
  dst_wh UUID REFERENCES warehouses(id),
  note TEXT,
  created_by VARCHAR(64),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS inv_movement_lines (
  id UUID PRIMARY KEY,
  movement_id UUID NOT NULL REFERENCES inv_movements(id) ON DELETE CASCADE,
  item_id UUID NOT NULL REFERENCES inv_items(id),
  lot_id UUID REFERENCES inv_lots(id),
  qty NUMERIC(18,3) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mv_lines_mv ON inv_movement_lines(movement_id);

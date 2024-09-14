
CREATE TABLE IF NOT EXISTS car (
    id uuid,
    manufacturer string,
    model string,
    name string,
    owner uuid,
    year i32
)
CREATE INDEX IF NOT EXISTS unique_id_index ON (
    id
)
CREATE INDEX IF NOT EXISTS unique_model_index ON (
    name,
    model
)
CREATE INDEX IF NOT EXISTS unique_manufacturer_index ON (
    manufacturer
)
CREATE INDEX IF NOT EXISTS unique_year_index ON (
    year
)
CREATE INDEX IF NOT EXISTS unique_owner_index ON (
    owner
)
CREATE UNIQUE INDEX IF NOT EXISTS unique_id_index ON (
    id
)
CREATE UNIQUE INDEX IF NOT EXISTS unique_name_index ON (
    name
)
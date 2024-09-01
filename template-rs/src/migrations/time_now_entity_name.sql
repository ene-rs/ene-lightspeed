-- for each entity
CREATE TABLE IF NOT EXISTS entity_name (
    -- for each attribute
    attribute_name attribute_type,
    attribute2_name attribute2_type
    -- end for each attribute
)

-- for each filter
--      filter
CREATE INDEX IF NOT EXISTS unique_most_specific_attribute_filter_name_index ON (
    --          for each filter attribute
    filter_attribute_name,
    filter2_attribute_name
    --          end for each filter attribute
)
--      end filter
--      filter
CREATE INDEX IF NOT EXISTS most_specific_attribute_filter_name_index ON (
    --          for each filter attribute
    filter_attribute_name,
    filter2_attribute_name
    --          end for each filter attribute
)
--      end filter
-- end for each filter

-- for each composite key
--      composite key
CREATE UNIQUE INDEX IF NOT EXISTS unique_most_specific_attribute_name_index ON (
    --          for each composite key attribute
    composite_key_attribute_name,
    composite2_key_attribute_name
    --          end for each composite key attribute
)
--      end composite key
--      composite key
CREATE INDEX IF NOT EXISTS most_specific_attribute_name_index ON (
    --          for each composite key attribute
    composite_key_attribute_name,
    composite2_key_attribute_name
    --          end for each composite key attribute
)
--      end composite key
-- end for each composite key

CREATE INDEX raw_items_buildtype_league_idx ON raw_items USING gin((item->'baseType'));

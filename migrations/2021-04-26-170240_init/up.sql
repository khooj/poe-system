CREATE TABLE IF NOT EXISTS latest_stash_id (id TEXT PRIMARY KEY);
CREATE TABLE IF NOT EXISTS items (
    id TEXT PRIMARY KEY NOT NULL,
    base_type TEXT NOT NULL,
    -- extended object
    -- from parent json model
    account_id TEXT NOT NULL,
    account_name TEXT NOT NULL,
    stash_id TEXT NOT NULL,
    league TEXT,
    -- end parent json model
    name TEXT NOT NULL,
    item_lvl INTEGER,
    identified BOOLEAN NOT NULL,
    inventory_id TEXT,
    type_line TEXT NOT NULL,
    abyss_jewel BOOLEAN,
    corrupted BOOLEAN,
    duplicated BOOLEAN,
    elder BOOLEAN,
    frame_type INTEGER,
    h INTEGER NOT NULL,
    w INTEGER NOT NULL,
    x_coordinate INTEGER,
    y_coordinate INTEGER,
    is_relic BOOLEAN,
    note TEXT,
    shaper BOOLEAN,
    stack_size INTEGER,
    max_stack_size INTEGER,
    support BOOLEAN,
    talisman_tier INTEGER,
    verified BOOLEAN NOT NULL,
    icon TEXT NOT NULL,
    delve BOOLEAN,
    fractured BOOLEAN,
    synthesised BOOLEAN,
    split BOOLEAN,
    sec_descr_text TEXT,
    veiled BOOLEAN,
    descr_text TEXT,
    prophecy_text TEXT,
    replica BOOLEAN,
    socket INTEGER,
    colour TEXT
);
CREATE INDEX item_account_stash ON items(account_id, stash_id);
-- utility, implicit, explicit, crafted, enchant, fractured, cosmetic, veiled,
-- explicit_hybrid
CREATE TABLE IF NOT EXISTS mods (
    item_id TEXT NOT NULL,
    type INTEGER NOT NULL,
    mod TEXT NOT NULL,
    PRIMARY KEY(item_id, mod),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE INDEX mods_item_id ON mods(item_id);
CREATE TABLE IF NOT EXISTS subcategories (
    item_id TEXT NOT NULL,
    subcategory TEXT NOT NULL,
    PRIMARY KEY(item_id, subcategory),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
-- properties, requirements, additional_properties, next_level_requirements,
-- notable_properties, hybrid
CREATE TABLE IF NOT EXISTS properties (
    item_id TEXT NOT NULL,
    property_type INTEGER NOT NULL,
    name TEXT NOT NULL,
    value_type INTEGER NOT NULL,
    value INTEGER NOT NULL,
    type INTEGER,
    progress REAL,
    suffix TEXT,
    PRIMARY KEY(item_id, name),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE INDEX properties_item_id ON properties(item_id);
CREATE TABLE IF NOT EXISTS socketed_items (
    item_id TEXT NOT NULL,
    socketed_item_id TEXT NOT NULL,
    PRIMARY KEY(item_id, socketed_item_id),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE TABLE IF NOT EXISTS sockets (
    item_id TEXT NOT NULL,
    s_group INTEGER NOT NULL,
    attr TEXT,
    s_colour TEXT,
    PRIMARY KEY(item_id, s_group),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE TABLE IF NOT EXISTS ultimatum_mods (
    item_id TEXT NOT NULL,
    type TEXT NOT NULL,
    tier INTEGER NOT NULL,
    PRIMARY KEY(item_id, type),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE INDEX ultimatum_mods_item_id ON ultimatum_mods(item_id);
CREATE TABLE IF NOT EXISTS incubated_item (
    item_id TEXT NOT NULL,
    name TEXT NOT NULL,
    level INTEGER NOT NULL,
    progress INTEGER NOT NULL,
    total INTEGER NOT NULL,
    PRIMARY KEY(item_id, name),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE TABLE IF NOT EXISTS hybrids (
    id TEXT,
    item_id TEXT NOT NULL,
    is_vaal_gem BOOLEAN,
    base_type_name TEXT NOT NULL,
    sec_descr_text TEXT,
    PRIMARY KEY(id, item_id),
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE TABLE IF NOT EXISTS extended (
    item_id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    prefixes INTEGER,
    suffixes INTEGER,
    FOREIGN KEY(item_id) REFERENCES items(id)
);
CREATE TABLE IF NOT EXISTS influences (
    item_id TEXT PRIMARY KEY,
    warlord BOOLEAN,
    crusader BOOLEAN,
    redeemer BOOLEAN,
    hunter BOOLEAN,
    FOREIGN KEY(item_id) REFERENCES items(id)
);
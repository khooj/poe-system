CREATE KEYSPACE IF NOT EXISTS poesystem WITH REPLICATION = { 'class': 'SimpleStrategy', 'replication_factor': 1 };

CREATE TABLE IF NOT EXISTS poesystem.items(
  id text PRIMARY KEY,
  basetype text,
  affixes map<text, text>
);

CREATE TABLE IF NOT EXISTS poesystem.affixes(
  affix text,
  value text,
  item_id text,
  PRIMARY KEY((affix), item_id)
);

//CREATE INDEX IF NOT EXISTS affixes_idx ON poesystem.items( KEYS(affixes) );
//WITH OPTIONS = { 'case_sensitive': false, 'normalize': true, 'ascii': true };

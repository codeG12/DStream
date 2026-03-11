``` text
-- 1. Connectors (source and target configurations)
CREATE TABLE connectors (
connector_id SERIAL PRIMARY KEY,
connector_name VARCHAR(255) UNIQUE NOT NULL,
connector_type VARCHAR(20) NOT NULL, -- 'tap', 'target'
config JSONB NOT NULL,
is_active BOOLEAN DEFAULT true,
created_at TIMESTAMP DEFAULT NOW(),
updated_at TIMESTAMP DEFAULT NOW()
);

-- 2. Streams (ETL pipelines)
CREATE TABLE streams (
stream_id SERIAL PRIMARY KEY,
stream_name VARCHAR(255) NOT NULL,
source_connector_id INT NOT NULL REFERENCES connectors(connector_id),
target_connector_id INT NOT NULL REFERENCES connectors(connector_id),
is_active BOOLEAN DEFAULT true,
last_sync_status VARCHAR(50),
last_sync_at TIMESTAMP,
created_at TIMESTAMP DEFAULT NOW(),
updated_at TIMESTAMP DEFAULT NOW(),
UNIQUE(stream_name, source_connector_id, target_connector_id)
);

-- 3. Catalog (discovered tables and schemas)
CREATE TABLE catalog (
catalog_id SERIAL PRIMARY KEY,
connector_id INT NOT NULL REFERENCES connectors(connector_id),
table_name VARCHAR(255) NOT NULL,
schema_name VARCHAR(255),
table_schema JSONB NOT NULL,
key_properties JSONB,
replication_method VARCHAR(50),
replication_key VARCHAR(255),
is_selected BOOLEAN DEFAULT false,
created_at TIMESTAMP DEFAULT NOW(),
updated_at TIMESTAMP DEFAULT NOW(),
UNIQUE(connector_id, table_name, schema_name)
);

-- 4. State (bookmark for incremental syncs)
CREATE TABLE state (
stream_id INT NOT NULL REFERENCES streams(stream_id),
table_name VARCHAR(255) NOT NULL,
bookmark_column VARCHAR(255),
bookmark_value VARCHAR(500),
bookmark_type VARCHAR(50),
records_synced BIGINT DEFAULT 0,
last_sync_at TIMESTAMP,
updated_at TIMESTAMP DEFAULT NOW(),
PRIMARY KEY (stream_id, table_name)
);
```

``` text
Core Concept:

✅ No connector objects - just validate configs
✅ Generic connection handling - create pools on-demand
✅ Batch sink - accumulate records, write in batches
✅ State update AFTER successful write - correct approach!
```
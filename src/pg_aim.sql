-- complaints table for extension versioning
CREATE TABLE IF NOT EXISTS pg_aim.pg_aim_version (
    version text NOT NULL
);

INSERT INTO pg_aim.pg_aim_version 
VALUES ('0.1.0') 
ON CONFLICT DO NOTHING;


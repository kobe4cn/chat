-- Add migration script here
-- workspace for users
CREATE TABLE IF NOT EXISTS workspaces (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE,
    owner_id BIGINT NOT NULL references users(id),
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

--alter users table for workspace
ALTER TABLE users ADD COLUMN ws_id BIGINT references workspaces(id);

-- add super user 0 workspace 0
BEGIN;
INSERT INTO users (id,fullname, email, password_hash)
VALUES (0,'super user', 'super@super.org','');
INSERT INTO workspaces (id,name, owner_id) VALUES (0,'default', 0);
update users set ws_id = 0 where id = 0;
alter table users alter column ws_id set not null;
COMMIT;

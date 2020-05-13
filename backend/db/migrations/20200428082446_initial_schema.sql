-- migrate:up
CREATE TABLE users
(
    id SERIAL PRIMARY KEY
);

CREATE TABLE activities
(
    id        SERIAL PRIMARY KEY,
    name      VARCHAR(255)              NOT NULL,
    user_id   INT REFERENCES users (id) NOT NULL,
    parent_id INT REFERENCES activities (id)

);

CREATE UNIQUE INDEX IXU_activities_name ON activities (user_id, name);

CREATE TABLE activity_events
(
    id          SERIAL PRIMARY KEY,
    notes       TEXT,
    activity_id INT REFERENCES activities (id) NOT NULL,
    parent_id   INT REFERENCES activity_events (id)
);

-- migrate:down
DROP TABLE users CASCADE;
DROP TABLE activities CASCADE;
DROP TABLE activity_events;

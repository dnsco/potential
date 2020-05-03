-- migrate:up
CREATE TABLE activities
(
    id        SERIAL PRIMARY KEY,
    name      VARCHAR(255) NOT NULL,
    parent_id INT REFERENCES activities (id)
);

CREATE TABLE activity_events
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255)                   NOT NULL,
    notes       TEXT,
    activity_id INT REFERENCES activities (id) NOT NULL,
    parent_id   INT REFERENCES activity_events (id)
);

-- migrate:down
DROP TABLE activities CASCADE;

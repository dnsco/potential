-- migrate:up
CREATE TABLE activities
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE activity_events
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    notes       TEXT,
    activity_id INT          NOT NULL references activities (id)
);

-- migrate:down
DROP TABLE activities CASCADE;

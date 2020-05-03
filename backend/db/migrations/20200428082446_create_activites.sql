-- migrate:up
CREATE TABLE activities
(
    id        SERIAL PRIMARY KEY,
    name      VARCHAR(255) NOT NULL,
    parent_id INT          NOT NULL references activities (id)
);

CREATE TABLE activity_events
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    notes       TEXT,
    activity_id INT          NOT NULL references activities (id),
    parent_id   INT          NOT NULL references activity_events (id)
);


-- migrate:down
DROP TABLE activities CASCADE;

CREATE TABLE personal_relationships (
    subject_id TEXT NOT NULL REFERENCES workers(id),
    other_id TEXT NOT NULL REFERENCES workers(id),
    state TEXT NOT NULL CHECK(json_valid(state)),
    PRIMARY KEY(subject_id, other_id),
    CHECK(subject_id <> other_id)
) STRICT;

CREATE TABLE relationship_memories (
    id TEXT PRIMARY KEY NOT NULL,
    subject_id TEXT NOT NULL,
    other_id TEXT NOT NULL,
    occurred_on TEXT NOT NULL,
    memory TEXT NOT NULL CHECK(json_valid(memory)),
    FOREIGN KEY(subject_id, other_id)
        REFERENCES personal_relationships(subject_id, other_id)
) STRICT;

CREATE INDEX relationship_memories_subject_date
    ON relationship_memories(subject_id, other_id, occurred_on DESC, id);

CREATE TABLE management_relationships (
    company_id TEXT NOT NULL REFERENCES promotions(id),
    worker_id TEXT NOT NULL REFERENCES workers(id),
    state TEXT NOT NULL CHECK(json_valid(state)),
    PRIMARY KEY(company_id, worker_id)
) STRICT;

CREATE TABLE player_interactions (
    request_id TEXT PRIMARY KEY NOT NULL,
    company_id TEXT NOT NULL REFERENCES promotions(id),
    worker_id TEXT NOT NULL REFERENCES workers(id),
    kind TEXT NOT NULL,
    occurred_on TEXT NOT NULL,
    context_worker_id TEXT REFERENCES workers(id),
    attention_cost INTEGER NOT NULL CHECK(attention_cost BETWEEN 1 AND 4),
    management_revision INTEGER NOT NULL CHECK(management_revision >= 1),
    outcome TEXT NOT NULL CHECK(json_valid(outcome))
) STRICT;

CREATE INDEX player_interactions_worker_date
    ON player_interactions(company_id, worker_id, occurred_on DESC, request_id);
CREATE INDEX player_interactions_daily_attention
    ON player_interactions(company_id, occurred_on);

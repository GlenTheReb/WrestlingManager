CREATE TABLE worker_discovery_index (
    worker_id TEXT PRIMARY KEY NOT NULL REFERENCES workers(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    nationality TEXT NOT NULL,
    school TEXT NOT NULL,
    languages TEXT NOT NULL CHECK(json_valid(languages)),
    biography TEXT NOT NULL,
    archetype TEXT NOT NULL,
    primary_discipline TEXT NOT NULL,
    overall INTEGER NOT NULL CHECK(overall BETWEEN 0 AND 100),
    movement INTEGER NOT NULL CHECK(movement BETWEEN 0 AND 100),
    physicality INTEGER NOT NULL CHECK(physicality BETWEEN 0 AND 100),
    ringcraft INTEGER NOT NULL CHECK(ringcraft BETWEEN 0 AND 100),
    psychology INTEGER NOT NULL CHECK(psychology BETWEEN 0 AND 100),
    fundamentals INTEGER NOT NULL CHECK(fundamentals BETWEEN 0 AND 100),
    entertainment INTEGER NOT NULL CHECK(entertainment BETWEEN 0 AND 100),
    fatigue INTEGER NOT NULL,
    morale INTEGER NOT NULL,
    momentum INTEGER NOT NULL,
    injury_days INTEGER NOT NULL
) STRICT;

CREATE INDEX worker_discovery_name ON worker_discovery_index(name COLLATE NOCASE, worker_id);
CREATE INDEX worker_discovery_age ON worker_discovery_index(age, worker_id);
CREATE INDEX worker_discovery_overall ON worker_discovery_index(overall DESC, worker_id);
CREATE INDEX worker_discovery_movement ON worker_discovery_index(movement DESC, worker_id);
CREATE INDEX worker_discovery_physicality ON worker_discovery_index(physicality DESC, worker_id);
CREATE INDEX worker_discovery_ringcraft ON worker_discovery_index(ringcraft DESC, worker_id);
CREATE INDEX worker_discovery_psychology ON worker_discovery_index(psychology DESC, worker_id);
CREATE INDEX worker_discovery_fundamentals ON worker_discovery_index(fundamentals DESC, worker_id);
CREATE INDEX worker_discovery_entertainment ON worker_discovery_index(entertainment DESC, worker_id);
CREATE INDEX worker_discovery_condition ON worker_discovery_index(injury_days, fatigue, worker_id);
CREATE INDEX worker_discovery_nationality ON worker_discovery_index(nationality COLLATE NOCASE, worker_id);
CREATE INDEX worker_discovery_school ON worker_discovery_index(school COLLATE NOCASE, worker_id);
CREATE INDEX worker_discovery_archetype ON worker_discovery_index(archetype COLLATE NOCASE, worker_id);
CREATE INDEX worker_discovery_discipline ON worker_discovery_index(primary_discipline COLLATE NOCASE, worker_id);

CREATE TABLE worker_saved_views (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL COLLATE NOCASE UNIQUE,
    request TEXT NOT NULL CHECK(json_valid(request))
) STRICT;

CREATE TABLE worker_shortlists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL COLLATE NOCASE UNIQUE
) STRICT;

CREATE TABLE worker_shortlist_members (
    shortlist_id INTEGER NOT NULL REFERENCES worker_shortlists(id) ON DELETE CASCADE,
    worker_id TEXT NOT NULL REFERENCES workers(id) ON DELETE CASCADE,
    PRIMARY KEY(shortlist_id, worker_id)
) STRICT;

CREATE TABLE worker_blacklist (
    worker_id TEXT PRIMARY KEY NOT NULL REFERENCES workers(id) ON DELETE CASCADE
) STRICT;

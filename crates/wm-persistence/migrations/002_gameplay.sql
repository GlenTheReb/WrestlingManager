CREATE TABLE workers (
    id TEXT PRIMARY KEY, name TEXT NOT NULL, age INTEGER NOT NULL CHECK(age BETWEEN 16 AND 120),
    style TEXT NOT NULL, nationality TEXT NOT NULL, language TEXT NOT NULL, school TEXT NOT NULL,
    background TEXT NOT NULL, personality TEXT NOT NULL, ambition TEXT NOT NULL, weight_kg INTEGER NOT NULL,
    appearance_fee INTEGER NOT NULL, attributes TEXT NOT NULL CHECK(json_valid(attributes)),
    wrestling_style TEXT NOT NULL CHECK(json_valid(wrestling_style)),
    identity TEXT NOT NULL CHECK(json_valid(identity)),
    condition TEXT NOT NULL CHECK(json_valid(condition)), moves TEXT NOT NULL CHECK(json_valid(moves))
) STRICT;
CREATE INDEX workers_name ON workers(name COLLATE NOCASE);
CREATE TABLE agents (id INTEGER PRIMARY KEY, data TEXT NOT NULL CHECK(json_valid(data))) STRICT;
CREATE TABLE shows (id INTEGER PRIMARY KEY, name TEXT NOT NULL, show_date TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('draft','live','complete')), revision INTEGER NOT NULL,
    report TEXT CHECK(report IS NULL OR json_valid(report))) STRICT;
CREATE TABLE segments (id INTEGER PRIMARY KEY, show_id INTEGER NOT NULL REFERENCES shows(id),
    position INTEGER NOT NULL, title TEXT NOT NULL, plan TEXT NOT NULL CHECK(json_valid(plan))) STRICT;
CREATE INDEX segments_show ON segments(show_id,position);
CREATE TABLE show_runtime (show_id INTEGER PRIMARY KEY REFERENCES shows(id), snapshot TEXT NOT NULL CHECK(json_valid(snapshot))) STRICT;
CREATE TABLE simulation_events (show_id INTEGER NOT NULL REFERENCES shows(id), sequence INTEGER NOT NULL,
    data TEXT NOT NULL CHECK(json_valid(data)), PRIMARY KEY(show_id,sequence)) STRICT;
CREATE TABLE worker_history (id INTEGER PRIMARY KEY, worker_id TEXT NOT NULL REFERENCES workers(id),
    show_id INTEGER NOT NULL REFERENCES shows(id), segment_id INTEGER NOT NULL REFERENCES segments(id),
    report TEXT NOT NULL CHECK(json_valid(report)), UNIQUE(worker_id,segment_id)) STRICT;
CREATE INDEX worker_history_worker ON worker_history(worker_id,id DESC);
CREATE TABLE relationships (worker_a TEXT NOT NULL REFERENCES workers(id), worker_b TEXT NOT NULL REFERENCES workers(id),
    chemistry INTEGER NOT NULL CHECK(chemistry BETWEEN 0 AND 100),PRIMARY KEY(worker_a,worker_b)) STRICT;
CREATE TABLE audience (id INTEGER PRIMARY KEY, trust INTEGER NOT NULL CHECK(trust BETWEEN 0 AND 100)) STRICT;
CREATE TABLE media_posts (id INTEGER PRIMARY KEY,show_id INTEGER NOT NULL REFERENCES shows(id),author TEXT NOT NULL,text TEXT NOT NULL,posted_on TEXT NOT NULL) STRICT;
CREATE TABLE ledger (id INTEGER PRIMARY KEY,show_id INTEGER NOT NULL UNIQUE REFERENCES shows(id),amount_pence INTEGER NOT NULL,reason TEXT NOT NULL) STRICT;

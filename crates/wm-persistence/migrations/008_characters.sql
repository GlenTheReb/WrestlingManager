CREATE TABLE persons (
    worker_id TEXT PRIMARY KEY NOT NULL REFERENCES workers(id) ON DELETE CASCADE,
    legal_name TEXT
) STRICT;

CREATE TABLE characters (
    id TEXT PRIMARY KEY NOT NULL,
    worker_id TEXT NOT NULL REFERENCES persons(worker_id) ON DELETE CASCADE,
    ring_name TEXT NOT NULL,
    alignment_intent TEXT NOT NULL CHECK(alignment_intent IN ('face','heel','tweener','unaligned')),
    status TEXT NOT NULL CHECK(status IN ('planned','active','retired')),
    revision INTEGER NOT NULL DEFAULT 1 CHECK(revision >= 1),
    masked INTEGER NOT NULL CHECK(masked IN (0,1)),
    concealed INTEGER NOT NULL CHECK(concealed IN (0,1)),
    gimmick TEXT NOT NULL CHECK(json_valid(gimmick)),
    created_on TEXT NOT NULL,
    CHECK(concealed = 0 OR masked = 1)
) STRICT;

CREATE TABLE character_tenures (
    id INTEGER PRIMARY KEY,
    character_id TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    company_id TEXT NOT NULL,
    brand TEXT,
    started_on TEXT NOT NULL,
    ended_on TEXT,
    knowledge TEXT NOT NULL CHECK(knowledge IN ('private','rumoured','public')),
    CHECK(ended_on IS NULL OR ended_on >= started_on)
) STRICT;
CREATE UNIQUE INDEX active_tenure_per_character_context
    ON character_tenures(character_id, company_id, ifnull(brand,'')) WHERE ended_on IS NULL;
CREATE INDEX character_tenure_context
    ON character_tenures(company_id, ifnull(brand,''), started_on, ended_on, character_id);
CREATE INDEX characters_worker ON characters(worker_id, status, created_on DESC);

CREATE TRIGGER reject_duplicate_active_ring_name
BEFORE INSERT ON character_tenures
WHEN NEW.ended_on IS NULL
BEGIN
    SELECT RAISE(ABORT, 'duplicate active ring name')
    WHERE EXISTS (
        SELECT 1
        FROM characters candidate
        JOIN character_tenures existing_tenure ON existing_tenure.character_id <> NEW.character_id
        JOIN characters existing ON existing.id = existing_tenure.character_id
        WHERE candidate.id = NEW.character_id
          AND candidate.status = 'active'
          AND existing.status = 'active'
          AND lower(existing.ring_name) = lower(candidate.ring_name)
          AND existing_tenure.company_id = NEW.company_id
          AND ifnull(existing_tenure.brand, '') = ifnull(NEW.brand, '')
          AND existing_tenure.ended_on IS NULL
    );
END;

CREATE TRIGGER reject_conflicting_active_identity
BEFORE INSERT ON character_tenures
WHEN NEW.ended_on IS NULL
BEGIN
    SELECT RAISE(ABORT, 'person already has an active identity in this company context')
    WHERE EXISTS (
        SELECT 1
        FROM characters incoming
        JOIN characters existing ON existing.worker_id = incoming.worker_id
        JOIN character_tenures existing_tenure ON existing_tenure.character_id = existing.id
        WHERE incoming.id = NEW.character_id
          AND incoming.status = 'active'
          AND existing.status = 'active'
          AND existing.id <> incoming.id
          AND existing_tenure.company_id = NEW.company_id
          AND ifnull(existing_tenure.brand, '') = ifnull(NEW.brand, '')
          AND existing_tenure.ended_on IS NULL
    );
END;

CREATE TRIGGER reject_conflicting_active_identity_update
BEFORE UPDATE OF character_id, company_id, brand, ended_on ON character_tenures
WHEN NEW.ended_on IS NULL
BEGIN
    SELECT RAISE(ABORT, 'person already has an active identity in this company context')
    WHERE EXISTS (
        SELECT 1
        FROM characters incoming
        JOIN characters existing ON existing.worker_id = incoming.worker_id
        JOIN character_tenures existing_tenure ON existing_tenure.character_id = existing.id
        WHERE incoming.id = NEW.character_id
          AND incoming.status = 'active'
          AND existing.status = 'active'
          AND existing.id <> incoming.id
          AND existing_tenure.id <> NEW.id
          AND existing_tenure.company_id = NEW.company_id
          AND ifnull(existing_tenure.brand, '') = ifnull(NEW.brand, '')
          AND existing_tenure.ended_on IS NULL
    );
END;

CREATE TRIGGER reject_duplicate_ring_name_update
BEFORE UPDATE OF ring_name, status ON characters
WHEN NEW.status = 'active'
BEGIN
    SELECT RAISE(ABORT, 'duplicate active ring name')
    WHERE EXISTS (
        SELECT 1
        FROM character_tenures own_tenure
        JOIN character_tenures other_tenure
          ON other_tenure.company_id = own_tenure.company_id
         AND ifnull(other_tenure.brand, '') = ifnull(own_tenure.brand, '')
         AND other_tenure.character_id <> NEW.id
         AND other_tenure.ended_on IS NULL
        JOIN characters other ON other.id = other_tenure.character_id
        WHERE own_tenure.character_id = NEW.id
          AND own_tenure.ended_on IS NULL
          AND other.status = 'active'
          AND lower(other.ring_name) = lower(NEW.ring_name)
    );
END;

CREATE TRIGGER reject_active_character_context_collision
BEFORE UPDATE OF status ON characters
WHEN NEW.status = 'active' AND OLD.status <> 'active'
BEGIN
    SELECT RAISE(ABORT, 'person already has an active identity in this company context')
    WHERE EXISTS (
        SELECT 1
        FROM character_tenures own_tenure
        JOIN character_tenures other_tenure
          ON other_tenure.company_id = own_tenure.company_id
         AND ifnull(other_tenure.brand, '') = ifnull(own_tenure.brand, '')
         AND other_tenure.character_id <> NEW.id
         AND other_tenure.ended_on IS NULL
        JOIN characters other ON other.id = other_tenure.character_id
        WHERE own_tenure.character_id = NEW.id
          AND own_tenure.ended_on IS NULL
          AND other.worker_id = NEW.worker_id
          AND other.status = 'active'
    );
END;

CREATE TRIGGER reject_duplicate_ring_name_tenure_update
BEFORE UPDATE OF character_id, company_id, brand, ended_on ON character_tenures
WHEN NEW.ended_on IS NULL
BEGIN
    SELECT RAISE(ABORT, 'duplicate active ring name')
    WHERE EXISTS (
        SELECT 1
        FROM characters incoming
        JOIN character_tenures existing_tenure
          ON existing_tenure.company_id = NEW.company_id
         AND ifnull(existing_tenure.brand, '') = ifnull(NEW.brand, '')
         AND existing_tenure.id <> NEW.id
         AND existing_tenure.ended_on IS NULL
        JOIN characters existing ON existing.id = existing_tenure.character_id
        WHERE incoming.id = NEW.character_id
          AND incoming.status = 'active'
          AND existing.status = 'active'
          AND lower(existing.ring_name) = lower(incoming.ring_name)
    );
END;

CREATE TABLE character_aliases (
    id INTEGER PRIMARY KEY,
    character_id TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    alias TEXT NOT NULL,
    started_on TEXT NOT NULL,
    ended_on TEXT,
    knowledge TEXT NOT NULL CHECK(knowledge IN ('private','rumoured','public')),
    UNIQUE(character_id, alias, started_on)
) STRICT;
CREATE INDEX character_alias_search ON character_aliases(alias COLLATE NOCASE, knowledge);

CREATE TABLE audience_response_evidence (
    id INTEGER PRIMARY KEY,
    character_id TEXT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    occurred_on TEXT NOT NULL,
    perceived_role TEXT NOT NULL CHECK(perceived_role IN ('face','heel','mixed','unclear')),
    response TEXT NOT NULL CHECK(response IN ('cheered','booed','mixed','indifferent')),
    intensity TEXT NOT NULL CHECK(intensity IN ('mild','moderate','strong')),
    acceptance TEXT NOT NULL CHECK(acceptance IN ('embraced','accepted','uncertain','rejected')),
    intent_match TEXT NOT NULL CHECK(intent_match IN ('matched','mismatched','ambiguous')),
    context TEXT NOT NULL
) STRICT;
CREATE INDEX audience_response_character ON audience_response_evidence(character_id, occurred_on DESC);

CREATE TABLE character_changes (
    id INTEGER PRIMARY KEY,
    worker_id TEXT NOT NULL REFERENCES persons(worker_id) ON DELETE CASCADE,
    request_id TEXT NOT NULL UNIQUE,
    proposed_ring_name TEXT NOT NULL,
    proposed_alignment TEXT NOT NULL CHECK(proposed_alignment IN ('face','heel','tweener','unaligned')),
    proposed_gimmick TEXT NOT NULL CHECK(json_valid(proposed_gimmick)),
    masked INTEGER NOT NULL CHECK(masked IN (0,1)),
    concealed INTEGER NOT NULL CHECK(concealed IN (0,1)),
    status TEXT NOT NULL CHECK(status IN ('proposed','negotiating','accepted','refused','ready','launched','cancelled')),
    revision INTEGER NOT NULL,
    proposed_on TEXT NOT NULL,
    intended_launch_on TEXT,
    launched_on TEXT,
    worker_response TEXT NOT NULL,
    readiness TEXT NOT NULL,
    risk TEXT NOT NULL,
    advice TEXT NOT NULL CHECK(json_valid(advice))
) STRICT;
CREATE INDEX character_changes_worker ON character_changes(worker_id, id DESC);

CREATE TABLE character_action_receipts (
    request_id TEXT PRIMARY KEY NOT NULL,
    worker_id TEXT NOT NULL REFERENCES persons(worker_id) ON DELETE CASCADE,
    action TEXT NOT NULL,
    occurred_on TEXT NOT NULL
) STRICT;

ALTER TABLE worker_discovery_index ADD COLUMN aliases TEXT NOT NULL DEFAULT '[]' CHECK(json_valid(aliases));

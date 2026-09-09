# ADR 0002: one bundled SQLite database per save

Status: accepted for schema v1.

Use rusqlite with bundled SQLite, strict relational tables, explicit schema versions and
transactional changes. Current state and important append-only events live together.
Portable saves must not depend on an external database service.

JSON is easy to inspect but becomes awkward for historical queries and atomic multi-entity
updates. An external database adds operational work without helping local single-player
play. SQLite requires careful migrations, backups and concurrency limits. Temporary-file
publication protects initial creation; migration backup rotation is still future work.

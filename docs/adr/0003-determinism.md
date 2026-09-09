# ADR 0003: versioned deterministic simulation

Status: seed/version storage accepted; simulation implementation deferred.

Persist an unsigned 64-bit seed as decimal text and an engine version. Before adding random
generation, pin a ChaCha-family generator and derive stable independent streams for domain
systems. Record inputs and order explicitly; use controlled arithmetic for simulation.

A global RNG is simpler but couples unrelated systems: one extra draw changes later
results everywhere. Independent versioned streams reduce that coupling at the cost of an
explicit allocation contract. A seed alone does not guarantee determinism across engine
changes. Incompatible versions must be rejected or explicitly migrated, never silently run.

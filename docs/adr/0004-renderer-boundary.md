# ADR 0004: event-driven presentation

Status: accepted design; renderer not yet implemented.

PixiJS 8 will render ordered simulation events using data-driven 2D/2.5D animation states.
Simulation owns action resolution and results. Presentation can interpolate frames,
change speed or skip without changing the world.

Resolving moves inside animation callbacks would be quick to prototype but tie results
to frame rate and viewing choices. Separating them requires stable event schemas and
testing, while enabling instant simulation, replay and later presentation improvements.
No physics wrestling engine or final animation work belongs in the initial milestone.

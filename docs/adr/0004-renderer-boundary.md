# ADR 0004: event-driven presentation

Status: accepted design; amended by PD-132 on 12 September 2026; renderer not yet implemented.

A renderer-neutral event stream feeds a complete text/2.5D viewer; HTML/CSS or a lightweight 2D
library may implement its approved design. Conditional WM-043 may later feed the same events into an
original PlayCanvas Engine v2 low-poly 3D broadcast viewer through `@playcanvas/react`. WebGL2 is the
required baseline and WebGPU is optional. Simulation owns action resolution and results.
Presentation can interpolate frames, change speed or skip without changing the world.
When PD-112 accepts a live instruction, Rust—not the active viewer—amends the unsimulated future. A
2.5D or 3D renderer may then rebuild buffered future presentation without rewriting displayed events.

Resolving moves inside animation callbacks would be quick to prototype but tie results
to frame rate and viewing choices. Separating them requires stable event schemas and
testing, while enabling instant simulation, replay and later presentation improvements.
No physics wrestling engine or character-animation work belongs in the initial milestones. WM-043
starts only after its separate feasibility gate and cannot replace the text/2.5D fallback.

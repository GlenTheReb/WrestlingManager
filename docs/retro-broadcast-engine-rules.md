# PD-132 — Optional retro 3D broadcast engine

Status: Accepted direction 12 September 2026. This is a conditional post-core presentation capability,
not current implementation and not a dependency of WM-025 or the first playable management milestones.

## Player promise

Players may eventually watch WM's canonical match and show timeline as a lightweight, stylised 3D
broadcast without changing results, requiring modern high-end hardware or losing the complete text-led
experience.

## Accepted technical direction

- Build an original WM-owned renderer with PlayCanvas Engine v2 through `@playcanvas/react` inside the
  existing React, strict TypeScript, Vite and Tauri application. WebGL2 is the required baseline;
  WebGPU is an optional enhancement only where the host supports it reliably.
- Virtual Pro Grappler remains reference/research material only. WM does not use its GPL code,
  depend on its unfinished engine or import assets whose rights and provenance are unverified.
- Reproduce the broad low-poly design principles of AKI-era wrestling games, never their proprietary
  code, ROM data, animation, textures, branding or wrestler likenesses.
- Rust simulation and the renderer-neutral WM-013 event stream remain authoritative. The 3D layer only
  selects presentation assets, cameras, lighting, crowd/audio cues and playback timing.
- The WM-014 text/2.5D viewer remains a complete accessible fallback. Missing hardware, models,
  animations or optional packs cannot block simulation, saving, reports or CPU-show inspection.
- Per [PD-112](live-match-direction-rules.md), every supported segment may use a show default or
  segment override: Quick Sim, 2.5D Extended Highlights, 2.5D Realtime, 3D Extended Highlights or
  3D Realtime. 3D availability never removes the guaranteed alternatives.
- Accepted live direction causes Rust to rebuild only the unsimulated match future. PlayCanvas may
  regenerate buffered future animation selections from the resulting WM-013 events, but animation
  timing and callbacks never create communication opportunities or apply instructions.

## Asset and automation strategy

- Use one documented canonical WM skeleton, modular low-poly bodies and parameterised height, build,
  skin, face, hair, mask and attire. Generate runtime GLB assets through a reproducible Blender
  processing pipeline that validates hierarchy, bone names, materials, clips and provenance.
- Seed the library with properly licensed commercial/original model and paired-wrestling animation
  assets only after commercial use, conversion, redistribution, attribution and provenance are
  verified. Never import unlicensed likenesses, logos, arenas or extracted game assets.
- Represent each move as a versioned WM-owned paired interaction recipe: synchronised attacker and
  receiver clips or poses, alignment anchors, phases, contact points, valid body/ring contexts,
  timing, selling, recovery, interruption/reversal markers and suggested camera/audio/effect cues.
- A database may contain many named moves mapped to a smaller library of main visual families and
  variants. Important signatures can receive dedicated recipes. A missing recipe uses an honest
  text/2.5D fallback instead of showing a contradictory move.
- AI may help write tooling or draft original recipe data, but is not assumed to generate reliable
  two-person contact animation or establish asset rights. Every generated/imported asset requires
  deterministic validation and visual approval.

## Scope and gates

- This is a broadcast viewer, not a player-controlled wrestling game. It does not own match rules,
  damage, winners, injuries, booking or canonical timing.
- Begin only with a bounded feasibility spike: one original arena, two generic modular wrestlers,
  representative body variation, five paired moves, automated cameras and playback of one stored WM
  timeline inside the real Tauri/WebView2 application. Continue only if contact, conversion, fallback
  and performance criteria pass, including sustained 60 FPS (16.7 ms frame budget) on a documented
  ordinary-PC baseline at the spike's target resolution and quality preset.
- Current planning estimate after a successful spike: roughly 6–9 full-time engineering months for
  the main move-family and complete show engine, and 12–18 months for broad release-quality polish.
  These are planning ranges, exclude uncertain asset production/licensing and increase substantially
  for part-time work.
- Entrances, interference and match playback precede staged promos/angles. Cages, ladders, battle
  royals, large multi-person structures and public creator tools expand only after representative
  singles/tag playback is convincing.

## Ownership

WM-043 owns the optional renderer and asset playback. WM-013 owns presentation events; WM-014 the
guaranteed text/2.5D viewer; WM-029 structured scenes; WM-034 production facts; WM-041 safe custom
content and optional 3D-pack manifests; WM-018 performance/accessibility evidence.

## Engine references

- [PlayCanvas React integration](https://developer.playcanvas.com/user-manual/react/getting-started/installation/)
- [PlayCanvas Engine compatibility](https://developer.playcanvas.com/user-manual/editor/engine-compatibility/)

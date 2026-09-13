# PD-112 — Match planning and live direction rules

Status: Accepted 12 September 2026. These rules govern future WM-006–009, WM-013–014, WM-031,
WM-034 and spike-gated WM-043 work. They do not describe the current prototype's complete behavior.

## Player promise

The player can agree a match with its participants, then react during the performance as a booker or
producer would: leave it alone, adjust pace or structure, protect someone, meet a broadcast deadline
or authorise a new finish. Changes travel through believable human communication, and workers respond
according to the situation instead of behaving as remote-controlled pieces.

## Before the match

- The accepted match brief records result, finish, time target, major bumps/spots, protected moments,
  sequence options, interference and safety limits. Important changes are put to affected workers.
- Most professional workers normally cooperate. Concern, negotiation, conditional acceptance or
  refusal requires a relevant cause such as health and safety, physical capability, insufficient
  rehearsal, creative control, trust, morale, status, relationships or a personality tendency.
- Egotistical or protective workers are not generically disobedient. They may specifically resist
  losing, reduced offence or time, an embarrassing presentation, a dangerous bump, making another
  worker look dominant, or a late change that harms their character. The UI explains the cause.
- The player may persuade, compromise, replace the spot or worker, use a contractual/governance power
  where one exists, or accept the refusal. Forcing a valid instruction does not erase later morale,
  trust, relationship, safety or reputation consequences.
- Pre-authorised contingencies define safe alternatives for injuries, time cuts, unavailable
  interference and failed sequences. They are used by Quick Sim and reduce live confusion.

## Communication opportunities

- Backstage management communicates to active wrestlers through the referee. A road agent, producer,
  booker or authorised manager may originate the instruction, but the referee is the in-ring relay.
- Delivery is not a fixed one- or two-minute pulse. Executed move and sequence definitions expose
  only two normal opportunity families: the referee checks a wrestler who is down on the mat or
  floor, or the referee plausibly separates/pulls back a wrestler during apparent over-attacking.
  Ringside checks and corner pull-backs are contextual variants of those families, not extra free windows.
- An opportunity records which participants can credibly hear, referee proximity, available cover,
  safe duration and reliability. It exists in Rust match state and WM-013 facts, never because an
  animation callback happened to reach a frame.
- Production capability determines how quickly an instruction reaches the referee. The instruction
  then waits for a compatible opportunity. The UI may estimate the next planned opening, but deviation,
  injury or failed execution can move or remove it.
- A ringside manager may create a distraction or previously understood cue, but cannot privately pass
  arbitrary backstage instructions by magic. Emergency stoppage remains available when waiting would
  be unsafe.
- Rest holds, elapsed time, camera cuts and convenient animation frames do not create communication
  access by themselves.

## Live instruction lifecycle

`Drafted → queued → production sent → referee received → opportunity found → participant response → applied/failed/expired`

- The player may give no instructions, one instruction or many. There is no artificial quota. Repeated,
  contradictory or complex changes increase cognitive load, missed cues and cohesion/safety risk.
- Supported families include pace/intensity; extend/cut time; go home; protect an injury; avoid a move;
  skip, replace, add or reorder a future sequence; redistribute future offence; change crowd strategy;
  change a spot; cue/cancel interference; and propose a different finish or winner.
- A result or finish change is a player-authorised canonical plan amendment, never an autonomous agent
  decision. Every essential participant must be informed before physical participation; an unbriefed
  surprise may exist for the audience, not as unsafe contact for a worker.
- Each worker may acknowledge, comply reluctantly, propose an alternative, partially execute,
  misunderstand, ignore or refuse. Most accept reasonable instructions. Response derives from the
  instruction, prior agreement, urgency, safety, ability, fatigue/injury, experience, psychology,
  adaptability, language, trust, morale, ego/status, relationships, contract rights and company culture.
- The road agent interprets the request into legal future beats and warns when it is impossible,
  contradictory or unlikely to reach everyone. Agents cannot silently override locked player intent.

## Incremental simulation and replanning

- Rust simulates and commits an immutable executed prefix. It plans only a bounded future horizon. A
  delivered and accepted instruction deterministically rebuilds the unsimulated suffix from the current
  legal match state, participant acknowledgements, remaining time, repertoire and contingency rules.
- Existing random state, instruction identity and event order are saved so different tick sizes,
  pause/resume and reload produce the same outcome. Already executed moves and communicated facts are
  never rewritten.
- WM-013 records the instruction lifecycle, delivery opportunity, worker responses, amended plan and
  causal consequences. Reports distinguish the original brief, live change and actual execution.
- A viewer may discard and regenerate only its buffered future presentation after an amendment. The
  2.5D or 3D animation script is derived output; it cannot apply the instruction or mutate match state.

## Time pressure and consequences

- WM-034 supplies broadcast blocks, commercial breaks, hard-outs and overrun costs. The live desk warns
  when current pacing threatens the allowed duration and can recommend cutting a sequence or going home.
- A high-priority go-home instruction still waits for a credible relay opportunity unless safety or an
  authorised emergency stoppage overrides normal performance. Missing the hard-out can affect ads,
  network trust, following segments, venue operation, audience experience and company finances.
- Last-minute bumps, sequence swaps, interference and finish changes may reduce cohesion or safety and
  can affect worker comfort, trust, morale, relationships, promises, character perception and crowd
  response. Consequences are contextual and explained; change is not automatically punished.
- WM-031 owns persistent morale and promises, WM-023 owns personal relationships, and WM-008 owns match,
  crowd, safety and career-consequence calibration. PD-112 emits evidence for those systems rather than
  duplicating their state.

## Presentation modes

- Every match, angle or other supported show segment has a show default and optional segment override:
  **Quick Sim**, **2.5D Extended Highlights**, **2.5D Realtime**, **3D Extended Highlights** or
  **3D Realtime**. Unavailable 3D falls back honestly to the complete 2.5D viewer.
- Quick Sim uses the accepted brief and pre-authorised contingencies without live prompts. Extended
  Highlights advances the same live simulation between selected beats and can pause for important
  management alerts, pending decisions and delivery responses. Realtime exposes the continuous live desk.
- Highlight selection, playback speed, pausing, skipping and renderer choice never change official
  facts. Only an explicit instruction accepted by authoritative Rust simulation can change the
  unsimulated future.

## Interface requirements

- The pre-match briefing shows each affected worker's acceptance, concern or refusal and the reason.
- The Live Direction panel shows the broadcast clock, current match phase, queued instruction, relay
  status, estimated next compatible opportunity, required acknowledgements and eventual response.
- Urgency and deadline are separate from delivery certainty. The UI never promises an exact time when
  the scripted sequence or match state cannot provide a safe opening.
- The player can cancel a still-undelivered instruction, inspect the agent's proposed translation and
  review after the match whether it was delivered, accepted and executed.

## Ownership

WM-006 owns agent interpretation, delivery and response rules; WM-007 owns sequence opportunities and
unsimulated-suffix replanning; WM-009 owns briefing/draft workflow; WM-013 owns presentation events;
WM-014 owns 2.5D controls; WM-031 owns persistent morale/promises; WM-034 owns broadcast constraints;
WM-043 owns only 3D playback of the resulting facts.

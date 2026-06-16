---
marp: true
theme: default
paginate: false
---

<!--
Status: draft
Kind: deck
Owner: interstate-comms

Truth label: concept. This is the public solution pitch for Interstate 2.0.
It does not name the supporting technology platform, claim final corridor
rankings, official endorsement, construction readiness, publication-grade SLA
proof, or ROI values.
-->

<style>
section {
  background: #111827;
  color: #f9fafb;
  font-family: "Aptos", "Segoe UI", sans-serif;
  padding: 36px 56px;
}
h1 {
  color: #fbbf24;
  font-size: 62px;
  letter-spacing: -1.2px;
}
h2 {
  color: #f9fafb;
  font-size: 36px;
  letter-spacing: -0.4px;
}
h3 {
  color: #bfdbfe;
  margin-top: 0;
}
strong {
  color: #fbbf24;
}
section.title {
  display: flex;
  flex-direction: column;
  justify-content: center;
}
section.title p {
  color: #d1d5db;
  font-size: 27px;
  max-width: 950px;
}
section.title::after {
  content: "";
  width: 70%;
  height: 8px;
  margin-top: 34px;
  border-radius: 999px;
  background: linear-gradient(90deg, #fbbf24, #60a5fa, #34d399);
}
.problem-board {
  display: grid;
  grid-template-columns: 1fr 1.05fr 1fr;
  grid-template-rows: repeat(2, 1fr);
  gap: 16px;
  margin-top: 24px;
}
.problem-board .hub {
  grid-column: 2;
  grid-row: 1 / span 2;
  border-radius: 50%;
  background: radial-gradient(circle at center, #fbbf24, #f97316);
  color: #111827;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 28px;
  font-size: 31px;
  font-weight: 900;
  min-height: 260px;
}
.card {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 18px;
  padding: 19px;
}
.card p,
.caption,
.panel p {
  color: #e5e7eb;
  font-size: 18px;
  line-height: 1.35;
  margin: 0;
}
.ladder {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 14px;
  margin-top: 28px;
}
.rung {
  background: linear-gradient(180deg, #1f2937, #111827);
  border: 1px solid #4b5563;
  border-radius: 18px;
  padding: 18px;
  min-height: 235px;
}
.rung .tier {
  color: #93c5fd;
  font-weight: 800;
  font-size: 28px;
}
.rung .promise {
  color: #fbbf24;
  font-size: 32px;
  font-weight: 900;
  margin: 8px 0;
}
.before-after {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-top: 26px;
}
.panel {
  border-radius: 22px;
  padding: 24px;
  min-height: 330px;
}
.before {
  background: #2a1f1f;
  border: 2px solid #ef4444;
}
.after {
  background: #13251e;
  border: 2px solid #34d399;
}
.stack {
  display: grid;
  gap: 12px;
  margin-top: 18px;
}
.stack div {
  border-radius: 14px;
  padding: 12px 16px;
  color: #111827;
  font-weight: 800;
}
.flat { background: #fecaca; }
.t1 { background: #fbbf24; }
.t2 { background: #93c5fd; }
.t3 { background: #a7f3d0; }
.t4 { background: #ddd6fe; }
.road {
  display: grid;
  grid-template-columns: 1fr 1.2fr 1fr;
  gap: 20px;
  align-items: center;
  margin-top: 18px;
}
.road-card {
  background: #1f2937;
  border: 1px solid #4b5563;
  border-radius: 18px;
  padding: 18px;
  min-height: 114px;
  margin-bottom: 16px;
}
.road-card p {
  color: #d1d5db;
  margin: 0;
  font-size: 16px;
}
.road-core {
  min-height: 360px;
  border-radius: 26px;
  background:
    linear-gradient(90deg, transparent 48%, #fbbf24 48%, #fbbf24 52%, transparent 52%),
    repeating-linear-gradient(180deg, #0f172a 0 42px, #1f2937 42px 84px);
  border: 3px solid #60a5fa;
  display: flex;
  align-items: center;
  justify-content: center;
}
.road-core div {
  background: #fbbf24;
  color: #111827;
  border-radius: 999px;
  padding: 24px 32px;
  font-size: 34px;
  font-weight: 900;
  text-align: center;
}
.map-hero,
.wide-map {
  display: grid;
  grid-template-columns: 1.35fr 0.65fr;
  gap: 24px;
  align-items: center;
}
.map-hero img,
.wide-map img,
.map-grid img {
  width: 100%;
  border-radius: 20px;
  border: 2px solid #374151;
  background: #ffffff;
  box-shadow: 0 18px 50px rgba(0, 0, 0, 0.35);
}
.wide-map img {
  max-height: 500px;
  object-fit: contain;
}
.side-list {
  display: grid;
  gap: 12px;
}
.side-list div {
  background: #1f2937;
  border-left: 5px solid #34d399;
  border-radius: 12px;
  padding: 13px 15px;
  color: #e5e7eb;
  font-size: 17px;
}
.map-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 14px;
  margin-top: 16px;
}
.map-card {
  background: #1f2937;
  border: 1px solid #374151;
  border-radius: 18px;
  padding: 10px;
}
.map-card img {
  height: 145px;
  object-fit: cover;
  object-position: center;
}
.map-card h3 {
  font-size: 18px;
  margin: 8px 0 4px;
}
.map-card p {
  color: #e5e7eb;
  font-size: 13px;
  margin: 0;
}
.relay-board {
  display: grid;
  grid-template-columns: 0.9fr 1.1fr;
  gap: 26px;
  align-items: center;
}
.quote {
  border-left: 6px solid #fbbf24;
  padding-left: 20px;
  color: #e5e7eb;
  font-size: 30px;
  line-height: 1.25;
}
.hub-ring {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 14px;
}
.hub-ring div {
  background: #1f2937;
  border: 1px solid #60a5fa;
  border-radius: 18px;
  padding: 18px;
  min-height: 100px;
  color: #e5e7eb;
}
.hub-ring strong {
  display: block;
  margin-bottom: 8px;
}
.win-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
  margin-top: 22px;
}
.win-grid .card {
  border-color: #34d399;
  min-height: 130px;
}
.takeaway,
.cta {
  margin-top: 24px;
  border-radius: 18px;
  background: linear-gradient(90deg, #fbbf24, #f97316);
  color: #111827;
  padding: 18px 24px;
  font-size: 25px;
  font-weight: 900;
  text-align: center;
}
.note {
  margin-top: 16px;
  border-radius: 14px;
  border: 1px solid rgba(251, 191, 36, 0.5);
  background: rgba(251, 191, 36, 0.1);
  color: #fde68a;
  padding: 12px 16px;
  font-size: 18px;
  line-height: 1.25;
}
</style>

<!-- _class: title -->

# Interstate 2.0 should be a promise, not a map.

A national service network for freight reliability, resilience, rural access, electrification, automation, and regional growth.

---

<!-- _class: title -->

# The old system was built for another era.

**The next one has to organize freight, energy, labor, technology, and resilience as one national operating system.**

---

## The problem leaders already recognize

<div class="problem-board">
  <div class="card">
    <h3>Gridlock</h3>
    <p>Ports, metros, mountain passes, and interchanges can break national freight reliability.</p>
  </div>
  <div class="hub">No national service standard</div>
  <div class="card">
    <h3>No organized resiliency</h3>
    <p>Closures and shocks are handled locally, not as a designed national redundancy system.</p>
  </div>
  <div class="card">
    <h3>Inconsistent freight lanes</h3>
    <p>Freight priority appears corridor by corridor instead of as a national operating layer.</p>
  </div>
  <div class="card">
    <h3>The new freight era needs nodes</h3>
    <p>EV charging, autonomous trunking, relay labor, and hubs need planned nodes.</p>
  </div>
</div>

---

## The answer: a national service promise ladder

<div class="ladder">
  <div class="rung">
    <div class="tier">T1</div>
    <div class="promise">48h / 36h</div>
    <p>National timed-freight spine for coast-to-coast and half-continent movement.</p>
  </div>
  <div class="rung">
    <div class="tier">T2</div>
    <div class="promise">24h/12h</div>
    <p>Regional connectors, relief corridors, ports, borders, and mega-region service.</p>
  </div>
  <div class="rung">
    <div class="tier">T3</div>
    <div class="promise">6h</div>
    <p>Feeder access for production zones, smaller metros, rural regions, and logistics anchors.</p>
  </div>
  <div class="rung">
    <div class="tier">T4</div>
    <div class="promise">1h</div>
    <p>Terminal, port, yard, warehouse, border, and last-mile freight access.</p>
  </div>
</div>

<div class="takeaway">The question changes: what should the network promise?</div>

<div class="note">These are planning targets for comparing service roles, not verified operating guarantees.</div>

---

## Roads need the hierarchy rail already has

<div class="before-after">
  <div class="panel before">
    <h3>Today: one flat category</h3>
    <p>Interstates are mostly treated as the same kind of asset. That makes the country argue corridor by corridor instead of by service role.</p>
    <div class="stack">
      <div class="flat">Interstate = Interstate = Interstate</div>
      <div class="flat">Same label, different national value</div>
      <div class="flat">Priorities blur together</div>
    </div>
  </div>
  <div class="panel after">
    <h3>Interstate 2.0: a service network</h3>
    <p>Rail and metro systems already use hierarchy: express, regional, feeder, terminal. Roads need the same operating logic.</p>
    <div class="stack">
      <div class="t1">T1 national spine — 48h / 36h</div>
      <div class="t2">T2 regional — 24h/12h</div>
      <div class="t3">T3 feeder access — 6h</div>
      <div class="t4">T4 terminal access — 1h</div>
    </div>
  </div>
</div>

---

## T1: where the country buys reliability

<div class="road">
  <div>
    <div class="road-card">
      <h3>Roadway needs</h3>
      <p>Surface, safety, capacity, and reliability work where the national promise depends on it.</p>
    </div>
    <div class="road-card">
      <h3>Managed lanes</h3>
      <p>Premium freight lanes where service windows justify the investment.</p>
    </div>
  </div>
  <div class="road-core"><div>48h / 36h<br/>national service</div></div>
  <div>
    <div class="road-card">
      <h3>Interchange fixes</h3>
      <p>Targeted bottleneck and junction repairs before giant corridor spending.</p>
    </div>
    <div class="road-card">
      <h3>Resilience + alternatives</h3>
      <p>Redundancy for closures, weather, mountain passes, ports, and national shocks.</p>
    </div>
  </div>
</div>

---

## The promise has to be visible

<div class="map-hero">
  <div>
    <img src="../../maps/beck-schematic.png" alt="Interstate 2.0 schematic map" />
  </div>
  <div>
    <h3>From highway map to service map</h3>
    <p class="caption">A national schematic makes the promise visible before every investment detail is final.</p>
    <p class="caption"><br/>At pitch scale, the point is the operating idea: spine, connectors, access, hubs, and handoffs.</p>
    <p class="caption"><br/>Detailed labels and proof belong in the evidence package.</p>
  </div>
</div>

---

## Every tier has a job

<div class="win-grid">
  <div class="card"><h3>T1 national spine</h3><p>Buy reliability where the whole economy depends on it.</p></div>
  <div class="card"><h3>T2 regional engine</h3><p>Connect mega-regions, ports, borders, and relief corridors.</p></div>
  <div class="card"><h3>T3 access layer</h3><p>Keep production zones, rural regions, and smaller metros visible.</p></div>
  <div class="card"><h3>T4 terminal layer</h3><p>Make ports, yards, warehouses, gates, and local freight districts work.</p></div>
  <div class="card"><h3>Relay hubs</h3><p>Turn long-haul movement into scheduled handoffs and regional jobs.</p></div>
  <div class="card"><h3>Resilience</h3><p>Design alternatives before closures, shocks, and disasters expose the gap.</p></div>
</div>

---

## Relay hubs: the aviation model for freight

<div class="relay-board">
  <div class="quote">
    Pilots use crew bases, duty rules, handoffs, maintenance systems, and hub operations.
    <br/><br/>
    Premium freight can evolve the same way.
  </div>
  <div class="hub-ring">
    <div><strong>Crew base</strong>Regional shifts and professional freight chauffeurs.</div>
    <div><strong>Charge + service</strong>Known EV, maintenance, and staging nodes.</div>
    <div><strong>Handoff point</strong>Scheduled load, driver, and future AV transfers.</div>
    <div><strong>State asset</strong>Jobs, hubs, and explainable infrastructure packages.</div>
  </div>
</div>

---

## The win-win

<div class="win-grid">
  <div class="card"><h3>States</h3><p>A stronger case for nationally relevant projects.</p></div>
  <div class="card"><h3>Industry</h3><p>Better service windows and a place to put operational requirements.</p></div>
  <div class="card"><h3>Funders</h3><p>Staged packages instead of one impossible mega-ask.</p></div>
  <div class="card"><h3>Drivers</h3><p>A premium regional career path, not just punishing long-haul work.</p></div>
  <div class="card"><h3>Communities</h3><p>Concerns enter the plan early, before positions harden.</p></div>
  <div class="card"><h3>Innovation</h3><p>A real deployment path for electric and automated freight.</p></div>
</div>

---

## What people should feel

<div class="win-grid">
  <div class="card"><h3>Farm regions</h3><p>Production zones stay visible when metro congestion dominates the funding story.</p></div>
  <div class="card"><h3>Small metros</h3><p>Regional access becomes part of the promise, not an afterthought.</p></div>
  <div class="card"><h3>Drivers</h3><p>Relay hubs create a path toward safer, more predictable regional work.</p></div>
  <div class="card"><h3>Emergency access</h3><p>Evacuation, winter closures, wildfire, flood, and port disruption get planned before crisis.</p></div>
  <div class="card"><h3>Communities</h3><p>Noise, air, safety, displacement, and local access enter before concrete.</p></div>
  <div class="card"><h3>Funders</h3><p>The vision becomes a staged package with checks, holds, and visible tradeoffs.</p></div>
</div>

---

## Who is at the table before concrete?

<div class="win-grid">
  <div class="card"><h3>State DOTs</h3><p>Funding match, maintenance burden, right-of-way, environmental review, delivery phasing.</p></div>
  <div class="card"><h3>Freight operators</h3><p>Delay windows, bottlenecks, terminal access, relay needs, parking, weights, clearances.</p></div>
  <div class="card"><h3>Rural regions</h3><p>Agriculture, production zones, healthcare reach, evacuation, and access to the national system.</p></div>
  <div class="card"><h3>Communities</h3><p>Noise, safety, displacement, first/last-mile access, and concerns before positions harden.</p></div>
  <div class="card"><h3>Environment</h3><p>Air quality, runoff, habitat, climate exposure, and mitigation before claims advance.</p></div>
  <div class="card"><h3>Funders</h3><p>Evidence labels, staged packages, exclusions, uncertainty, and decision gates.</p></div>
</div>

---

## Make it safe to say yes

<div class="before-after">
  <div class="panel before">
    <h3>Risky ask</h3>
    <p>A finished national build map before states, operators, communities, and evidence have shaped it.</p>
    <div class="stack">
      <div class="flat">Promises sound like guarantees</div>
      <div class="flat">Tradeoffs stay hidden</div>
      <div class="flat">Public trust arrives too late</div>
    </div>
  </div>
  <div class="panel after">
    <h3>Fundable ask</h3>
    <p>A staged national program that starts with standards, intake, evidence, and pilots.</p>
    <div class="stack">
      <div class="t1">Vision leaders can explain</div>
      <div class="t2">Evidence funders can inspect</div>
      <div class="t3">Pilots states can deliver</div>
    </div>
  </div>
</div>

---

## The fundable sequence

<div class="win-grid">
  <div class="card"><h3>1. Story</h3><p>Make the national promise legible to leaders, states, industry, and communities.</p></div>
  <div class="card"><h3>2. Evidence</h3><p>Separate what is ready, what needs study, and what must wait before investment claims.</p></div>
  <div class="card"><h3>3. Pilots</h3><p>Start with hubs, charging, interchanges, state views, delivery checks, and service demonstrations.</p></div>
</div>

<div class="cta">Fund the Interstate 2.0 story, evidence, and pilot package.</div>

---

<!-- _class: title -->

# Interstate 2.0 is the vision.

**A national service network people can fund, refine, and believe in.**

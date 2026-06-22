---
marp: true
theme: default
paginate: false
---

<!--
Status: draft
Kind: deck
Owner: route-comms

Truth label: sales-ready / evidence-bounded. This deck sells the ROUTE state
service-network diagnostic, DCR expansion path, and Texas paid-pilot pathway.
It does not claim
TxDOT endorsement, official-plan status, legal SLA readiness, construction
readiness, numeric ROI, procurement readiness, validation, public readiness, or
source-backed full inventory.
-->

<style>
section {
  background: #0f172a;
  color: #f8fafc;
  font-family: "Aptos", "Segoe UI", sans-serif;
  padding: 34px 54px;
}
h1 {
  color: #f8fafc;
  font-size: 58px;
  letter-spacing: 0;
  line-height: 1.02;
}
h2 {
  color: #f8fafc;
  font-size: 36px;
  letter-spacing: 0;
}
h3 {
  color: #bfdbfe;
  margin: 0 0 8px;
}
strong {
  color: #fbbf24;
}
section.title {
  display: grid;
  grid-template-columns: 0.9fr 1.1fr;
  gap: 34px;
  align-items: center;
}
section.title p {
  color: #cbd5e1;
  font-size: 25px;
  line-height: 1.25;
}
.hero-map img,
.map-card img {
  width: 100%;
  border-radius: 8px;
  border: 1px solid #334155;
  background: #ffffff;
  box-shadow: 0 22px 60px rgba(0, 0, 0, 0.35);
}
.eyebrow {
  display: inline-block;
  color: #111827;
  background: #fbbf24;
  border-radius: 999px;
  padding: 7px 12px;
  font-size: 16px;
  font-weight: 800;
  margin-bottom: 16px;
}
.grid2,
.grid3,
.grid4 {
  display: grid;
  gap: 16px;
  margin-top: 22px;
}
.grid2 { grid-template-columns: repeat(2, 1fr); }
.grid3 { grid-template-columns: repeat(3, 1fr); }
.grid4 { grid-template-columns: repeat(4, 1fr); }
.card,
.panel,
.rail {
  background: #162033;
  border: 1px solid #334155;
  border-radius: 8px;
  padding: 18px;
}
.card p,
.panel p,
.rail p,
.caption {
  color: #cbd5e1;
  font-size: 18px;
  line-height: 1.32;
  margin: 0;
}
.card.good { border-color: #34d399; }
.card.warn { border-color: #fbbf24; }
.card.blue { border-color: #60a5fa; }
.big {
  color: #fbbf24;
  font-size: 48px;
  font-weight: 900;
  line-height: 1;
}
.takeaway {
  margin-top: 20px;
  background: rgba(251, 191, 36, 0.12);
  border: 1px solid rgba(251, 191, 36, 0.55);
  border-radius: 8px;
  color: #fde68a;
  padding: 13px 16px;
  font-size: 24px;
  font-weight: 900;
  text-align: center;
}
.flow {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 10px;
  margin-top: 24px;
}
.flow .step {
  min-height: 112px;
  border-radius: 8px;
  border: 1px solid #60a5fa;
  background: #162033;
  padding: 13px;
}
.step b {
  display: block;
  color: #fbbf24;
  font-size: 19px;
  margin-bottom: 8px;
}
.step span {
  color: #cbd5e1;
  font-size: 15px;
  line-height: 1.22;
}
.ladder .rail {
  min-height: 185px;
}
.tier {
  color: #93c5fd;
  font-size: 30px;
  font-weight: 900;
}
.role {
  color: #fbbf24;
  font-size: 25px;
  font-weight: 900;
  margin: 8px 0;
}
.split {
  display: grid;
  grid-template-columns: 1.05fr 0.95fr;
  gap: 24px;
  align-items: center;
}
.mock {
  background: #e5e7eb;
  color: #111827;
  border-radius: 8px;
  padding: 14px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.32);
}
.mock-top {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  background: #111827;
  color: #f8fafc;
  border-radius: 6px;
  padding: 10px 12px;
  font-weight: 800;
}
.mock-body {
  display: grid;
  grid-template-columns: 0.75fr 1.35fr 0.9fr;
  gap: 10px;
  margin-top: 10px;
}
.mock-list,
.mock-map,
.mock-dials {
  background: #ffffff;
  border: 1px solid #cbd5e1;
  border-radius: 6px;
  padding: 10px;
  min-height: 280px;
}
.mock-map {
  background:
    linear-gradient(35deg, transparent 44%, #fbbf24 44%, #fbbf24 48%, transparent 48%),
    linear-gradient(145deg, transparent 52%, #60a5fa 52%, #60a5fa 56%, transparent 56%),
    radial-gradient(circle at 28% 36%, #0f172a 0 9px, transparent 10px),
    radial-gradient(circle at 70% 30%, #0f172a 0 9px, transparent 10px),
    radial-gradient(circle at 52% 70%, #0f172a 0 9px, transparent 10px),
    #f8fafc;
}
.mock-chip {
  border-radius: 999px;
  padding: 7px 9px;
  background: #e0f2fe;
  color: #0f172a;
  font-size: 13px;
  font-weight: 800;
  margin-bottom: 8px;
}
.dial {
  height: 10px;
  border-radius: 999px;
  background: linear-gradient(90deg, #60a5fa 0 65%, #e5e7eb 65%);
  margin: 10px 0 16px;
}
.gate {
  display: grid;
  grid-template-columns: 0.8fr 1.2fr;
  gap: 20px;
  align-items: stretch;
}
.gate .left {
  border-radius: 8px;
  background: linear-gradient(180deg, #fbbf24, #f97316);
  color: #111827;
  padding: 24px;
}
.gate .left h3 {
  color: #111827;
  font-size: 34px;
}
.gate .left p {
  color: #111827;
  font-size: 22px;
  font-weight: 800;
}
table {
  background: #f8fafc;
  color: #111827;
  border-radius: 8px;
  overflow: hidden;
  font-size: 17px;
}
th {
  background: #fde68a;
  color: #111827;
}
td, th {
  padding: 9px 11px;
}
.cta {
  margin-top: 22px;
  border-radius: 8px;
  background: linear-gradient(90deg, #fbbf24, #f97316);
  color: #111827;
  padding: 18px 24px;
  font-size: 27px;
  font-weight: 900;
  text-align: center;
}
</style>

<!-- _class: title -->

<div>
  <div class="eyebrow">ROUTE state service-network diagnostic</div>
  <h1>Sell the network job, not another road map.</h1>
  <p>ROUTE turns a state or authority's priorities into a service hierarchy, failure ledger, evidence boundary, and paid pilot path.</p>
</div>
<div class="hero-map">
  <img src="../../maps/all-tiers-v2.png" alt="ROUTE all tiers structural map" />
  <p class="caption">Map level: structural. Excluded claims: official plan, legal SLA, construction readiness, numeric ROI, validation, public readiness.</p>
</div>

---

## The buyer already has maps

<div class="grid3">
  <div class="card warn">
    <h3>Maps show ownership</h3>
    <p>They rarely say what each link must do for the state's economy, resilience, and access promises.</p>
  </div>
  <div class="card warn">
    <h3>Project lists show demand</h3>
    <p>They blur operations, asset repair, access, resilience, terminal, and capital decisions together.</p>
  </div>
  <div class="card warn">
    <h3>Dashboards show symptoms</h3>
    <p>They often miss the service promise that makes a failure unacceptable.</p>
  </div>
</div>

<div class="takeaway">ROUTE adds the missing layer: role, promise, failure, evidence, and next decision.</div>

---

## What ROUTE sells

<div class="grid4 ladder">
  <div class="rail">
    <div class="tier">T1</div>
    <div class="role">Statewide trunk</div>
    <p>Top city, gateway, and economic spine promises.</p>
  </div>
  <div class="rail">
    <div class="tier">T2</div>
    <div class="role">Regional connector</div>
    <p>Market, relief, and regional access roles.</p>
  </div>
  <div class="rail">
    <div class="tier">T3</div>
    <div class="role">Rural mesh</div>
    <p>Production, healthcare, small-market, and continuity needs.</p>
  </div>
  <div class="rail">
    <div class="tier">T4 / R / M / X</div>
    <div class="role">Access and discipline</div>
    <p>Terminal access, resilience overlay, maintenance-only, and explicit non-promotion.</p>
  </div>
</div>

<div class="takeaway">The product is not promotion. It is full-system role assignment with evidence holds.</div>

---

## The state product has a real workflow

<div class="flow">
  <div class="step"><b>1. Intake</b><span>Top places, failures, source owners, and claim boundaries.</span></div>
  <div class="step"><b>2. Payload</b><span>Segments, nodes, failures, terminals, and non-promotion rows.</span></div>
  <div class="step"><b>3. Custody</b><span>Source identity, row traceability, scope labels, review disposition.</span></div>
  <div class="step"><b>4. Fit</b><span>Candidate T1/T2/T3/T4/R/M/X roles for the pilot scope.</span></div>
  <div class="step"><b>5. Review</b><span>Client owner marks each role pass, hold, or fail.</span></div>
  <div class="step"><b>6. Readout</b><span>Executive story, failure modes, evidence gaps, next package.</span></div>
</div>

<div class="takeaway">Every step can stop cleanly instead of overclaiming.</div>

---

## Texas proves the sale path

<div class="grid3">
  <div class="card good">
    <div class="big">01</div>
    <h3>Buyer review</h3>
    <p>Go/no-go memo, agenda, source request, and safe answers for a sponsor conversation.</p>
  </div>
  <div class="card good">
    <div class="big">02</div>
    <h3>Paid pilot scope</h3>
    <p>Phases, deliverables, acceptance gates, and non-fit responses.</p>
  </div>
  <div class="card good">
    <div class="big">03</div>
    <h3>Kickoff readiness</h3>
    <p>Sponsor, scope, source owners, data handling, review cadence, and claim boundary before start.</p>
  </div>
</div>

<div class="takeaway">Texas is now a sales motion, not just a sample map.</div>

---

## What the buyer receives

<div class="grid3">
  <div class="card blue"><h3>Pilot scope sheet</h3><p>What is in, what is out, who owns sources, and which claims stay held.</p></div>
  <div class="card blue"><h3>Source custody ledger</h3><p>Which rows support analysis, which rows are source-needed, and why.</p></div>
  <div class="card blue"><h3>Candidate hierarchy</h3><p>T1/T2/T3/T4/R/M/X candidate roles for the scoped network.</p></div>
  <div class="card blue"><h3>Failure-mode scorecard</h3><p>Closure, restriction, bottleneck, terminal, recovery, and access gaps.</p></div>
  <div class="card blue"><h3>Investment question backlog</h3><p>Studies, pilots, operations, access, asset, and capital questions grouped by role.</p></div>
  <div class="card blue"><h3>Executive readout</h3><p>A leadership story that names evidence boundaries instead of hiding them.</p></div>
</div>

---

## The future UI makes it a working session

<div class="split">
  <div>
    <h2>Clients should edit the network live.</h2>
    <p class="caption">Preload the state, show candidate roles, expose held claims, then let the buyer add places, change tiers, adjust dials, and export a bounded readout.</p>
    <div class="takeaway">This turns ROUTE from report delivery into a planning workbench.</div>
  </div>
  <div class="mock">
    <div class="mock-top"><span>Texas Service Network</span><span>evidence: source-needed</span></div>
    <div class="mock-body">
      <div class="mock-list">
        <div class="mock-chip">Dallas-Fort Worth</div>
        <div class="mock-chip">Houston / Gulf</div>
        <div class="mock-chip">Border gateways</div>
        <div class="mock-chip">Energy regions</div>
        <div class="mock-chip">Rural access</div>
      </div>
      <div class="mock-map"></div>
      <div class="mock-dials">
        <b>Reliability</b><div class="dial"></div>
        <b>Freight priority</b><div class="dial"></div>
        <b>Rural access</b><div class="dial"></div>
        <b>Resilience</b><div class="dial"></div>
      </div>
    </div>
  </div>
</div>

---

## DCR turns the package into operations

<div class="grid3">
  <div class="card good">
    <h3>Monitor the promise</h3>
    <p>Track reliability, incidents, closures, assets, terminal access, EV charging stress, and evidence drift.</p>
  </div>
  <div class="card good">
    <h3>Simulate the switch</h3>
    <p>Replay closure, weather, EV range, signage, detour, terminal, and package scenarios before committing.</p>
  </div>
  <div class="card good">
    <h3>Export the decision</h3>
    <p>Produce claim-safe readouts for reroute, signage, EV support, recovery, access, asset, or investment posture.</p>
  </div>
</div>

<div class="takeaway">DCR is the recurring product: the Decision Control Room for the service network.</div>

---

## What DCR decides

<table>
  <thead><tr><th>Decision</th><th>Trigger</th><th>Output</th></tr></thead>
  <tbody>
    <tr><td>Reroute</td><td>Closure, degradation, restriction, shared hazard</td><td>Preferred alternate and blocked alternates</td></tr>
    <tr><td>Signage</td><td>Drivers need earlier route, charge, or detour choice</td><td>Message intent, location theme, and timing</td></tr>
    <tr><td>EV support</td><td>Charger outage, detour length, weather range loss</td><td>EV-sensitive path and staging/support note</td></tr>
    <tr><td>Recovery</td><td>Incident duration exceeds target window</td><td>Escalation, staging, and communication posture</td></tr>
    <tr><td>Investment</td><td>Recurring monitored failures outrank old sequence</td><td>Re-ranked operations, asset, access, resilience, or capital package</td></tr>
  </tbody>
</table>

<div class="takeaway">ROUTE recommends and documents. The operator retains authority.</div>

---

## The kickoff gate protects the buyer

<div class="gate">
  <div class="left">
    <h3>Do not start until this is true</h3>
    <p>Sponsor, scope, source owners, data handling, review cadence, and claim boundary are confirmed.</p>
  </div>
  <div class="grid2" style="margin-top:0">
    <div class="card warn"><h3>Workshop only</h3><p>Interest exists, but source owners or scope are missing.</p></div>
    <div class="card warn"><h3>Procurement hold</h3><p>Buyer needs price quote, contracting, or purchasing artifacts first.</p></div>
    <div class="card warn"><h3>Source hold</h3><p>Payloads or custody metadata cannot support source-backed fit yet.</p></div>
    <div class="card warn"><h3>Claim hold</h3><p>Buyer wants official, SLA, ROI, construction, validation, approval, or public claims too early.</p></div>
  </div>
</div>

---

## Safe answers to hard buyer questions

| Question | Answer |
|---|---|
| Is this a DOT plan? | No. It is a diagnostic review packet until an authorized external review says otherwise. |
| Can we use the hierarchy publicly? | Not from this packet. Public-readiness stays held until release review and claim approval. |
| Does this guarantee service? | No. The pilot defines candidate promises and evidence gaps; legal SLA claims remain held. |
| Will this produce ROI? | It can define the ROI evidence contract; numeric ROI remains held. |
| Is this a construction recommendation? | No. It structures next studies, pilots, and decision packages. |

---

## Why this is valuable even before a full inventory

<div class="grid3">
  <div class="card good"><h3>It exposes missing owners</h3><p>States learn which source surfaces block stronger claims.</p></div>
  <div class="card good"><h3>It separates service from politics</h3><p>The discussion moves from "my road" to "what job does this link perform?"</p></div>
  <div class="card good"><h3>It prevents fake certainty</h3><p>Unsupported official, SLA, ROI, construction, approval, and validation claims stay held.</p></div>
</div>

<div class="takeaway">The first paid product is a better decision process.</div>

---

## The first close

<div class="grid3">
  <div class="card blue"><h3>Pick the pilot scope</h3><p>State, authority, corridor, port region, gateway, or freight coalition.</p></div>
  <div class="card blue"><h3>Name source owners</h3><p>Segment inventory, priority nodes, failure evidence, terminal access, non-promotion, claim boundary.</p></div>
  <div class="card blue"><h3>Choose the path</h3><p>Diagnostic first, DCR tabletop, 30-90 day DCR pilot, or workshop-only hold.</p></div>
</div>

<div class="cta">Start with a bounded service-network diagnostic, then keep it alive with DCR.</div>

---

<!-- _class: title -->

<div>
  <div class="eyebrow">ROUTE</div>
  <h1>The state sells itself when the network job is visible.</h1>
  <p>ROUTE gives a buyer the service hierarchy, evidence discipline, and kickoff gate needed to start without pretending the plan is finished.</p>
</div>
<div class="hero-map">
  <img src="../../maps/t3-texas-border.png" alt="ROUTE Texas border structural map" />
  <p class="caption">Map level: structural regional view. Use for service discussion only; not proof of official route status, terminal readiness, SLA, ROI, or construction.</p>
</div>

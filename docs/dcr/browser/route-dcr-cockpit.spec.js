const { test, expect } = require("@playwright/test");
const path = require("path");

const pagePath = "file:///" + path.resolve(__dirname, "route-dcr-cockpit.html").replace(/\\/g, "/");

test.describe("ROUTE DCR cockpit", () => {
  test("desktop exposes the simulated operating board and claim boundaries", async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto(pagePath);

    await expect(page.getByRole("heading", { name: "ROUTE DCR Cockpit", exact: true })).toBeVisible();
    await expect(page.getByLabel("Simulated service network")).toBeVisible();
    await expect(page.getByText("Mode advisory simulation")).toBeVisible();
    await expect(page.getByText("traffic control, legal detour, SLA, EV availability")).toBeVisible();
    await expect(page.locator("#active-case")).toHaveText("Winter closure and EV stress");
    await expect(page.locator("#promise-risk")).toHaveText("46");
    await expect(page.locator("#account-name")).toHaveText("Avery Chen");
    await expect(page.locator("#signal-count")).toHaveText("6 active");
    await expect(page.locator("#express-demand")).toHaveText("38%");
    await expect(page.locator("#paid-requests")).toHaveText("0");
    await expect(page.locator("#revenue-proxy")).toHaveText("$0");
    await expect(page.getByLabel("Run summary")).toContainText("baseline");
    await expect(page.locator("#summary-outcome")).toHaveText("standby");
    await expect(page.locator("#scorecard-grade")).toHaveText("DCR-0");
    await expect(page.locator("#score-renewal")).toHaveText("watch");
    await expect(page.getByLabel("Operator SLA")).toContainText("Claimbounded");
    await expect(page.locator("#sla-status")).toHaveText("simulated");
    await expect(page.locator("#sla-detect")).toHaveText("pending");
    await expect(page.getByLabel("Guidance board")).toContainText("Routemonitor base route");
    await expect(page.locator("#guidance-status")).toHaveText("draft");
    await expect(page.locator("#guidance-source")).toHaveText("source-needed");
    await expect(page.getByLabel("Workload board")).toContainText("Open Actions0");
    await expect(page.locator("#workload-status")).toHaveText("held");
    await expect(page.locator("#workload-source")).toHaveText("3");
    await expect(page.locator("#workload-next")).toHaveText("verify 511 closure feed");
    await expect(page.getByLabel("Maintenance case")).toContainText("Measured Failurecorridor reliability failure");
    await expect(page.locator("#maintenance-fix")).toHaveText("weather-responsive split + EV staging");
    await expect(page.locator("#maintenance-priority")).toHaveText("P2");
    await expect(page.locator("#maintenance-thresholds")).toHaveText("risk 46>=45");
    await expect(page.locator("#maintenance-evidence")).toHaveText("511 closure feed; Charging source owner; Operator message owner");
    await expect(page.getByLabel("Fix package")).toContainText("OwnerDOT operations");
    await expect(page.locator("#fix-status")).toHaveText("blocked");
    await expect(page.locator("#fix-scope")).toHaveText("weather-responsive split + EV staging");
    await expect(page.locator("#fix-approval")).toHaveText("source custody");
    await expect(page.getByLabel("Decision dossier")).toContainText("DecisionP2 weather-responsive split + EV staging");
    await expect(page.locator("#dossier-status")).toHaveText("blocked");
    await expect(page.locator("#dossier-blockers")).toHaveText("source custody: verify 511 closure feed");
    await expect(page.getByLabel("Portfolio proof")).toContainText("Saved Runs0");
    await expect(page.locator("#portfolio-grade")).toHaveText("0 runs");
    await expect(page.getByLabel("Renewal backlog")).toContainText("No saved renewal work");
    await expect(page.locator("#renewal-backlog-count")).toHaveText("0 items");
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/ROUTE DCR shift handoff - 00:00/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Next action: verify 511 closure feed/);
    await expect(page.getByLabel("Scenario run plan")).toContainText("Primary pass restriction crosses promise-risk threshold.");
    await expect(page.getByLabel("Evidence gates")).toContainText("511 closure feed");
    await expect(page.locator("#gate-count")).toHaveText("0/3 ready");
    await expect(page.getByLabel("Timeline queue")).toContainText("00:10 Snow band reduces alternate reliability");
    await expect(page.getByLabel("Action queue")).toContainText("No open action");
    await expect(page.locator("#run-owner")).toHaveText("DOT operations");
    await expect(page.getByText("Held traffic control, legal detour, SLA, EV availability, pricing authority")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/held_claims/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/sla_boundary,"simulated timing; no guaranteed SLA"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/guidance_boundary,"advisory, not field command"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/workload_queue,"source custody"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/maintenance_boundary,"recommendation only; owner approval required"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/maintenance_thresholds,"risk 46>=45"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/fix_boundary,"recommendation package, not a work order"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/dossier_boundary,"advisory dossier, not procurement approval"/);
  });

  test("controls create and approve switch packets without changing authority boundary", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await page.getByRole("button", { name: "Reroute" }).click();
    await expect(page.getByText("Reroute advisory created from simulated signal review.")).toBeVisible();
    await expect(page.locator("#decision-count")).toHaveText("0 reviewed");

    await page.getByLabel("Active role").selectOption("operator");
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.locator("#decision-count")).toHaveText("1 reviewed");
    await expect(page.getByText("Operator reviewed advisory. ROUTE still does not command field devices, set prices, or publish claims.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/operator_status,"approved"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/active_role,"Operator"/);
  });

  test("scenario and injected signals update cockpit state and readout export", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 840 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await page.getByLabel("Scenario case").selectOption("terminal-access");
    await expect(page.locator("#active-case")).toHaveText("Terminal access disruption");
    await expect(page.locator("#promise-risk")).toHaveText("52");
    await expect(page.locator("#run-owner")).toHaveText("Port operations");
    await expect(page.locator("#source-status")).toHaveText("source-needed");
    await expect(page.getByLabel("Scenario run plan")).toContainText("Terminal queue owner plus local truck-route authority required.");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/operator_owner,"Port operations"/);
    await expect(page.locator("#guidance-route")).toHaveText("draft reroute split");
    await expect(page.locator("#guidance-signage")).toHaveText("queue early signage");
    await expect(page.locator("#maintenance-failure")).toHaveText("intersection access failure");
    await expect(page.locator("#maintenance-fix")).toHaveText("retime gate approach routing");
    await expect(page.locator("#maintenance-priority")).toHaveText("P1");
    await expect(page.locator("#maintenance-thresholds")).toHaveText("risk 52>=45; flow 84%<85%");
    await expect(page.locator("#fix-owner")).toHaveText("Port operations");
    await expect(page.locator("#fix-scope")).toHaveText("retime gate approach routing");
    await expect(page.locator("#dossier-alternatives")).toHaveText("hold gate timing; manual detour only");

    await page.getByRole("button", { name: "Terminal", exact: true }).click();
    await expect(page.locator("#promise-risk")).toHaveText("57");
    await expect(page.locator("#network-flow")).toHaveText("80%");
    await expect(page.getByLabel("Timeline queue")).toContainText("manual");
    await expect(page.getByLabel("Action queue")).toContainText("Reroute advisory");
    await page.getByRole("button", { name: "EV Support" }).click();
    await page.getByRole("button", { name: "Hold For Authority" }).click();
    await expect(page.getByText("Held for authority. Traffic control, legal detour, SLA, EV availability, pricing authority, and revenue guarantees remain blocked claims.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/operator_status,"held"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/manual_events,"Terminal access exception added"/);

    const downloadPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Download CSV" }).click();
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toBe("route-dcr-cockpit-readout.csv");
  });

  test("scheduled timeline events evolve risk and flow deterministically", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await expect(page.locator("#promise-risk")).toHaveText("46");
    await expect(page.locator("#network-flow")).toHaveText("91%");
    await expect(page.locator("#timeline-count")).toHaveText("0 applied");

    await page.getByRole("button", { name: "Step", exact: true }).click();
    await expect(page.locator("#promise-risk")).toHaveText("46");
    await expect(page.locator("#timeline-count")).toHaveText("0 applied");

    await page.getByRole("button", { name: "Step", exact: true }).click();
    await expect(page.locator("#promise-risk")).toHaveText("51");
    await expect(page.locator("#network-flow")).toHaveText("89%");
    await expect(page.locator("#risk-delta")).toHaveText("+5");
    await expect(page.locator("#timeline-count")).toHaveText("1 applied");
    await expect(page.getByLabel("Timeline queue")).toContainText("applied");
    await expect(page.locator("#action-count")).toHaveText("1 open");
    await expect(page.getByLabel("Action queue")).toContainText("Reroute advisory");
    await expect(page.locator("#guidance-status")).toHaveText("held");
    await expect(page.locator("#guidance-route")).toHaveText("draft reroute split");
    await expect(page.locator("#guidance-source")).toHaveText("511 closure feed");
    await expect(page.locator("#workload-open")).toHaveText("1");
    await expect(page.locator("#workload-owner")).toHaveText("Operator");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/timeline_events,"Snow band reduces alternate reliability"/);

    await page.getByRole("button", { name: "Create Reroute advisory" }).click();
    await page.getByLabel("Active role").selectOption("operator");
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.getByText("Held for source custody: 511 closure feed.")).toBeVisible();
    await expect(page.locator("#action-count")).toHaveText("1 open");
    await expect(page.locator("#summary-outcome")).toHaveText("source-held");
    await expect(page.locator("#summary-open-work")).toHaveText("1");
    await expect(page.locator("#sla-status")).toHaveText("held");
    await expect(page.locator("#sla-hold")).toHaveText("0m");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/missing_evidence,"511 closure feed; Charging source owner; Operator message owner"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/run_outcome,"source-held"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/sla_status,"held"/);

    await page.getByRole("button", { name: "Verify 511 closure feed" }).click();
    await expect(page.locator("#gate-count")).toHaveText("1/3 ready");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/evidence_ready,"1\/3"/);
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.locator("#promise-risk")).toHaveText("42");
    await expect(page.locator("#network-flow")).toHaveText("96%");
    await expect(page.locator("#action-count")).toHaveText("0 open");
    await expect(page.locator("#summary-outcome")).toHaveText("mitigated");
    await expect(page.locator("#summary-risk-change")).toHaveText("-4");
    await expect(page.locator("#summary-flow-change")).toHaveText("+5%");
    await expect(page.locator("#summary-evidence-ready")).toHaveText("1/3");
    await expect(page.locator("#summary-pilot-value")).toHaveText("proof-ready");
    await expect(page.locator("#scorecard-grade")).toHaveText("DCR-4");
    await expect(page.locator("#score-response")).toHaveText("0m");
    await expect(page.locator("#score-mitigation")).toHaveText("100%");
    await expect(page.locator("#score-evidence")).toHaveText("33%");
    await expect(page.locator("#score-renewal")).toHaveText("renew");
    await expect(page.locator("#sla-status")).toHaveText("mitigated");
    await expect(page.locator("#sla-verify")).toHaveText("0m");
    await expect(page.locator("#sla-mitigate")).toHaveText("0m");
    await expect(page.locator("#guidance-status")).toHaveText("reviewed");
    await expect(page.locator("#guidance-route")).toHaveText("reviewed alternate route");
    await expect(page.locator("#guidance-source")).toHaveText("operator-reviewed");
    await expect(page.locator("#workload-status")).toHaveText("held");
    await expect(page.locator("#workload-open")).toHaveText("0");
    await expect(page.locator("#workload-source")).toHaveText("2");
    await expect(page.locator("#workload-next")).toHaveText("verify Charging source owner");
    await expect(page.locator("#maintenance-status")).toHaveText("validated");
    await expect(page.locator("#maintenance-thresholds")).toHaveText("none crossed");
    await expect(page.locator("#maintenance-evidence")).toHaveText("Charging source owner; Operator message owner");
    await expect(page.locator("#fix-status")).toHaveText("validated");
    await expect(page.locator("#fix-effect")).toHaveText("-4 risk; +5% flow");
    await expect(page.locator("#fix-approval")).toHaveText("source custody");
    await expect(page.locator("#dossier-status")).toHaveText("validated");
    await expect(page.locator("#dossier-why")).toHaveText("risk 42; flow 96%; recovery 43m");
    await expect(page.locator("#dossier-proof")).toHaveText("Charging source owner; Operator message owner");
    await expect(page.locator("#handoff-status")).toHaveText("mitigated");
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Outcome: mitigated; pilot value: proof-ready/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/SLA view: detect 0m, hold 0m, verify 0m, mitigate 0m/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Guidance: reviewed; route reviewed alternate route/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Workload: held; queue source custody; next owner Charging source owner/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Maintenance: P3 corridor reliability failure; fix weather-responsive split \+ EV staging; thresholds none crossed; evidence gap Charging source owner; Operator message owner/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Fix package: validated; owner DOT operations; scope weather-responsive split \+ EV staging; effect -4 risk; \+5% flow; approval source custody/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Dossier: decision P3 weather-responsive split \+ EV staging; why risk 42; flow 96%; recovery 43m; blockers source custody: verify Charging source owner/);
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Next action: verify Charging source owner/);
    await expect(page.getByLabel("Timeline queue")).toContainText("resolved");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/resolved_events,"1"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/pilot_value,"proof-ready"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/scorecard_grade,"DCR-4"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/time_to_mitigate,"0m"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/guidance_status,"reviewed"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/workload_source_holds,"2"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/maintenance_status,"validated"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/maintenance_fix,"weather-responsive split \+ EV staging"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/maintenance_justification,"risk 42; flow 96%; recovery 43m"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/maintenance_evidence_gap,"Charging source owner; Operator message owner"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/fix_expected_effect,"-4 risk; \+5% flow"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/dossier_status,"validated"/);

    const handoffPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Download TXT" }).click();
    const handoffDownload = await handoffPromise;
    expect(handoffDownload.suggestedFilename()).toBe("route-dcr-shift-handoff.txt");
  });

  test("demo director runs the guided DCR sequence", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await page.getByRole("button", { name: "Next Demo Step" }).click();
    await expect(page.locator("#clock")).toHaveText("00:10");
    await expect(page.locator("#director-status")).toHaveText("Applied event: Snow band reduces alternate reliability.");
    await expect(page.locator("#action-count")).toHaveText("1 open");

    await page.getByRole("button", { name: "Next Demo Step" }).click();
    await expect(page.getByText("Reroute advisory created from simulated signal review.")).toBeVisible();
    await expect(page.locator("#director-status")).toHaveText("Created switch packet: Reroute advisory.");

    await page.getByRole("button", { name: "Next Demo Step" }).click();
    await expect(page.getByText("Held for source custody: 511 closure feed.")).toBeVisible();
    await expect(page.locator("#summary-outcome")).toHaveText("source-held");

    await page.getByRole("button", { name: "Next Demo Step" }).click();
    await expect(page.locator("#gate-count")).toHaveText("1/3 ready");
    await expect(page.locator("#director-status")).toHaveText("Verified source gate: 511 closure feed.");

    await page.getByRole("button", { name: "Next Demo Step" }).click();
    await expect(page.locator("#summary-outcome")).toHaveText("mitigated");
    await expect(page.locator("#action-count")).toHaveText("0 open");
    await expect(page.locator("#scorecard-grade")).toHaveText("DCR-4");
    await expect(page.locator("#sla-status")).toHaveText("mitigated");
    await expect(page.locator("#guidance-status")).toHaveText("reviewed");
    await expect(page.locator("#workload-next")).toHaveText("verify Charging source owner");
    await expect(page.locator("#maintenance-status")).toHaveText("validated");
    await expect(page.getByLabel("Shift handoff")).toHaveValue(/Outcome: mitigated; pilot value: proof-ready/);

    await page.getByRole("button", { name: "Save Run" }).click();
    await expect(page.locator("#portfolio-grade")).toHaveText("proof");
    await expect(page.locator("#portfolio-runs")).toHaveText("1");
    await expect(page.locator("#portfolio-proof")).toHaveText("1");
    await expect(page.locator("#portfolio-mitigated")).toHaveText("1");
    await expect(page.locator("#portfolio-renewal")).toHaveText("1");
    await expect(page.locator("#renewal-backlog-count")).toHaveText("2 items");
    await expect(page.getByLabel("Renewal backlog")).toContainText("Renew proven DCR coverage");
    await expect(page.getByLabel("Renewal backlog")).toContainText("Expand scenario coverage");
  });

  test("express payment signal creates a bounded service advisory", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await page.getByRole("button", { name: "Payment" }).click();
    await expect(page.getByText("Express service payment advisory created from simulated signal review.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/pending_or_held_switches,"Express service payment advisory: pending"/);

    await page.getByRole("button", { name: "Priority advisory $45" }).click();
    await expect(page.locator("#paid-requests")).toHaveText("10");
    await expect(page.locator("#revenue-proxy")).toHaveText("$450");
    await expect(page.getByText("Priority advisory is simulated at $45.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/pricing_status,"simulated only; owner authority required"/);

    await page.getByLabel("Active role").selectOption("payment");
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.locator("#decision-count")).toHaveText("1 reviewed");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/approved_switches,"Express service payment advisory"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/pricing_authority; revenue_guarantee/);

    await page.getByRole("button", { name: "Verified window $120" }).click();
    await expect(page.locator("#paid-requests")).toHaveText("5");
    await expect(page.locator("#revenue-proxy")).toHaveText("$600");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/express_tier,"Verified window"/);
  });

  test("role permissions block mismatched approvals", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await page.getByRole("button", { name: "Payment" }).click();
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.getByText("Held for Planner authority mismatch. Planner can create advisory packets only.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/operator_status,"held"/);

    await page.getByLabel("Active role").selectOption("payment");
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.getByText("Payment owner reviewed advisory. ROUTE still does not command field devices, set prices, or publish claims.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/active_role,"Payment owner"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/role_authority,"can approve express payment posture"/);
  });

  test("local accounts save and replay cockpit runs", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await page.getByLabel("Account profile").selectOption("operator");
    await expect(page.locator("#account-name")).toHaveText("Marta Ruiz");
    await expect(page.getByLabel("Active role")).toHaveValue("operator");
    await expect(page.getByText("Marta Ruiz at District Operations. Local demo account; no authentication backend.")).toBeVisible();

    await page.getByLabel("Scenario case").selectOption("managed-lane");
    await page.getByRole("button", { name: "Step", exact: true }).click();
    await page.getByRole("button", { name: "Step", exact: true }).click();
    await expect(page.getByLabel("Timeline queue")).toContainText("Clearance estimate slips by one cycle");
    await page.getByRole("button", { name: "Save Run" }).click();
    await expect(page.locator("#saved-run-status")).toContainText("Saved Managed-lane incident recovery for Marta Ruiz");
    await expect(page.getByLabel("Run library")).toContainText("Managed-lane incident recovery - Marta Ruiz @ 00:10");

    await page.getByLabel("Account profile").selectOption("planner");
    await page.getByLabel("Scenario case").selectOption("winter-closure");
    await expect(page.locator("#account-name")).toHaveText("Avery Chen");

    await page.getByRole("button", { name: "Load Last" }).click();
    await expect(page.locator("#account-name")).toHaveText("Marta Ruiz");
    await expect(page.locator("#active-case")).toHaveText("Managed-lane incident recovery");
    await expect(page.locator("#clock")).toHaveText("00:10");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/account_name,"Marta Ruiz"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/account_org,"District Operations"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/risk_delta,"7"/);
    await expect(page.getByLabel("Executive readout")).toHaveValue(/timeline_events,"Clearance estimate slips by one cycle"/);
    await expect(page.locator("#risk-delta")).toHaveText("+7");

    await page.getByLabel("Scenario case").selectOption("freight-bottleneck");
    await page.getByRole("button", { name: "Replay Selected" }).click();
    await expect(page.locator("#active-case")).toHaveText("Managed-lane incident recovery");

    const exportPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Export Runs" }).click();
    const exportDownload = await exportPromise;
    expect(exportDownload.suggestedFilename()).toBe("route-dcr-cockpit-runs.json");

    await page.getByRole("button", { name: "Clear Runs" }).click();
    await expect(page.locator("#saved-run-status")).toHaveText("Saved runs cleared.");
    await expect(page.locator("#portfolio-runs")).toHaveText("0");
    await expect(page.locator("#renewal-backlog-count")).toHaveText("0 items");
  });
});

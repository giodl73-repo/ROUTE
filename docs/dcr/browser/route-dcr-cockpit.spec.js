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
    await expect(page.getByLabel("Scenario run plan")).toContainText("Primary pass restriction crosses promise-risk threshold.");
    await expect(page.getByLabel("Evidence gates")).toContainText("511 closure feed");
    await expect(page.locator("#gate-count")).toHaveText("0/3 ready");
    await expect(page.getByLabel("Timeline queue")).toContainText("00:10 Snow band reduces alternate reliability");
    await expect(page.getByLabel("Action queue")).toContainText("No open action");
    await expect(page.locator("#run-owner")).toHaveText("DOT operations");
    await expect(page.getByText("Held traffic control, legal detour, SLA, EV availability, pricing authority")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/held_claims/);
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

    await page.getByRole("button", { name: "Step" }).click();
    await expect(page.locator("#promise-risk")).toHaveText("46");
    await expect(page.locator("#timeline-count")).toHaveText("0 applied");

    await page.getByRole("button", { name: "Step" }).click();
    await expect(page.locator("#promise-risk")).toHaveText("51");
    await expect(page.locator("#network-flow")).toHaveText("89%");
    await expect(page.locator("#risk-delta")).toHaveText("+5");
    await expect(page.locator("#timeline-count")).toHaveText("1 applied");
    await expect(page.getByLabel("Timeline queue")).toContainText("applied");
    await expect(page.locator("#action-count")).toHaveText("1 open");
    await expect(page.getByLabel("Action queue")).toContainText("Reroute advisory");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/timeline_events,"Snow band reduces alternate reliability"/);

    await page.getByRole("button", { name: "Create Reroute advisory" }).click();
    await page.getByLabel("Active role").selectOption("operator");
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.getByText("Held for source custody: 511 closure feed.")).toBeVisible();
    await expect(page.locator("#action-count")).toHaveText("1 open");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/missing_evidence,"511 closure feed; Charging source owner; Operator message owner"/);

    await page.getByRole("button", { name: "Verify 511 closure feed" }).click();
    await expect(page.locator("#gate-count")).toHaveText("1/3 ready");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/evidence_ready,"1\/3"/);
    await page.getByRole("button", { name: "Approve Reviewed Switch" }).click();
    await expect(page.locator("#promise-risk")).toHaveText("42");
    await expect(page.locator("#network-flow")).toHaveText("96%");
    await expect(page.locator("#action-count")).toHaveText("0 open");
    await expect(page.getByLabel("Timeline queue")).toContainText("resolved");
    await expect(page.getByLabel("Executive readout")).toHaveValue(/resolved_events,"1"/);
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
    await page.getByRole("button", { name: "Step" }).click();
    await page.getByRole("button", { name: "Step" }).click();
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
  });
});

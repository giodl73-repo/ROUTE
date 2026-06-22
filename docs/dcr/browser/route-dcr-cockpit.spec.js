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
    await expect(page.locator("#signal-count")).toHaveText("6 active");
    await expect(page.locator("#express-demand")).toHaveText("38%");
    await expect(page.locator("#paid-requests")).toHaveText("0");
    await expect(page.locator("#revenue-proxy")).toHaveText("$0");
    await expect(page.getByLabel("Scenario run plan")).toContainText("Primary pass restriction crosses promise-risk threshold.");
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

    await page.getByRole("button", { name: "Terminal" }).click();
    await page.getByRole("button", { name: "EV Support" }).click();
    await page.getByRole("button", { name: "Hold For Authority" }).click();
    await expect(page.getByText("Held for authority. Traffic control, legal detour, SLA, EV availability, pricing authority, and revenue guarantees remain blocked claims.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/operator_status,"held"/);

    const downloadPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Download CSV" }).click();
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toBe("route-dcr-cockpit-readout.csv");
  });

  test("express payment signal creates a bounded service advisory", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);
    await page.getByRole("button", { name: "Pause" }).click();

    await page.getByRole("button", { name: "Payment" }).click();
    await expect(page.getByText("Express service payment advisory created from simulated signal review.")).toBeVisible();
    await expect(page.getByLabel("Executive readout")).toHaveValue(/pending_or_held_switches,"Express service payment advisory: pending"/);

    await page.getByRole("button", { name: "Priority advisory $45" }).click();
    await expect(page.locator("#paid-requests")).toHaveText("8");
    await expect(page.locator("#revenue-proxy")).toHaveText("$360");
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
});

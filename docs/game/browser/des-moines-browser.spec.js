const { test, expect } = require("@playwright/test");
const path = require("path");

const pagePath = "file:///" + path.resolve(__dirname, "des-moines-diamond.html").replace(/\\/g, "/");

test.describe("Des Moines Diamond browser prototype", () => {
  test("desktop board exposes map, evidence, playback, and publication lock", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);

    await expect(page.getByRole("heading", { name: "Des Moines Diamond", exact: true })).toBeVisible();
    await expect(page.getByLabel("I-35 and I-80 Des Moines transfer topology")).toBeVisible();
    await expect(page.getByText("Publication claim locked")).toBeVisible();
    await expect(page.getByText("I35xI80 recognized; k=0; 3 connectors needed.")).toBeVisible();
    await expect(page.locator("#connector")).toBeVisible();

    await page.getByRole("button", { name: "Before" }).click();
    await expect(page.locator("#incident-value")).toHaveText("86,671");
    await page.getByRole("button", { name: "After" }).click();
    await expect(page.locator("#incident-value")).toHaveText("83,423");
  });

  test("mobile keeps the scenario board visible before panels", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 840 });
    await page.goto(pagePath);

    const board = page.getByLabel("Scenario board");
    await expect(board).toBeVisible();
    await expect(page.getByLabel("I-35 and I-80 Des Moines transfer topology")).toBeVisible();
    await expect(page.getByText("Win band Operational win")).toBeVisible();

    const box = await board.boundingBox();
    expect(box.y).toBeLessThan(220);
  });

  test("local season mutation updates tracks and event log", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 820 });
    await page.goto(pagePath);

    await page.locator('button[data-project="source-request"]').click();
    await page.getByLabel("Event card").selectOption("source-challenge");
    await page.getByRole("button", { name: "Advance Season" }).click();

    await expect(page.locator("#season")).toHaveText("4");
    await expect(page.locator("#budget")).toHaveText("6");
    await expect(page.locator("#evidence")).toHaveText("3");
    await expect(page.getByText("Season 4: Source request completed.")).toBeVisible();
    await expect(page.getByText("Season 4: source challenge; publication remains locked.")).toBeVisible();
    await expect(page.getByLabel("CLI-compatible session log")).toHaveValue(/4,"source-request",0,6,5,4,4,3,1\.000,0\.9,"bounded heuristic"/);
  });
});

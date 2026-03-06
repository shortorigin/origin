const { expect, test } = require("@playwright/test");

const selectedScene = process.env.SHORT_ORIGIN_E2E_SCENE;

const scenes = [
  {
    id: "shell-default",
    assertScene: async ({ page, root }) => {
      await expect(root).toHaveAttribute("data-high-contrast", "false");
      await expect(
        page.getByRole("toolbar", { name: "Desktop taskbar" })
      ).toBeVisible();
      await expect(
        page.getByRole("button", { name: "Open application launcher" })
      ).toBeVisible();
    },
  },
  {
    id: "settings-appearance",
    assertScene: async ({ page }) => {
      await expect(page.getByText("Appearance", { exact: true })).toBeVisible();
      await expect(page.getByText("Soft Neumorphic")).toBeVisible();
    },
  },
  {
    id: "settings-accessibility",
    assertScene: async ({ page }) => {
      await expect(
        page.getByText("Accessibility", { exact: true })
      ).toBeVisible();
      await expect(page.getByText("High contrast")).toBeVisible();
      await expect(page.getByText("Reduced motion")).toBeVisible();
    },
  },
  {
    id: "shell-high-contrast",
    assertScene: async ({ root }) => {
      await expect(root).toHaveAttribute("data-high-contrast", "true");
    },
  },
  {
    id: "terminal-default",
    assertScene: async ({ page }) => {
      await expect(
        page.getByText("Use `help list` to inspect commands.")
      ).toBeVisible();
    },
  },
];

async function openScene(page, id) {
  await page.goto(`/?e2e-scene=${id}`);
  const root = page.locator('[data-ui-kind="desktop-root"]');
  await expect(root).toHaveAttribute("data-e2e-scene", id);
  await expect(root).toHaveAttribute("data-e2e-ready", "true");
  const marks = await page.evaluate(
    () => performance.getEntriesByName("os:e2e-ready").length
  );
  expect(marks).toBeGreaterThan(0);
  return root;
}

for (const scene of scenes) {
  if (selectedScene && selectedScene !== scene.id) {
    continue;
  }

  test(`renders ${scene.id}`, async ({ page }) => {
    const root = await openScene(page, scene.id);
    await scene.assertScene({ page, root });
  });
}

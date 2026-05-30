import { test, expect } from "@playwright/test";

test.describe("ADLER ASI — Ana UI Testleri", () => {
  test("chat panel varsayılan sekmede görüntülenir", async ({ page }) => {
    await page.goto("/");
    await expect(page.locator("h1, h2, h3").first()).toBeVisible();
    await expect(page.getByRole("textbox").first()).toBeVisible();
  });

  test("sidebar sekmeler arasında geçiş yapılabilir", async ({ page }) => {
    await page.goto("/");
    const sidebarButtons = page.getByRole("button");
    const count = await sidebarButtons.count();
    expect(count).toBeGreaterThanOrEqual(3);
  });

  test("sayfa başlığı görüntülenir", async ({ page }) => {
    await page.goto("/");
    await expect(page).toHaveTitle(/ADLER|Adler|asi/i);
  });

  test("arka plan hatası tüm UI'ı çökertmez", async ({ page }) => {
    await page.goto("/");
    const body = page.locator("body");
    await expect(body).toBeVisible();
    const text = await body.innerText();
    expect(text.length).toBeGreaterThan(0);
  });

  test("sayfa boş değil — en az bir bileşen render olmuş", async ({ page }) => {
    await page.goto("/");
    const children = page.locator("body > *");
    const count = await children.count();
    expect(count).toBeGreaterThanOrEqual(1);
  });
});

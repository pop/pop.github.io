import { test, expect } from '@playwright/test';

test('taskbar is visible on load', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('.taskbar')).toBeVisible();
});

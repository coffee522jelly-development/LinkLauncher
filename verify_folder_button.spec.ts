import { test, expect } from '@playwright/test';

test('verify folder button changes', async ({ page }) => {
  await page.goto('http://localhost:1420');

  // Wait for the app to load
  await page.waitForSelector('main');

  // Add a local link to test with
  await page.fill('#name', 'Test Local');
  await page.fill('#path', 'C:\\Windows');
  await page.click('button:has-text("追加")');

  // Screenshot of Table View
  await page.screenshot({ path: 'verification/folder_button_table.png' });

  // Switch to Grid View
  await page.click('button[title="ボタン表示"]');

  // Hover over the card to show buttons
  await page.hover('role=listitem');
  await page.screenshot({ path: 'verification/folder_button_grid.png' });
});

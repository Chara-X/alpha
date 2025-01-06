import { test, expect } from '@playwright/test';

test('test', async ({ page }) => {
  await page.goto('https://www.npmjs.com/package/create-playwright');
  await page.getByPlaceholder('Search packages').click();
  await page.getByText('create-playwright', { exact: true }).click();
});

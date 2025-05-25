import { test, expect } from '@playwright/test';
import { readFileSync } from 'node:fs';

const pobFile = './tests/testdata/pob.txt';

test('can upload build', async ({ page }) => {
  await page.goto('http://localhost:4000/');

  await page.getByText('Path of Exile 1').click();
  await page.getByText('New build').click();

  const pobInput = page.getByLabel('Path of Building data');
  await expect(pobInput).toBeVisible();
  await pobInput.fill(readFileSync(pobFile).toString());
  const proceedBtn = page.getByRole('button', { name: 'Proceed' });
  await expect(proceedBtn).toBeVisible();

  await proceedBtn.click();
  await page.getByRole('button', { name: 'Submit build' }).click();
  await expect(page).toHaveURL(/build/);
});

test('can modify mod configs on preview', async ({ page }) => {
  await page.goto('http://localhost:4000/');
  await page.getByText('Path of Exile 1').click();
  await page.getByText('New build').click();
  const pobInput = page.getByLabel('Path of Building data');
  await pobInput.fill(readFileSync(pobFile).toString());
  const proceedBtn = page.getByRole('button', { name: 'Proceed' });
  await proceedBtn.click();

  const firstModSelect = page.locator('div').filter({ hasText: '+(16-24) to Strength and Dexterity' }).locator('select').first();
  await expect(firstModSelect).toHaveValue('Exist');
  await firstModSelect.selectOption('Exact match');
  await expect(firstModSelect).toHaveValue('Exact');

  await firstModSelect.selectOption('Exist');
  await expect(firstModSelect).toHaveValue('Exist');
  const submitBtn = page.getByRole('button', { name: 'Submit build' });
  await expect(submitBtn).toBeEnabled();
});

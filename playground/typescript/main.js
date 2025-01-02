import { chromium, devices } from 'playwright';
import assert from 'assert';
(async () => {
    // Setup
    const browser = await chromium.launch();
    const context = await browser.newContext(devices['iPhone 11']);
    const page = await context.newPage();
    // The actual interesting bit
    await context.route('**.jpg', route => route.abort());
    await page.goto('https://example.com/');
    assert(await page.title() === 'Example1 Domain'); // 👎 not a Web First assertion
    // Teardown
    await context.close();
    await browser.close();
})();

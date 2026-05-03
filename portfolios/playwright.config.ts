import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: 'tests/e2e',
  reporter: [['list'], ['html', { open: 'never' }]],
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure',
  },
  projects: [
    {
      name: 'pixel-7',
      use: { ...devices['Pixel 7'] },
      // Excludes the headed exploration tests from the default pixel-7 run
      testIgnore: ['**/webamp-drag-headed.spec.ts'],
    },
    {
      name: 'firefox-mobile-headed',
      use: {
        browserName: 'firefox',
        headless: false,
        viewport: { width: 412, height: 915 },
        userAgent:
          'Mozilla/5.0 (Android 13; Mobile; rv:109.0) Gecko/116.0 Firefox/116.0',
        hasTouch: true,
      },
      testIgnore: ['**/webamp-drag-headed.spec.ts'],
    },
    {
      // Exploratory project for project-os-hh0j: CDP + viewport shrink repro
      // attempts. Uses non-headless-shell Chromium (full Chrome devtools
      // available) with Pixel 7 device emulation as base, then overlays CDP
      // Emulation.setDeviceMetricsOverride with mobile:true during the test.
      name: 'pixel-7-cdp',
      use: {
        ...devices['Pixel 7'],
        // Use full Chromium (not headless-shell) so CDP devtools are available
        channel: undefined,
        headless: true,
        launchOptions: {
          // These flags help Chromium behave more like mobile Chrome:
          // MobileLayoutViewport enables the layout/visual viewport split
          args: [
            '--enable-features=MobileLayoutViewport',
            '--touch-events=enabled',
            '--use-mobile-user-agent',
          ],
        },
      },
      testMatch: ['**/webamp-drag-headed.spec.ts'],
    },
  ],
  webServer: {
    command: 'trunk serve --port 8080',
    url: 'http://localhost:8080',
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});

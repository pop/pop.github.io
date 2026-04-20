/**
 * Failing repro for the mobile Webamp drag bug (project-os-0b6n).
 *
 * On Pixel 7 viewport, dragging the Webamp player shifts the .taskbar
 * and/or .clippy-widget rather than keeping them fixed in place. The root
 * cause is that Webamp's drag grows the layout viewport (body scrollWidth
 * exceeds window.innerWidth), which triggers visual-viewport re-resolution
 * and causes position:fixed elements to visually shift.
 *
 * This test MUST FAIL on current main — it is the red light that the
 * fix ticket will turn green.
 *
 * Two mechanisms are tested:
 *   1. Body expansion: document.body.scrollWidth should not exceed
 *      window.innerWidth after dragging Webamp off the right edge.
 *   2. Fixed-element bbox stability: .taskbar and .clippy-widget
 *      getBoundingClientRect() should be unchanged after drag.
 *
 * Selector note: Webamp injects its DOM directly into <body>, not inside
 * #webamp-mount. The correct selectors are bare #main-window and
 * #main-window #title-bar (confirmed via diagnostic run).
 *
 * Touch note: page.touchscreen sends real browser-level touch events that
 * Webamp's own drag handler processes, unlike dispatchEvent() which may not
 * trigger Webamp's document-level listeners reliably.
 */

import { test, expect } from '@playwright/test';

/** Tolerance in pixels: a fixed element should not move at all. */
const TOLERANCE_PX = 1;

type BBox = { x: number; y: number; w: number; h: number };

test.describe('Webamp drag does not move fixed UI (Pixel 7)', () => {
  test.beforeEach(async ({ page }) => {
    // Pre-set localStorage so the "About this Portfolio" Clippy modal does
    // not appear and interfere with clicks / bbox readings.
    await page.addInitScript(() => {
      localStorage.setItem('about_dismissed', '1');
    });
  });

  test('dragging the Webamp title bar only moves the player', async ({ page }) => {
    // ── 1. Load page ──────────────────────────────────────────────────────
    await page.goto('/');
    await expect(page.locator('.taskbar')).toBeVisible();
    await expect(page.locator('.clippy-widget')).toBeVisible();

    // ── 2. Spawn Webamp via the tray icon ─────────────────────────────────
    const trayBtn = page.locator('.taskbar .tray-icon-btn');
    await expect(trayBtn).toBeVisible({ timeout: 5_000 });
    await trayBtn.tap();

    // Webamp injects asynchronously (CDN module + renderWhenReady promise).
    await page.waitForFunction(
      () => {
        const el = document.querySelector('#main-window');
        if (!el) return false;
        const r = el.getBoundingClientRect();
        return r.width > 0 && r.height > 0;
      },
      { timeout: 15_000, polling: 250 },
    );
    // Let Webamp finish its render cycle
    await page.waitForTimeout(500);

    // ── 3. Capture initial state ──────────────────────────────────────────
    const before = await page.evaluate((): {
      taskbar: BBox | null;
      clippy: BBox | null;
      mainWindow: BBox | null;
      titleBar: BBox | null;
      bodyScrollWidth: number;
      innerWidth: number;
    } => {
      const bb = (sel: string): BBox | null => {
        const el = document.querySelector(sel);
        if (!el) return null;
        const r = el.getBoundingClientRect();
        return { x: r.x, y: r.y, w: r.width, h: r.height };
      };
      return {
        taskbar: bb('.taskbar'),
        clippy: bb('.clippy-widget'),
        mainWindow: bb('#main-window'),
        titleBar: bb('#main-window #title-bar'),
        bodyScrollWidth: document.body.scrollWidth,
        innerWidth: window.innerWidth,
      };
    });

    expect(before.taskbar, 'taskbar should be present before drag').not.toBeNull();
    expect(before.clippy, 'clippy-widget should be present before drag').not.toBeNull();
    expect(before.mainWindow, 'Webamp #main-window should be present before drag').not.toBeNull();
    expect(before.titleBar, 'Webamp #title-bar should be present before drag').not.toBeNull();

    // ── 4. Touch-drag the Webamp title bar rightward past the viewport ────
    // Use page.touchscreen for real browser-level touch events that Webamp's
    // drag handler processes, then drag 300px to the right so Webamp's window
    // extends beyond the viewport width — this is what grows the body's
    // scrollable area and triggers the layout viewport re-resolution bug.
    const tb = before.titleBar!;
    const startX = tb.x + tb.w / 2;
    const startY = tb.y + tb.h / 2;
    const endX = startX + 300;
    const endY = startY;

    await page.touchscreen.tap(startX, startY);
    await page.waitForTimeout(50);

    // Simulate a slow drag so Webamp's touchmove handler fires repeatedly
    await page.evaluate(
      ({ fromX, fromY, toX, toY }) => {
        const titleBar = document.querySelector('#main-window #title-bar') as Element;
        if (!titleBar) throw new Error('#main-window #title-bar not found');
        const mainWin = document.querySelector('#main-window') as Element;
        if (!mainWin) throw new Error('#main-window not found');

        // Use the main window as target since Webamp attaches drag to it
        const target = titleBar;

        const touch = (el: Element, type: string, x: number, y: number) => {
          const t = new Touch({
            identifier: 1,
            target: el,
            clientX: x,
            clientY: y,
            pageX: x + window.scrollX,
            pageY: y + window.scrollY,
            screenX: x,
            screenY: y,
          });
          el.dispatchEvent(
            new TouchEvent(type, {
              touches: type === 'touchend' ? [] : [t],
              targetTouches: type === 'touchend' ? [] : [t],
              changedTouches: [t],
              bubbles: true,
              cancelable: true,
            }),
          );
        };

        touch(target, 'touchstart', fromX, fromY);
        const steps = 20;
        for (let i = 1; i <= steps; i++) {
          const x = fromX + ((toX - fromX) * i) / steps;
          const y = fromY + ((toY - fromY) * i) / steps;
          touch(target, 'touchmove', x, y);
        }
        touch(target, 'touchend', toX, toY);
      },
      { fromX: startX, fromY: startY, toX: endX, toY: endY },
    );

    // Give browser time to process layout reflow
    await page.waitForTimeout(600);

    // ── 5. Re-sample state after drag ─────────────────────────────────────
    const after = await page.evaluate((): {
      taskbar: BBox | null;
      clippy: BBox | null;
      bodyScrollWidth: number;
      innerWidth: number;
      scrollX: number;
    } => {
      const bb = (sel: string): BBox | null => {
        const el = document.querySelector(sel);
        if (!el) return null;
        const r = el.getBoundingClientRect();
        return { x: r.x, y: r.y, w: r.width, h: r.height };
      };
      return {
        taskbar: bb('.taskbar'),
        clippy: bb('.clippy-widget'),
        bodyScrollWidth: document.body.scrollWidth,
        innerWidth: window.innerWidth,
        scrollX: window.scrollX,
      };
    });

    // ── 6. Assert: body did not grow beyond viewport width ────────────────
    // The root cause of the bug: Webamp drag grows body.scrollWidth > innerWidth,
    // causing layout viewport re-resolution and position:fixed element shift.
    expect(
      after.bodyScrollWidth,
      `Bug reproduced: Webamp drag expanded body scrollWidth from ` +
        `${before.bodyScrollWidth}px to ${after.bodyScrollWidth}px ` +
        `(viewport: ${after.innerWidth}px). This causes fixed elements to shift.`,
    ).toBeLessThanOrEqual(after.innerWidth);

    // ── 7. Assert fixed elements did NOT move ─────────────────────────────
    const taskbarDy = Math.abs((after.taskbar?.y ?? 0) - (before.taskbar!.y));
    const clippyDy = Math.abs((after.clippy?.y ?? 0) - (before.clippy!.y));
    const taskbarDx = Math.abs((after.taskbar?.x ?? 0) - (before.taskbar!.x));
    const clippyDx = Math.abs((after.clippy?.x ?? 0) - (before.clippy!.x));

    expect(
      taskbarDy,
      `Taskbar shifted vertically by ${taskbarDy.toFixed(1)}px during Webamp drag!\n` +
        `  before: y=${before.taskbar!.y.toFixed(1)}\n` +
        `  after:  y=${(after.taskbar?.y ?? 0).toFixed(1)}\n` +
        `  scrollX after drag: ${after.scrollX}`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);

    expect(
      taskbarDx,
      `Taskbar shifted horizontally by ${taskbarDx.toFixed(1)}px during Webamp drag!\n` +
        `  before: x=${before.taskbar!.x.toFixed(1)}\n` +
        `  after:  x=${(after.taskbar?.x ?? 0).toFixed(1)}\n` +
        `  scrollX after drag: ${after.scrollX}`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);

    expect(
      clippyDy,
      `Clippy shifted vertically by ${clippyDy.toFixed(1)}px during Webamp drag!\n` +
        `  before: y=${before.clippy!.y.toFixed(1)}\n` +
        `  after:  y=${(after.clippy?.y ?? 0).toFixed(1)}\n` +
        `  scrollX after drag: ${after.scrollX}`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);

    expect(
      clippyDx,
      `Clippy shifted horizontally by ${clippyDx.toFixed(1)}px during Webamp drag!\n` +
        `  before: x=${before.clippy!.x.toFixed(1)}\n` +
        `  after:  x=${(after.clippy?.x ?? 0).toFixed(1)}\n` +
        `  scrollX after drag: ${after.scrollX}`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);
  });
});

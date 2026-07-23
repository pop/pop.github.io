/**
 * Exploratory repro attempt for the mobile Webamp drag bug (project-os-hh0j).
 *
 * Two approaches are exercised here:
 *
 * Approach A — CDP Emulation.setDeviceMetricsOverride with mobile: true and an
 * explicit visibleSize that is shorter than the layout size (simulates Chrome's
 * URL bar reducing the visible area). Checks whether visualViewport.offsetTop
 * becomes non-zero (which is the root cause symptom on real devices).
 *
 * Approach B — Mid-test viewport shrink via page.setViewportSize() after drag.
 * Simulates the URL bar retracting by reducing the reported viewport height
 * by ~100px to provoke fixed-element offset shift, then asserts taskbar/Clippy
 * stayed put.
 *
 * Both approaches check:
 *   1. body.scrollWidth did not exceed window.innerWidth after drag (body expansion)
 *   2. .taskbar and .clippy-widget getBoundingClientRect() Y did not change
 *   3. window.visualViewport.offsetTop (if non-zero, URL-bar shift is simulated)
 *
 * This is an exploratory test — the goal is to determine whether any headless/
 * CDP approach can reproduce the visual-viewport divergence, not to be a
 * permanent regression guard.
 */

import { test, expect, type Page, type BrowserContext } from '@playwright/test';

/** Tolerance in pixels for fixed-element positions. */
const TOLERANCE_PX = 2;

type BBox = { x: number; y: number; w: number; h: number };

type State = {
  taskbar: BBox | null;
  clippy: BBox | null;
  mainWindow: BBox | null;
  titleBar: BBox | null;
  bodyScrollWidth: number;
  innerWidth: number;
  innerHeight: number;
  visualViewportOffsetTop: number;
  visualViewportHeight: number;
};

async function captureState(page: Page): Promise<State> {
  return page.evaluate((): State => {
    const bb = (sel: string): BBox | null => {
      const el = document.querySelector(sel);
      if (!el) return null;
      const r = el.getBoundingClientRect();
      return { x: r.x, y: r.y, w: r.width, h: r.height };
    };
    const vv = window.visualViewport;
    return {
      taskbar: bb('.taskbar'),
      clippy: bb('.clippy-widget'),
      mainWindow: bb('#main-window'),
      titleBar: bb('#main-window #title-bar'),
      bodyScrollWidth: document.body.scrollWidth,
      innerWidth: window.innerWidth,
      innerHeight: window.innerHeight,
      visualViewportOffsetTop: vv ? vv.offsetTop : 0,
      visualViewportHeight: vv ? vv.height : window.innerHeight,
    };
  });
}

async function openWebamp(page: Page): Promise<void> {
  const trayBtn = page.locator('.taskbar .tray-icon-btn');
  await expect(trayBtn).toBeVisible({ timeout: 5_000 });
  await trayBtn.tap();

  await page.waitForFunction(
    () => {
      const el = document.querySelector('#main-window');
      if (!el) return false;
      const r = el.getBoundingClientRect();
      return r.width > 0 && r.height > 0;
    },
    { timeout: 15_000, polling: 250 },
  );
  await page.waitForTimeout(500);
}

async function dragWebampRight(page: Page, titleBar: BBox, dragDelta = 300): Promise<void> {
  const startX = titleBar.x + titleBar.w / 2;
  const startY = titleBar.y + titleBar.h / 2;
  const endX = startX + dragDelta;
  const endY = startY;

  await page.evaluate(
    ({ fromX, fromY, toX, toY }) => {
      const target = document.querySelector('#main-window #title-bar') as Element;
      if (!target) throw new Error('#main-window #title-bar not found');

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

  await page.waitForTimeout(600);
}

// ─────────────────────────────────────────────────────────────────────────────
// APPROACH A: CDP Emulation.setDeviceMetricsOverride with mobile:true and
// visibleSize smaller than the layout size (simulates URL bar reducing visible
// area). This is the closest Playwright can get to simulating the URL bar
// shrink without a real device.
// ─────────────────────────────────────────────────────────────────────────────
test.describe('Approach A — CDP mobile emulation with shrunk visibleSize', () => {
  // These tests run in the 'pixel-7-cdp' project (defined in playwright.config.ts).
  // They use a CDP session to set mobile:true and a smaller visibleSize AFTER drag.

  test('CDP setDeviceMetricsOverride: shrink visible size after drag triggers visualViewport shift', async ({
    page,
    context,
  }) => {
    await page.addInitScript(() => {
      localStorage.setItem('about_dismissed', '1');
    });

    await page.goto('/');
    await expect(page.locator('.taskbar')).toBeVisible();
    await expect(page.locator('.clippy-widget')).toBeVisible();

    // Open a CDP session for this page
    const cdp = await (context as BrowserContext).newCDPSession(page);

    // Set mobile emulation with full viewport height (no URL bar yet)
    await cdp.send('Emulation.setDeviceMetricsOverride', {
      width: 412,
      height: 915,
      deviceScaleFactor: 2.625,
      mobile: true,
      screenWidth: 412,
      screenHeight: 915,
    });

    await openWebamp(page);

    const before = await captureState(page);
    expect(before.titleBar, 'Webamp title bar should be present').not.toBeNull();

    // Drag Webamp rightward past the viewport edge
    await dragWebampRight(page, before.titleBar!);

    const afterDrag = await captureState(page);

    // Log body expansion data for diagnostic purposes
    console.log(`[A] After drag — body.scrollWidth: ${afterDrag.bodyScrollWidth}, innerWidth: ${afterDrag.innerWidth}`);
    console.log(`[A] visualViewport.offsetTop after drag: ${afterDrag.visualViewportOffsetTop}`);

    // Now simulate URL bar retraction by reducing the visible viewport height
    // (URL bar appears, taking ~100px from visible area)
    const urlBarHeight = 100;
    await cdp.send('Emulation.setDeviceMetricsOverride', {
      width: 412,
      height: 915,
      deviceScaleFactor: 2.625,
      mobile: true,
      screenWidth: 412,
      screenHeight: 915,
      // visibleSize makes the viewport window smaller than the full screen
      visibleSize: { width: 412, height: 915 - urlBarHeight },
    });

    await page.waitForTimeout(400);

    const afterShrink = await captureState(page);

    console.log(`[A] After CDP shrink — visualViewport.offsetTop: ${afterShrink.visualViewportOffsetTop}`);
    console.log(`[A] taskbar before: y=${before.taskbar?.y?.toFixed(1)}, after shrink: y=${afterShrink.taskbar?.y?.toFixed(1)}`);
    console.log(`[A] clippy before: y=${before.clippy?.y?.toFixed(1)}, after shrink: y=${afterShrink.clippy?.y?.toFixed(1)}`);

    // The "bug reproduced" condition: visualViewport.offsetTop should be 0.
    // On a real device it becomes positive (URL bar height) which shifts fixed elements.
    // If this assertion FAILS, the bug is reproduced.
    const visualViewportShifted = afterShrink.visualViewportOffsetTop > 0;
    console.log(`[A] visualViewport shifted: ${visualViewportShifted} (offsetTop=${afterShrink.visualViewportOffsetTop})`);

    // Assert body did not expand
    expect(
      afterDrag.bodyScrollWidth,
      `[A] body.scrollWidth (${afterDrag.bodyScrollWidth}) should be ≤ innerWidth (${afterDrag.innerWidth}) after drag`,
    ).toBeLessThanOrEqual(afterDrag.innerWidth);

    // Assert fixed elements did not move after viewport shrink
    const taskbarDy = Math.abs((afterShrink.taskbar?.y ?? 0) - (before.taskbar?.y ?? 0));
    const clippyDy = Math.abs((afterShrink.clippy?.y ?? 0) - (before.clippy?.y ?? 0));

    expect(
      taskbarDy,
      `[A] Taskbar shifted ${taskbarDy.toFixed(1)}px after CDP viewport shrink (bug reproduced!)`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);

    expect(
      clippyDy,
      `[A] Clippy shifted ${clippyDy.toFixed(1)}px after CDP viewport shrink (bug reproduced!)`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);
  });
});

// ─────────────────────────────────────────────────────────────────────────────
// APPROACH B: Mid-test page.setViewportSize() shrink to simulate URL bar
// retraction. Simpler — no CDP — but only changes the JS-visible viewport.
// ─────────────────────────────────────────────────────────────────────────────
test.describe('Approach B — mid-test setViewportSize shrink after Webamp drag', () => {
  test('setViewportSize shrink after drag: fixed elements stay put', async ({ page }) => {
    await page.addInitScript(() => {
      localStorage.setItem('about_dismissed', '1');
    });

    await page.goto('/');
    await expect(page.locator('.taskbar')).toBeVisible();
    await expect(page.locator('.clippy-widget')).toBeVisible();

    await openWebamp(page);

    const before = await captureState(page);
    expect(before.titleBar, 'Webamp title bar should be present').not.toBeNull();

    // Drag Webamp rightward
    await dragWebampRight(page, before.titleBar!);

    const afterDrag = await captureState(page);
    console.log(`[B] After drag — body.scrollWidth: ${afterDrag.bodyScrollWidth}, innerWidth: ${afterDrag.innerWidth}`);

    // Simulate URL bar appearing: shrink viewport height by ~100px.
    // On a real device the URL bar consumes ~100px from the bottom of the
    // visual viewport. page.setViewportSize() changes window.innerHeight
    // which is the closest headless analogue.
    const urlBarHeight = 100;
    const currentVp = page.viewportSize()!;
    await page.setViewportSize({ width: currentVp.width, height: currentVp.height - urlBarHeight });
    await page.waitForTimeout(400);

    const afterShrink = await captureState(page);
    console.log(`[B] After viewport shrink — innerHeight: ${afterShrink.innerHeight}`);
    console.log(`[B] visualViewport.offsetTop: ${afterShrink.visualViewportOffsetTop}`);
    console.log(`[B] taskbar before: y=${before.taskbar?.y?.toFixed(1)}, after shrink: y=${afterShrink.taskbar?.y?.toFixed(1)}`);
    console.log(`[B] clippy before: y=${before.clippy?.y?.toFixed(1)}, after shrink: y=${afterShrink.clippy?.y?.toFixed(1)}`);

    // The expected (correct) behavior: taskbar is bottom:0, so its Y should
    // decrease by ~urlBarHeight when the viewport shrinks. That is intentional
    // and correct — it stays anchored to the visible bottom edge.
    // The BUG would be if the element shifted MORE than urlBarHeight,
    // indicating a double-shift from layout-viewport + visual-viewport fighting.
    const taskbarShrinkDy = (before.taskbar?.y ?? 0) - (afterShrink.taskbar?.y ?? 0);
    const clippyShrinkDy = (before.clippy?.y ?? 0) - (afterShrink.clippy?.y ?? 0);
    console.log(`[B] taskbar Y shift during shrink: ${taskbarShrinkDy.toFixed(1)}px (expected ~${urlBarHeight})`);
    console.log(`[B] clippy Y shift during shrink: ${clippyShrinkDy.toFixed(1)}px`);

    // Now expand back (simulates URL bar hide / scroll)
    await page.setViewportSize({ width: currentVp.width, height: currentVp.height });
    await page.waitForTimeout(200);

    const afterExpand = await captureState(page);
    console.log(`[B] After re-expand — innerHeight: ${afterExpand.innerHeight}`);
    console.log(`[B] taskbar after expand: y=${afterExpand.taskbar?.y?.toFixed(1)}`);
    console.log(`[B] clippy after expand: y=${afterExpand.clippy?.y?.toFixed(1)}`);

    // Assert body did not expand beyond viewport
    expect(
      afterDrag.bodyScrollWidth,
      `[B] body.scrollWidth (${afterDrag.bodyScrollWidth}) should be ≤ innerWidth (${afterDrag.innerWidth}) after drag`,
    ).toBeLessThanOrEqual(afterDrag.innerWidth);

    // After re-expanding the viewport, fixed elements should return to their
    // original Y positions. If they don't, there's a residual layout glitch.
    const taskbarRestoreDy = Math.abs((afterExpand.taskbar?.y ?? 0) - (before.taskbar?.y ?? 0));
    const clippyRestoreDy = Math.abs((afterExpand.clippy?.y ?? 0) - (before.clippy?.y ?? 0));

    expect(
      taskbarRestoreDy,
      `[B] Taskbar did not fully restore after viewport shrink+expand: ` +
        `before=${before.taskbar?.y?.toFixed(1)}, after=${afterExpand.taskbar?.y?.toFixed(1)} ` +
        `(delta=${taskbarRestoreDy.toFixed(1)}px)`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);

    expect(
      clippyRestoreDy,
      `[B] Clippy did not fully restore after viewport shrink+expand: ` +
        `before=${before.clippy?.y?.toFixed(1)}, after=${afterExpand.clippy?.y?.toFixed(1)} ` +
        `(delta=${clippyRestoreDy.toFixed(1)}px)`,
    ).toBeLessThanOrEqual(TOLERANCE_PX);
  });
});

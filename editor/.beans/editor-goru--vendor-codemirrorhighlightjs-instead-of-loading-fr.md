---
# editor-goru
title: Vendor CodeMirror/highlight.js instead of loading from CDNs
status: completed
type: task
priority: high
created_at: 2026-09-16T22:33:53Z
updated_at: 2026-09-16T22:42:24Z
---

esm.sh intermittently fails to serve @codemirror/lang-markdown and @replit/codemirror-vim, breaking the editor at runtime. Bundle the JS deps locally with esbuild and serve them from our own origin.

- [x] Add nodejs/esbuild to the flake devShell
- [x] Add js/package.json + lockfile pinning CodeMirror deps
- [x] Move js/codemirror.js to bare-specifier source, bundle to js/vendor/
- [x] Vendor highlight.js + github.min.css
- [x] Update index.html to reference local assets only
- [x] Verify no cross-origin requests at runtime

## Summary of Changes

- `js/package.json` + `package-lock.json` pin the browser deps (@codemirror/state, view, commands, lang-markdown, @replit/codemirror-vim, @highlightjs/cdn-assets).
- `js/codemirror.js` moved to `js/src/codemirror.js`; esm.sh URL imports replaced with bare specifiers.
- `just vendor` runs `npm ci` + esbuild to produce `js/vendor/codemirror.js` (single minified ESM bundle, ~684 KB) and copies `highlight.min.js` / `github.min.css`. The `js/vendor/` output is committed so builds and deploys need no npm registry access.
- `index.html` serves all three assets from our own origin via Trunk copy-file/css.
- `nodejs` and `esbuild` added to the flake devShell.

Verified with headless Chromium against `dist/`: `cmIsReady` true, an editor instance mounts with vim mode on, `hljs` defined, and the only cross-origin requests are api.github.com.

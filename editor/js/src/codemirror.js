// Dependencies are bundled from js/package.json by `just vendor` into
// js/vendor/codemirror.js — nothing here is fetched from a CDN at runtime.
import { EditorState } from '@codemirror/state';
import {
  EditorView,
  keymap,
  drawSelection,
  dropCursor,
  highlightActiveLine,
} from '@codemirror/view';
import {
  defaultKeymap,
  history,
  historyKeymap,
  indentWithTab,
  emacsStyleKeymap,
} from '@codemirror/commands';
import { markdown } from '@codemirror/lang-markdown';
import { vim, Vim } from '@replit/codemirror-vim';

const instances = new Map();
let nextId = 0;

// :w / :write — trigger the toolbar Save button so all existing save logic
// (branch creation, unsaved-changes guard, etc.) runs unchanged.
Vim.defineEx('write', 'w', () => {
  document.querySelector('.save-btn')?.click();
});

// :q / :quit — navigate back (same as the ← Back link).
Vim.defineEx('quit', 'q', () => {
  document.querySelector('.back-link')?.click();
});

// :wq — save, wait for confirmation, then go back.
Vim.defineEx('wq', 'wq', () => {
  const saveBtn = document.querySelector('.save-btn');
  if (!saveBtn || saveBtn.disabled) {
    // Nothing to save — just go back.
    document.querySelector('.back-link')?.click();
    return;
  }
  saveBtn.click();
  // Watch for .save-msg (success) or .error (failure) to appear.
  const observer = new MutationObserver(() => {
    if (document.querySelector('.save-msg')) {
      observer.disconnect();
      document.querySelector('.back-link')?.click();
    } else if (document.querySelector('.error')) {
      observer.disconnect(); // Save failed — stay on page.
    }
  });
  observer.observe(document.body, { childList: true, subtree: true });
});

function buildExtensions(onChange, vimMode) {
  const baseKeys = [...defaultKeymap, ...historyKeymap, indentWithTab];
  // emacsStyleKeymap only when vim is off: vim mode has its own Ctrl-A/E/K bindings
  if (!vimMode) baseKeys.push(...emacsStyleKeymap);

  const exts = [
    history(),
    drawSelection(),
    dropCursor(),
    highlightActiveLine(),
    markdown(),
    EditorView.lineWrapping,
    EditorView.contentAttributes.of({ spellcheck: 'true' }),
    keymap.of(baseKeys),
    EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        onChange(update.state.doc.toString());
      }
    }),
  ];
  if (vimMode) exts.unshift(vim());
  return exts;
}

window.cmCreateEditor = function (mountEl, initialDoc, onChange, vimMode) {
  const state = EditorState.create({
    doc: initialDoc,
    extensions: buildExtensions(onChange, vimMode),
  });
  const view = new EditorView({ state, parent: mountEl });
  const id = String(nextId++);
  instances.set(id, view);
  return id;
};

window.cmGetValue = function (id) {
  const view = instances.get(id);
  return view ? view.state.doc.toString() : '';
};

window.cmSetValue = function (id, value) {
  const view = instances.get(id);
  if (!view) return;
  view.dispatch({
    changes: { from: 0, to: view.state.doc.length, insert: value },
  });
};

window.cmInsertAtCursor = function (id, text) {
  const view = instances.get(id);
  if (!view) return;
  const { from } = view.state.selection.main;
  view.dispatch({
    changes: { from, insert: text },
    selection: { anchor: from + text.length },
  });
  view.focus();
};

window.cmWrapSelection = function (id, prefix, suffix, placeholder) {
  const view = instances.get(id);
  if (!view) return;
  const { from, to } = view.state.selection.main;
  const selected = view.state.sliceDoc(from, to);
  const inner = selected || placeholder;
  view.dispatch({
    changes: { from, to, insert: prefix + inner + suffix },
    selection: {
      anchor: from + prefix.length,
      head: from + prefix.length + inner.length,
    },
  });
  view.focus();
};

window.cmDestroy = function (id) {
  const view = instances.get(id);
  if (view) {
    view.destroy();
    instances.delete(id);
  }
};

window.cmFocus = function (id) {
  const view = instances.get(id);
  if (view) view.focus();
};

// Signal that the module evaluated and the API is ready. The WASM side
// checks this before calling cmCreateEditor, since the module script loads
// asynchronously and may not have run when the component mounts.
window.cmIsReady = true;
window.dispatchEvent(new Event('cm-ready'));

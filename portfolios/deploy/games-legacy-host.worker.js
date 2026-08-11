// Transparent legacy-host serve for games.elijah.run.
//
// The portfolios site is deployed to pages.elijah.run and served under
// /portfolios/. This Worker lets the legacy domain games.elijah.run serve that
// same content WITHOUT a 301 — the browser URL stays games.elijah.run while the
// bytes come from pages.elijah.run/portfolios.
//
// IMPORTANT: pages.elijah.run is itself a Worker ("pages") on the elijah.run
// zone. A same-zone Worker-to-Worker call via global fetch() does NOT invoke
// that Worker (Cloudflare loop-prevention) — it falls through to a non-existent
// origin and returns 522. So we call it through a Service Binding (env.PAGES),
// which dispatches directly to the "pages" Worker. The binding is declared in
// wrangler.toml.
//
// Because the built site uses RELATIVE asset paths, a flat 1:1 path prefix is
// all that's needed:
//   games.elijah.run/            -> pages.elijah.run/portfolios/
//   games.elijah.run/foo.png     -> pages.elijah.run/portfolios/foo.png
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    url.hostname = "pages.elijah.run";
    url.pathname = "/portfolios" + url.pathname; // "/" -> "/portfolios/"
    return env.PAGES.fetch(new Request(url, request));
  },
};

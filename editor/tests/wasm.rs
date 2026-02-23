#![cfg(target_arch = "wasm32")]

//! WASM integration tests
//!
//! Run with: `wasm-pack test --headless --chrome`
//!
//! Prerequisite: `wasm-pack` must be added to `flake.nix` so it is available
//! in the dev shell and can be invoked in CI.
//!
//! ## HTTP mocking evaluation (Phase 11b)
//!
//! Testing `GitHubClient` HTTP methods requires mocking `fetch`. Four options
//! were evaluated:
//!
//! 1. **gloo-net mock facility** — does not exist; gloo-net has no built-in
//!    mock. No action possible here.
//!
//! 2. **Trait-based HTTP abstraction** — refactor `GitHubClient` to accept an
//!    `impl HttpClient` trait rather than calling `gloo_net::http::Request`
//!    directly. Inject a `MockHttpClient` in tests. Enables unit-testing every
//!    response-parsing branch without a running server. Most invasive option
//!    but produces the cleanest tests. Recommended if HTTP tests become a
//!    priority.
//!
//! 3. **Local mock HTTP server** — run a localhost server during
//!    `wasm-pack test --headless --chrome`. `GitHubClient` would need an
//!    overridable base URL. No Rust refactoring, but adds external
//!    test-infrastructure complexity and ties tests to a running process.
//!
//! 4. **Service-worker fetch intercept** — register a service worker that
//!    intercepts all `fetch` calls and returns canned responses. Complex
//!    browser-side setup; not worth it for this project's scale.
//!
//! **Decision:** defer HTTP mocking. The response-parsing, status-code
//! dispatch, and base64 logic are already well-covered by native
//! `#[cfg(test)]` unit tests. The tests below cover the `js_sys::Date`
//! behaviors that genuinely require a browser runtime and underpin the
//! sort-by-date feature.

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// ── js_sys::Date::parse ───────────────────────────────────────────────────
//
// `get_commit_dates_bulk` converts ISO-8601 commit timestamps to epoch-ms via
// `js_sys::Date::parse`, then filters out NaN results. These tests verify the
// expected browser behaviour so regressions surface immediately.

#[wasm_bindgen_test]
fn date_parse_valid_iso_utc() {
    let ms = js_sys::Date::parse("2024-01-15T00:00:00Z");
    assert!(
        !ms.is_nan(),
        "ISO-8601 UTC timestamp should parse to a valid epoch"
    );
    assert!(
        ms > 0.0,
        "Timestamp from 2024 should be a positive epoch value"
    );
}

#[wasm_bindgen_test]
fn date_parse_invalid_string_is_nan() {
    let ms = js_sys::Date::parse("not-a-date");
    assert!(ms.is_nan(), "Non-date string should produce NaN");
}

#[wasm_bindgen_test]
fn date_parse_empty_string_is_nan() {
    // Regression guard: an empty commit-date field must be filtered out by the
    // NaN check in `get_commit_dates_bulk`, not treated as epoch 0.
    let ms = js_sys::Date::parse("");
    assert!(ms.is_nan(), "Empty string should produce NaN");
}

#[wasm_bindgen_test]
fn date_parse_preserves_chronological_ordering() {
    // Sort-by-date compares epoch-ms values directly; later dates must produce
    // larger values.
    let earlier = js_sys::Date::parse("2023-06-01T00:00:00Z");
    let later = js_sys::Date::parse("2024-06-01T00:00:00Z");
    assert!(!earlier.is_nan());
    assert!(!later.is_nan());
    assert!(
        earlier < later,
        "Earlier date should produce a smaller epoch value for correct sort ordering"
    );
}

// ── js_sys::Date component extraction ────────────────────────────────────
//
// `format_date` in dashboard.rs builds "YYYY-MM-DD" strings from epoch-ms
// using `Date::new` + `get_full_year` / `get_month` / `get_date`. Test that
// a known UTC epoch maps to the expected local components.
//
// NOTE: `get_full_year`, `get_month`, and `get_date` return LOCAL time, so
// the assertion uses a noon-UTC timestamp to avoid date-boundary edge cases
// across time zones.

#[wasm_bindgen_test]
fn date_components_from_epoch() {
    use wasm_bindgen::JsValue;
    // 2024-03-15 12:00:00 UTC — noon avoids DST/timezone off-by-one-day issues
    let epoch_ms = js_sys::Date::parse("2024-03-15T12:00:00Z");
    assert!(!epoch_ms.is_nan());

    let date = js_sys::Date::new(&JsValue::from_f64(epoch_ms));
    let year = date.get_full_year();
    let month = date.get_month() + 1; // 0-indexed in JS
    let day = date.get_date();

    // All time zones between UTC-12 and UTC+12 should resolve 12:00 UTC
    // to 2024-03-15 (possibly 14th or 16th at extremes, but realistic TZs
    // stay within one day).
    assert_eq!(year, 2024, "year should be 2024");
    // Allow ±1 day tolerance for edge time zones
    assert!(
        month >= 3 && month <= 4,
        "month should be around March, got {month}"
    );
    assert!(
        day >= 14 && day <= 16,
        "day should be around 15th, got {day}"
    );
}

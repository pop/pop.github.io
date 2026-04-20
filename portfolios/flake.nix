{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  outputs = { self, nixpkgs, utils, rust-overlay }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
      rust = pkgs.rust-bin.stable.latest.default.override {
        targets = [ "wasm32-unknown-unknown" ];
      };
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          # Praise the good language
          rust
          # Wasm
          trunk
          # Wasm
          wasm-bindgen-cli
          # Cloudflare
          wrangler
          # Wasm build
          worker-build
          # Wasm tests
          wasm-pack
          # JSON parsing
          jq
          # Issues tracking for Claude
          beans
          # Like make
          just
          # Image processing
          imagemagick
          # Video processing
          ffmpeg
          # E2e test runner
          bun
          # Playwright-compatible Chromium for NixOS
          playwright-driver.browsers
        ];
        shellHook = ''
          export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}
          export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1
          export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true
        '';
      };
    }
  );
}

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
      # Toolchain is driven by ./rust-toolchain.toml (rustup-style), so `cargo`
      # outside the devshell and the nix devshell resolve to the same toolchain.
      rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
    in
    {
      # One shell for everything needed to maintain the site: build/package the
      # Rust/WASM app, prep assets, run e2e tests, and deploy the rewrite Worker.
      devShells.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          # --- Build the Rust/WASM site (trunk) ---
          rust
          trunk
          wasm-bindgen-cli
          binaryen # wasm-opt (index.html: data-wasm-opt="z")
          sccache  # shared compiler cache (~/.cache/sccache)

          # --- Asset prep ---
          imagemagick
          ffmpeg

          # --- E2e tests ---
          bun
          playwright-driver.browsers

          # --- Deploy the legacy-host rewrite Worker (plain JS) ---
          wrangler

          # --- Misc ---
          jq
          beans
          just
        ];

        shellHook = ''
          # Route rustc through sccache (scoped to the devshell).
          export RUSTC_WRAPPER=sccache

          # Playwright uses the nixpkgs browsers, not npm-downloaded ones.
          export PLAYWRIGHT_BROWSERS_PATH=${pkgs.playwright-driver.browsers}
          export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1
          export PLAYWRIGHT_SKIP_VALIDATE_HOST_REQUIREMENTS=true
        '';
      };
    }
  );
}

{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
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
        BD_NO_DAEMON = "true";
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
          # Infra as Code
          opentofu
          # JSON parsing
          jq
          # Issues tracking for Claude
          beads
        ];
      };
    }
  );
}

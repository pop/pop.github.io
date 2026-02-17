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
        buildInputs = with pkgs; [
          rust
          trunk
          wasm-bindgen-cli
          gnumake
          wrangler
          worker-build
          opentofu
        ];
      };
    }
  );
}

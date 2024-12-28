{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      aspellWithDicts = pkgs.aspellWithDicts (d: [d.en]);
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
            zola
            gnumake
            aspellWithDicts
        ];
      };
    }
  );
}

{
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShells.default = pkgs.rustPlatform.buildRustPackage {
          name = "ti_rand";
          src = self;
          cargoHash = "sha256-LgKJj+M+AeDNFjunEDNStazLKRByt+CFjF7GKiCjqZQ=";
          nativeBuildInputs = with pkgs; [ clippy rustfmt cargo-deny ];
        };
      }
    );
}

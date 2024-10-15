{
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "ti_rand-test";
          src = self;
          cargoHash = "sha256-OBAKEhCSuqBXlPeD9OMlQYuDvImz5V0NBvJiPN3kxYY=";
        };
        devShells.default = packages.default.overrideAttrs (final: old: {
          nativeBuildInputs = old.nativeBuildInputs
            ++ (with pkgs; [ clippy rustfmt ]);
        });
      });
}

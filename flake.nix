{
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShells.default = pkgs.rustPlatform.buildRustPackage {
          name = "ti-rand";
          src = self;
          cargoHash = "sha256-JEdVJiM8+pvXJkq4KiI2vtcA8rBCMi0Zu2dqa0+1Bsk=";
          nativeBuildInputs = with pkgs; [ clippy rustfmt ];
        };
      }
    );
}

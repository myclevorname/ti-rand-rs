{
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShells.default = pkgs.rustPlatform.buildRustPackage {
          name = "ti_rand";
          src = self;
          cargoHash = "sha256-VJNy1AgFFJI37e6otTl9gD0z8rs+6O+o2KyENWcZcqs=";
          nativeBuildInputs = with pkgs; [ clippy rustfmt ];
        };
      }
    );
}

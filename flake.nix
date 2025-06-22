{
  description = "rtest";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: 
    flake-utils.lib.eachDefaultSystem
    (
      system: let 
        pkgs = nixpkgs.legacyPackages.${system};

        rtest = pkgs.rustPlatform.buildRustPackage {
          pname = "rtest";
          version = "0.1";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      in {
        formatter = pkgs.alejandra;
        devShells.default = import ./shell.nix {inherit pkgs;};
        packages.default = rtest;

        apps.default = flake-utils.lib.mkApp {
          drv = rtest;
        }
      }
    );
}
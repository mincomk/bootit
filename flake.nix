{
  description = "bootit: a lightweight next-boot selector";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

      version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
    in
    {
      packages = forAllSystems (pkgs: rec {
        bootit = pkgs.rustPlatform.buildRustPackage {
          pname = "bootit";
          inherit version;
          src = ./.;

          cargoLock.lockFile = ./Cargo.lock;

          # `it` is a second bin in the same crate; buildRustPackage installs
          # every bin, so both `bootit` and `it` land in $out/bin.
          meta = {
            description = "Lightweight next-boot selector";
            homepage = "https://github.com/mincomk/bootit";
            license = pkgs.lib.licenses.mit;
            mainProgram = "bootit";
            platforms = pkgs.lib.platforms.linux;
          };
        };

        default = bootit;
      });

      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.bootit}/bin/bootit";
          meta.description = "Run bootit without installing it";
        };
      });

      checks = forAllSystems (pkgs: {
        bootit = self.packages.${pkgs.stdenv.hostPlatform.system}.bootit;
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
            pkgs.rust-analyzer
          ];
        };
      });
    };
}

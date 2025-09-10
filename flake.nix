{
  description = "A fast, minimalist directory tree viewer, written in Rust.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    naersk,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {inherit system;};
        naersk' = pkgs.callPackage naersk {};

        manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        drv = naersk'.buildPackage {
          pname = manifest.package.name;
          version = manifest.package.version;
          src = ./.;

          doCheck = true;

          # MUST be a function (list -> list)
          cargoTestOptions = opts:
            opts
            ++ [
              "--"
              "--skip"
              "test_git_status_flag"
              "--skip"
              "test_gitignore_flag"
            ];

          nativeBuildInputs = [pkgs.pkg-config];
        };
      in {
        packages.default = drv;

        apps.default = {
          type = "app";
          program = "${drv}/bin/${manifest.package.name}";
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.rustc
            pkgs.cargo
            pkgs.clippy
            pkgs.rustfmt
            pkgs.rust-analyzer
            pkgs.pkg-config
          ];
        };

        checks = {
          tests = drv;

          clippy = pkgs.runCommand "clippy" {} ''
            cd ${self}
            cargo clippy --all-targets -- -D warnings
            touch $out
          '';

          fmt = pkgs.runCommand "fmt" {} ''
            cd ${self}
            cargo fmt --all -- --check
            touch $out
          '';
        };
      }
    );
}

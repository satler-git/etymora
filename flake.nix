{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    git-hooks-nix = {
      url = "github:cachix/git-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix.url = "github:numtide/treefmt-nix";

    crane.url = "github:ipetkov/crane";
  };

  outputs =
    inputs@{
      self,
      flake-parts,
      crane,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      flake = { };

      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.git-hooks-nix.flakeModule
      ];

      systems = [
        "aarch64-linux"
        "x86_64-linux"
      ];

      perSystem =
        {
          config,
          system,
          pkgs,
          lib,
          self',
          ...
        }:
        let
          rust-bin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain rust-bin;

          commonArgs = {
            src = craneLib.cleanCargoSource ./.;
            strictDeps = true;

            buildInputs = with pkgs; [ ];

            nativeBuildInputs = with pkgs; [ ];
          };

          cargoArtifacts = craneLib.buildDepsOnly (
            commonArgs
            // {
              pname = "etymora-deps";
            }
          );
        in
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              inputs.rust-overlay.overlays.default
            ];
          };

          treefmt = {
            projectRootFile = "flake.nix";

            programs.actionlint.enable = true;
            programs.nixfmt.enable = true;
            programs.rustfmt.enable = true;
            programs.stylua.enable = true;
            programs.taplo.enable = true;
            programs.yamlfmt.enable = true;
          };

          pre-commit = {
            settings.hooks = {
              flake-treefmt = {
                enable = true;
                name = "flake-treefmt";
                entry = lib.getExe config.treefmt.build.wrapper;
                pass_filenames = false;
              };

              # clippy.enable = true;
              # cargo-check.enable = true;
            };
          };

          packages.default = self'.packages.etymora;

          packages.etymora = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              pname = "etymora";
              version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version;
              cargoExtraArgs = "-p etymora";
            }
          );

          devShells.default = pkgs.mkShell {
            inputsFrom = [ config.pre-commit.devShell ];

            buildInputs = with pkgs; [
              cargo-expand
              cargo-nextest

              rust-bin
            ];
          };
        };
    };
}

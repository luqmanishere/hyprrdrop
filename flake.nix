{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    nci.url = "github:yusdacra/nix-cargo-integration";
    nci.inputs.nixpkgs.follows = "nixpkgs";
    parts.url = "github:hercules-ci/flake-parts";
    parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs @ {
    parts,
    nci,
    devshell,
    rust-overlay,
    nixpkgs,
    ...
  }:
    parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      imports = [nci.flakeModule parts.flakeModules.easyOverlay devshell.flakeModule];
      perSystem = {
        config,
        pkgs,
        system,
        lib,
        ...
      }: let
        crateName = "hyprrdrop";
        # shorthand for accessing this crate's outputs
        # you can access crate outputs under `config.nci.outputs.<crate name>` (see documentation)
        crateOutputs = config.nci.outputs.${crateName};
      in {
        # use oxalica/rust-overlay
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };

        nci = {
          # relPath is empty to denote current dir
          projects.${crateName}.path = ./.;

          crates.${crateName} = {
            # export crate (packages and devshell) in flake outputs
            export = true;
          };

          toolchains = {
            build =
              pkgs.rust-bin.stable.latest.minimal;
          };
        };

        # use numtide/devshell
        devshells.default = with pkgs; {
          motd = ''
            -----------------
            -hyprrdrop devshell-
            -----------------
            $(type -p menu &>/dev/null && menu)
          '';

          packages = [
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-src"];
            })
            rust-analyzer
          ];

          packagesFrom = [crateOutputs.packages.release];

          commands = [
            {
              name = "nix-run-${crateName}";
              command = "RUST_LOG=debug nix run .#${crateName}-dev";
              help = "Run ${crateName} (debug build)";
              category = "Run";
            }
            {
              name = "nix-run-${crateName}-rel";
              command = "RUST_LOG=debug nix run .#${crateName}-rel";
              help = "Run ${crateName} (release build)";
              category = "Run";
            }
            {
              name = "nix-build-${crateName}";
              command = "RUST_LOG=debug nix build .#${crateName}-dev";
              help = "Build ${crateName} (debug build)";
              category = "Build";
            }
            {
              name = "nix-build-${crateName}-rel";
              command = "RUST_LOG=debug nix build .#${crateName}-rel";
              help = "Build ${crateName} (release build)";
              category = "Build";
            }
            {
              name = "tail-log";
              command = "tail -f /tmp/hyprrdrop/hyprrdrop-log";
              help = "tail the ${crateName} logfile";
              category = "Utilities";
            }
          ];
        };

        # export the release package of the crate as default package
        packages.default = crateOutputs.packages.release;

        # export overlay using easyOverlays
        overlayAttrs = {
          inherit (config.packages) hyprrdrop;
          /*
          inherit (inputs.rust-overlay.overlays) default;
          */
        };
        packages.hyprrdrop = crateOutputs.packages.release;
        packages.hyprrdrop-dev = crateOutputs.packages.dev;
      };
      flake = {
        homeManagerModules = {
          # TODO: hm module
          # hyprrdrop = import ./nix/hm-module.nix inputs.self;
          # default = inputs.self.homeManagerModules.hyprrdrop;
        };
      };
    };
}

{
  inputs = {
    devenv.url = "github:cachix/devenv";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/x86_64-linux";

    devenv-root = {
      url = "file+file:///dev/null";
      flake = false;
    };
  };

  outputs =
    inputs@{ flake-parts, devenv-root, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = import inputs.systems;

      perSystem =
        { pkgs, ... }:
        {
          devenv.shells.default = {
            devenv.root =
              let
                devenvRootFileContent = builtins.readFile devenv-root.outPath;
              in
              pkgs.lib.mkIf (devenvRootFileContent != "") devenvRootFileContent;

            languages = {
              nix.enable = true;
              rust.enable = true;
            };

            pre-commit.hooks = {
              clippy.enable = true;
              commitlint-rs = {
                enable = true;
                entry = "commitlint --edit";
                language = "rust";
                package = pkgs.commitlint-rs;
                pass_filenames = false;
                stages = [ "commit-msg" ];
              };
              deadnix.enable = true;
              rustfmt.enable = true;
              statix.enable = true;
              nixfmt-rfc-style.enable = true;
            };
          };

          formatter = pkgs.nixfmt-rfc-style;
        };
    };
}

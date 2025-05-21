{
  description = "alladin";
  inputs = {
    nixpkgs.url =
      "github:nixos/nixpkgs/2f9173bde1d3fbf1ad26ff6d52f952f9e9da52ea";
    go_1_24_0-pkgs.url =
      "github:nixos/nixpkgs/2d068ae5c6516b2d04562de50a58c682540de9bf";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ ];
        pkgs = import nixpkgs { inherit system overlays; };
        go-pkgs = inputs.go_1_24_0-pkgs.legacyPackages.${system};

        ctx = {
          package = {
            name = "alladin";
            version = "0.0.1";
            src = ./.;
            go-mod = ./go.mod;
          };
          go = go-pkgs.go_1_24;
          build-deps = [ pkgs.ffmpeg ];
        };

        package = import ./nix/package.nix { inherit pkgs ctx; };

        devShell = import ./nix/shell.nix { inherit pkgs ctx; };
      in {
        formatter = pkgs.nixfmt-classic;
        devShells.default = devShell;
        packages.default = package;
        apps.default = {
          type = "app";
          program = "${package}/bin/${ctx.package.name}";
        };
      });
}

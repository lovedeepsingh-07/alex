{
  description = "alex";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/d2ed99647a4b195f0bcc440f76edfa10aeb3b743";
    rust_overlay = {
      url = "github:oxalica/rust-overlay/a9c35d6e7cb70c5719170b6c2d3bb589c5e048af";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
  };
  outputs = {...} @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [(import inputs.rust_overlay)];
      };
      rust_pkg = pkgs.rust-bin.stable."1.88.0".default;
      shell = import ./_nix/shell.nix {inherit pkgs rust_pkg;};
      alex = import ./_nix/alex.nix {inherit pkgs rust_pkg;};
    in {
      devShells.linux = shell.linux;
      packages.linux = alex.linux;
    });
}

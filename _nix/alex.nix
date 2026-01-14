{
  pkgs,
  rust_pkg,
  ...
}: let
  rust-platform = pkgs.makeRustPlatform {
    cargo = rust_pkg;
    rustc = rust_pkg;
  };
in {
  linux = rust-platform.buildRustPackage {
    pname = "alex";
    version = "0.1.0";
    src = ../.;
    buildInputs = [pkgs.alsa-lib];
    nativeBuildInputs = [pkgs.pkg-config];
    cargoLock.lockFile = ../Cargo.lock;
  };
}

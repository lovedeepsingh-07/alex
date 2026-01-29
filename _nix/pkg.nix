{
  pkgs,
  rust_pkg,
  ...
}: let
  rust-platform = pkgs.makeRustPlatform {
    cargo = rust_pkg;
    rustc = rust_pkg;
  };
  package_info = (builtins.fromTOML (builtins.readFile ../Cargo.toml)).package;
in rec {
  default = rust-platform.buildRustPackage {
    pname = package_info.name;
    version = package_info.version;
    src = ../.;
    buildInputs = [pkgs.alsa-lib];
    nativeBuildInputs = [pkgs.pkg-config];
    cargoLock.lockFile = ../Cargo.lock;
  };
  release = pkgs.stdenv.mkDerivation {
    name = package_info.name;
    dontUnpack = true;
    nativeBuildInputs = [pkgs.zip];
    installPhase = ''
      mkdir -p $out/stage
      cp -r ${default}/bin/* $out/stage/
      cd $out/stage/
      zip -r $out/${package_info.name}-${package_info.version}-linux-x86_64.zip .
    '';
  };
}

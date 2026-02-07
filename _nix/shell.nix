{
  pkgs,
  rust_pkg,
  ...
}: {
  default = pkgs.mkShell {
    nativeBuildInputs = [pkgs.alejandra rust_pkg pkgs.pkg-config pkgs.flatbuffers];
    buildInputs = [pkgs.alsa-lib];
    shellHook = "zsh";
  };
}

{
  pkgs,
  rust_pkg,
  ...
}: {
  default = pkgs.mkShell {
    nativeBuildInputs = [pkgs.alejandra rust_pkg pkgs.pkg-config];
    buildInputs = [pkgs.alsa-lib];
    shellHook = "zsh";
  };
}

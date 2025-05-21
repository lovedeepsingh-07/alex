{ pkgs, ctx }:

(pkgs.buildGoModule.override { go = ctx.go; }) {
  pname = ctx.package.name;
  version = ctx.package.name;
  src = ctx.package.src;
  goMod = ctx.package.go-mod;

  subPackages = [ "bin/${ctx.package.name}" ];
  vendorHash = null;
  proxyVendor = true;

  nativeBuildInputs = [ pkgs.makeWrapper ];
  buildInputs = ctx.build-deps;

  postInstall = ''
    wrapProgram $out/bin/${ctx.package.name} \
      --prefix PATH : ${pkgs.lib.makeBinPath ctx.build-deps}
  '';
}

{ pkgs, ctx }:

pkgs.mkShell { packages = [ pkgs.just ctx.go ] ++ ctx.build-deps; }

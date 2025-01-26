{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell rec {
          buildInputs = [ 
            cargo 
            rustc 
            rustfmt 
            pre-commit 
            rustPackages.clippy 
            pkg-config 
            libudev-zero
            alsa-lib.dev
            xorg.libXi
            xorg.libX11
            xorg.libXcursor
            xorg.libXrandr
            libGL.dev
            libxkbcommon
            mesa.dev
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = builtins.concatStringsSep ":" [
            "${xorg.libX11}/lib"
            "${xorg.libXi}/lib"
            "${libGL}/lib"
            "${libxkbcommon}/lib"
          ];
        };
      });
}

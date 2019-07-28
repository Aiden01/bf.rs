
let
  pkgs = import <nixpkgs> {};
  stdenv = pkgs.stdenv;
  executable = pkgs.rustPlatform.buildRustPackage rec {
    name = "bf";
    version = "0.0.1";
    src = ./.;
    cargoSha256 = "1rhqszic2japdggxgdysg19xl19wq9dx0jn8bsbbsmz194pb5i5l";
  };
  bf = pkgs.runCommand "bf" {} ''
     mkdir -p $out/bin
     cp ${executable}/bin/bfrs $out/bin/bf
     '';
in bf

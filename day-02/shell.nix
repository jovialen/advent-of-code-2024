{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    ghc
    cabal-install
    haskell-language-server
  ];
}

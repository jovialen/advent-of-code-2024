 { pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    libgcc
    clang-tools
    lldb
    gdb
    valgrind
  ];
}

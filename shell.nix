let
  pkgs = import <nixpkgs> {};
in
  pkgs.pkgsCross.riscv64.mkShell {
    packages = with pkgs; [alejandra cargo-binutils clang-tools fd gcc gdb qemu];
  }

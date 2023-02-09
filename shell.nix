let
  pkgs = import <nixpkgs> {};
in
  pkgs.pkgsCross.riscv64.mkShell {
    packages = with pkgs; [alejandra clang-tools fd gcc];
  }

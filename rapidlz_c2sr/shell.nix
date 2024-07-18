let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.05";
  pkgs = import nixpkgs { config = {}; overlays = []; };
in
pkgs.mkShell {
  packages = [
    pkgs.git
    pkgs.cacert
    pkgs.cargo
    pkgs.rustc
    pkgs.gcc11
    pkgs.cmake
    pkgs.conan
    pkgs.ninja
  ];
}
{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      pkgs = nixpkgs.${system}.legacyPackages;
      system = "x86_64-linux";
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "pgi";
        nativeBuildInputs = with pkgs; [
          typst
          typst-fmt
          typst-lsp

          clippy
          rustfmt
          rust-analyzer
        ];
        buildInputs = with pkgs; [
          pkg-config
          alsa-lib

          cargo
          rustc
        ];
      };
    };
}

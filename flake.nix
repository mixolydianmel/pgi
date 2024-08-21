{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        name = "pgi";
        nativeBuildInputs = with pkgs; [
          typst
          typst-fmt
          typst-lsp

          bacon
          clippy
          rustfmt
          rust-analyzer
        ];
        buildInputs = with pkgs; [
          pkg-config
          alsa-lib
          wayland
          udev
          libxkbcommon
          vulkan-loader
          # opencv
          libclang
          # llvmPackages.libcxx
          # libv4l
          linuxHeaders
          glibc

          cargo
          rustc
        ];

        LD_LIBRARY_PATH =
          with pkgs;
          lib.makeLibraryPath [
            libxkbcommon
            vulkan-loader
            libclang
            # llvmPackages.libcxx
            # libv4l
            # linuxHeaders
          ];
        CPATH =
          with pkgs;
          lib.makeIncludePath [
            # libv4l
            linuxHeaders
            glibc
          ];
      };
    };
}

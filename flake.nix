{
  description = "Swiss - a universal file conversion CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.default =
        pkgs.rustPlatform.buildRustPackage {
          pname = "swiss";
          version = "0.1.0";

          src = pkgs.lib.cleanSource ./.;

          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = [
            pkgs.makeWrapper
          ];

          postInstall = ''
            wrapProgram $out/bin/swiss \
              --prefix PATH : ${
                pkgs.lib.makeBinPath [
                  pkgs.imagemagick
                  pkgs.ffmpeg
                  pkgs.pandoc
                  pkgs.libreoffice
                ]
              }
          '';
        };

      devShells.${system}.default =
        pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer

            imagemagick
            ffmpeg
            pandoc
            libreoffice
          ];

          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
    };
}

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/23.05";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay"; # use master since newer Rust versions are continuously pushed
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    rust-overlay,
  }:
    utils.lib.eachDefaultSystem (system: let
      overlays = [(import rust-overlay)];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      defaultApp = utils.lib.mkApp {
        drv = self.defaultPackage."${system}";
      };

      # More info here: https://github.com/bevyengine/bevy/issues/9203#issuecomment-1657248743
      devShell = with pkgs;
        mkShell rec {
          buildInputs = [
            rust-bin.stable.latest.default
            rust-analyzer
            pkg-config
            alsa-lib
            udev

            vulkan-loader

            libxkbcommon
            wayland # To use the wayland feature

            # xorg.libX11
            # xorg.libXcursor
            # xorg.libXi
            # xorg.libXrandr # To use the x11 feature
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
          PKG_CONFIG_ALLOW_SYSTEM_CFLAGS = "1";
        };

      formatter = nixpkgs.legacyPackages.x86_64-linux.alejandra;
    });
}

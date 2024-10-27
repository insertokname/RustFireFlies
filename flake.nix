{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
        appRuntimeInputs = with pkgs; [
          udev
          libxkbcommon
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          alsaLib
          vulkan-headers
          vulkan-validation-layers

        ];
        buildInputs = with pkgs; [
          sccache
          pkg-config
          clang
          mold
          makeWrapper
          lld
          vulkan-tools
        ];
        devInputs = with pkgs; [
	        rustup
          cargo
          rustc
          rustfmt
          pre-commit
          rustPackages.clippy
        ];
      in {
        defaultPackage = naersk-lib.buildPackage {
          name = "bevy_test";
          src = ./.;
          buildInputs = buildInputs ++ appRuntimeInputs;
          doCheck = true;
        };
        devShell = with pkgs;
          mkShell {
            shellHook = ''
              export LD_LIBRARY_PATH="${
                pkgs.lib.makeLibraryPath appRuntimeInputs
              }"'';
            buildInputs = buildInputs ++ appRuntimeInputs ++ devInputs;
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
      });
}

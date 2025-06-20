{
  description = "Nix Flake for Dioxus development";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixgl.url = "github:nix-community/nixGL";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = inputs: with inputs; let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
      overlays = [
        rust-overlay.overlays.default
        nixgl.overlay
      ];
    };

    rust = pkgs.rust-bin.fromRustupToolchainFile ./Toolchain.toml;

    builder =
    { lib
    , glib
    , gtk3
    , xdotool
    , webkitgtk_4_1
    , pkg-config
    , rustPlatform }: let
      toml = (lib.importTOML ./Cargo.toml).package;
    in rustPlatform.buildRustPackage {
      inherit (toml) version;

      pname = toml.name;
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;

      buildInputs = [
        glib
        gtk3
        xdotool
        webkitgtk_4_1
      ];

      nativeBuildInputs = [ pkg-config ];

      meta.mainProgram = "brewery";
    };
  in with pkgs; {
    packages.${system} = {
      brewery = callPackage builder { };
      default = self.packages.${system}.brewery;
    };

    devShells.${system}.default = mkShellNoCC {
      packages = [
        rust
        rust-analyzer-unwrapped
        rust-bin.nightly."2024-04-07".rustfmt
        dioxus-cli
        nixgl.nixGLMesa
      ];

      buildInputs = [
        gtk3
        xdotool
        libsoup_3
      ];

      env = {
        RUST_SRC_PATH="${rust}/lib/rustlib/src/rust/library";
        PKG_CONFIG_PATH="${webkitgtk_4_1.dev}/lib/pkgconfig";
        WEBKIT_DISABLE_DMABUF_RENDERER = 1;
        LOCALE_ARCHIVE = if system == "x86_64-linux"
          then "${glibcLocales}/lib/locale/locale-archive" else "";
      };
    };
  };
}

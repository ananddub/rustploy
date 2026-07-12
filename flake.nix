{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self, ... }@inputs:

    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            inherit system;
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
            ]
          );
      };

      devShells = forEachSupportedSystem (
        { pkgs, system }:
        let
          optionalPackage = name: pkgs.lib.optional (builtins.hasAttr name pkgs) (builtins.getAttr name pkgs);
          optionalPackages = names: pkgs.lib.concatLists (map optionalPackage names);
          buildpackTools = optionalPackages [
            "nixpacks"
            "pack"
            "pack-cli"
            "heroku"
            "heroku-cli"
            "railpack"
          ];
        in
        {
          default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                rustToolchain
                openssl
                pkg-config
                atlas
                cargo-deny
                cargo-edit
                cargo-watch
                sqlx-cli
                rust-analyzer
                mold
                git
                curl
                gnutar
                gzip
                docker-client
                nodejs_22
                self.formatter.${system}
              ]
              ++ buildpackTools;

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";
            };

            shellHook = ''
              export RUSTPLOY_TOOL_HOME="''${RUSTPLOY_TOOL_HOME:-$HOME/.cache/rustploy-tools}"
              mkdir -p "$RUSTPLOY_TOOL_HOME/bin"
              export PATH="$RUSTPLOY_TOOL_HOME/bin:$PATH:$HOME/.cargo/bin"

              rustploy_install_pack() {
                local version="''${PACK_VERSION:-0.39.1}"
                local machine
                machine="$(uname -m)"
                case "$machine" in
                  x86_64) machine="x86-64" ;;
                  aarch64|arm64) machine="arm64" ;;
                esac
                local tmp
                tmp="$(mktemp -d)"
                curl -fsSL "https://github.com/buildpacks/pack/releases/download/v$version/pack-v$version-linux-$machine.tgz" \
                  | tar -C "$tmp" -xz pack
                install -m 0755 "$tmp/pack" "$RUSTPLOY_TOOL_HOME/bin/pack"
                rm -rf "$tmp"
              }

              rustploy_install_nixpacks() {
                curl -fsSL https://nixpacks.com/install.sh | NIXPACKS_VERSION="''${NIXPACKS_VERSION:-1.41.0}" bash
              }

              rustploy_install_railpack() {
                curl -fsSL https://railpack.com/install.sh | RAILPACK_VERSION="''${RAILPACK_VERSION:-0.15.4}" bash
              }

              rustploy_install_heroku() {
                npm_config_prefix="$RUSTPLOY_TOOL_HOME/npm" npm install -g heroku
                ln -sf "$RUSTPLOY_TOOL_HOME/npm/bin/heroku" "$RUSTPLOY_TOOL_HOME/bin/heroku"
              }

              rustploy_ensure_build_tools() {
                command -v pack >/dev/null 2>&1 || rustploy_install_pack
                command -v nixpacks >/dev/null 2>&1 || rustploy_install_nixpacks
                command -v railpack >/dev/null 2>&1 || rustploy_install_railpack
                command -v heroku >/dev/null 2>&1 || rustploy_install_heroku
              }

              echo "Rustploy dev shell ready."
              echo "Build tools: docker, pack/Paketo, nixpacks, railpack, heroku."
              echo "If any optional tool is missing, run: rustploy_ensure_build_tools"
            '';
          };
        }
      );

      formatter = forEachSupportedSystem ({ pkgs, ... }: pkgs.nixfmt);
    };
}

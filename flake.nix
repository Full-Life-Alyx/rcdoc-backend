{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = [
          pkgs.cargo
          pkgs.rustc

          pkgs.rust-analyzer
          pkgs.rustfmt

          # If the dependencies need system libs, you usually need pkg-config + the lib
          pkgs.pkg-config
          pkgs.openssl

          pkgs.sqlx-cli
        ];

        env = { RUST_BACKTRACE = "full"; };
      };
    };
}


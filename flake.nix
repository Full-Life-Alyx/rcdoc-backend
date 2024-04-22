{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          rustc

          rust-analyzer
          rustfmt

          # If the dependencies need system libs, you usually need pkg-config + the lib
          pkg-config
          openssl

          sqlx-cli
        ];

        env = { RUST_BACKTRACE = "full"; };
      };
    };
}


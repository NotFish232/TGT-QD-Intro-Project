{
  description = "Rust Flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs, ... }@inputs:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          pkgs.pkg-config
          pkgs.openssl
        ];

        packages = with pkgs; [
          rustc
          cargo
          clippy
          rust-analyzer
          rustfmt
        ];
      };
    };
}

{
  description = "Markdownizer";

  inputs.nixpkgs.url = github:NixOS/nixpkgs/nixos-20.09;

  outputs = { self, nixpkgs }:
  let
    systems = [ "x86_64-linux" "i686-linux" "aarch64-linux" ];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system); 
    # Memoize nixpkgs for different platforms for efficiency.
    nixpkgsFor = forAllSystems (system:
      import nixpkgs {
        inherit system;
        overlays = [ self.overlay ];
      }
    );
  in {
    overlay = final: prev: {

      markdownizer = with final; ( rustPlatform.buildRustPackage rec {
          name = "markdownizer";
          version = "0.1.0";
          src = ./.;

          cargoSha256 = "sha256-K1faj/H8mb4cdOFnvlm1ZUGkoRnVvKLWF7SUNLOGEYY=";

          # nativeBuildInputs = [ pkgconfig ];
          # buildInputs = [ openssl ];

          meta = with pkgs.stdenv.lib; {
            description = "A project planning system based on markdown files";
            homepage = "https://github.com/mmai/markdownizer";
            license = licenses.gpl3;
            platforms = platforms.unix;
            maintainers = with maintainers; [ mmai ];
          };
        });
    };

    packages = forAllSystems (system: {
      inherit (nixpkgsFor.${system}) markdownizer;
    });

    defaultPackage = forAllSystems (system: self.packages.${system}.markdownizer);

    devShell = forAllSystems (system: (import ./shell.nix { pkgs = nixpkgs.legacyPackages.${system}; }));

  };
}

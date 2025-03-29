{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    nodejs
    fish
    # nodePackages.npm
  ];

  shellHook = ''
    export PATH="$PWD/node_modules/.bin:$PATH"

    exec /bin/fish
  '';
}

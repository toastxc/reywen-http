let
  pkgs = import <nixpkgs> { };

  libraries = with pkgs; [
    pkg-config
    openssl_3
  ];

  packages = with pkgs; [
    pkg-config
    openssl_3
  ];
in
pkgs.mkShell {
  buildInputs = packages;

  shellHook =
    ''
      export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    '';
}

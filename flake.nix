{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };
  };

  outputs = { self, nixpkgs }: {
    devShell = {
      x86_64-linux =
        let
          pkgs = import nixpkgs {
            system = "x86_64-linux";
          };

        in
        pkgs.mkShell {
          buildInputs = with pkgs; [
            (tic-80.override {
              # We've got the pro version bought, but the official releases only
              # provide a *.deb file, so:
              withPro = true;
            })
          ];
        };
    };
  };
}

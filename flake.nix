{
  description = "Flake for shell to install 7th Heaven on NixOS distro";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in {
    devShells.x86_64-linux.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        killall
        perl
        curl
        xdg-user-dirs
        zenity
      ];
      shellHook = ''
        echo "Run 'steam-run ./install.sh' for installing 7th Heaven"
      '';
    };
  };
}

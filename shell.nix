# $ cat ~/.nix-defexpr/channels/nixpkgs/.git-revision
# REVISION
# $ nix-prefetch-url --unpack https://github.com/NixOS/nixpkgs/archive/REVISION.tar.gz
# SHA256

let
    nixpkgs = (import (builtins.fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/cc1ae9f21b9e0ce998e706a3de1bad0b5259f22d.tar.gz";
        sha256 = "0zjafww05h50ncapw51b5qxgbv9prjyag0j22jnfc3kcs5xr4ap0";
    })) {};
in

with nixpkgs;
let
    shared = [
        jq
        openssl
        rustup
        shellcheck
    ];
    hook = ''
        . .env
        . .shellhook
    '';
in
{
    darwin = mkShell {
        buildInputs = shared;
        shellHook = hook;
    };
    linux = mkShell {
        buildInputs = [
            pkg-config
        ] ++ shared;
        shellHook = hook;
    };
}

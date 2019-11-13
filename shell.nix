with import <nixpkgs> {};
let
    shared = [
        jq
        openssl_1_0_2
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
    linux = gccStdenv.mkDerivation {
        name = "_";
        buildInputs = [
            pkg-config
        ] ++ shared;
        shellHook = hook;
    };
}
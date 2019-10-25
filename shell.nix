with import <nixpkgs> {};
let
    shared = [
        jq
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
        buildInputs = [
            openssl_1_0_2
        ] ++ shared;
        shellHook = hook;
    };
    linux = mkShell {
        buildInputs = [
            openssl
            pkg-config
        ] ++ shared;
        shellHook = hook;
    };
}

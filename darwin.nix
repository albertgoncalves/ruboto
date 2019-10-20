with import <nixpkgs> {};
mkShell {
    buildInputs = [
        jq
        openssl_1_0_2
        rustup
        shellcheck
    ];
    shellHook = ''
        . .env
        . .shellhook
    '';
}

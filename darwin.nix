with import <nixpkgs> {};
mkShell {
    buildInputs = [
        jq
        openssl
        rlwrap
        rustup
        shellcheck
    ];
    shellHook = ''
        . .env
        . .shellhook
    '';
}

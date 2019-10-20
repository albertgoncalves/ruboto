with import <nixpkgs> {};
mkShell {
    buildInputs = [
        jq
        openssl
        rustup
        shellcheck
    ];
    shellHook = ''
        . .env
        . .shellhook
    '';
}

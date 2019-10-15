with import <nixpkgs> {};
mkShell {
    buildInputs = [
        jq
        openssl
        pkg-config
        rlwrap
        rustup
        shellcheck
    ];
    shellHook = ''
        . .env
        . .shellhook
    '';
}

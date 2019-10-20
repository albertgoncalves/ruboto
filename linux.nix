with import <nixpkgs> {};
mkShell {
    buildInputs = [
        jq
        openssl
        pkg-config
        rustup
        shellcheck
    ];
    shellHook = ''
        . .env
        . .shellhook
    '';
}

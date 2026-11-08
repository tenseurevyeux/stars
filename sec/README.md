# Security Assets (`sec/`)

This directory holds the TLS certificate and private key used by the backend server.

> [!CAUTION]
> The certificates and private keys provided here are mock development keys. Do NOT use them in production.

## Files

- `server.crt`: Server X.509 certificate signed by `certificates/rootCA.pem`.
- `server.key`: Server private key in PKCS#8 format.

To regenerate these certificates, run `./certificates/generate_certs.sh`.

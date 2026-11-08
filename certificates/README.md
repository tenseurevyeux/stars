# Certificates Directory

This directory contains TLS Root Certificate Authorities (CA) used to establish mutual trust between the client and server.

> [!NOTE]
> All certificates in this directory are mock / dummy certificates intended for development, testing, and demonstration. Never use these certificates or private keys in a production environment.

## Files

- `rootCA.pem`: Mock Root Certificate Authority public certificate (used by the client to verify the server's certificate).
- `rootCA.key`: Private key for the Mock Root CA.
- `rootCA-loc.pem`: Mock Local Root Certificate Authority public certificate.
- `rootCA-loc.key`: Private key for the Mock Local Root CA.
- `generate_certs.sh`: Shell script to regenerate mock certificates and server keys.

## Generating New Certificates

You can run the generation script to create a fresh set of mock CA certificates and server keypairs:

```bash
./certificates/generate_certs.sh
```

This will automatically:
1. Generate a 2048-bit RSA Mock Root CA key and certificate (`certificates/rootCA.pem`, `certificates/rootCA.key`).
2. Generate a 2048-bit RSA Mock Local Root CA key and certificate (`certificates/rootCA-loc.pem`, `certificates/rootCA-loc.key`).
3. Generate a 2048-bit RSA Server key in PKCS#8 format (`sec/server.key`).
4. Sign the server certificate (`sec/server.crt`) using `rootCA.pem` with Subject Alternative Names (SAN) for `localhost`, `127.0.0.1`, `0.0.0.0`, and `217.60.2.146`.

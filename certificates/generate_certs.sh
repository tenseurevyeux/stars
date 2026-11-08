#!/usr/bin/env bash
set -euo pipefail

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
SEC_DIR="$ROOT_DIR/sec"

mkdir -p "$SCRIPT_DIR" "$SEC_DIR"

echo "=== Generating Mock Root CA ==="
openssl req -x509 -newkey rsa:2048 -nodes \
  -keyout "$SCRIPT_DIR/rootCA.key" \
  -out "$SCRIPT_DIR/rootCA.pem" \
  -days 3650 \
  -subj "/C=US/ST=MockState/L=MockCity/O=Mock University/OU=IT/CN=MockRootCA" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,digitalSignature,cRLSign,keyCertSign" \
  -addext "subjectKeyIdentifier=hash"

echo "=== Generating Mock Local Root CA ==="
openssl req -x509 -newkey rsa:2048 -nodes \
  -keyout "$SCRIPT_DIR/rootCA-loc.key" \
  -out "$SCRIPT_DIR/rootCA-loc.pem" \
  -days 3650 \
  -subj "/C=US/ST=MockState/L=MockCity/O=Mock University/OU=IT/CN=MockLocalRootCA" \
  -addext "basicConstraints=critical,CA:TRUE" \
  -addext "keyUsage=critical,digitalSignature,cRLSign,keyCertSign" \
  -addext "subjectKeyIdentifier=hash"

echo "=== Generating Mock Server Certificate and Key ==="
TMP_KEY="$(mktemp)"
TMP_CSR="$(mktemp)"
TMP_EXT="$(mktemp)"

openssl req -newkey rsa:2048 -nodes \
  -keyout "$TMP_KEY" \
  -out "$TMP_CSR" \
  -subj "/C=US/ST=MockState/L=MockCity/O=Mock University/CN=localhost"

# Convert server key to standard PKCS#8 format for rustls
openssl pkcs8 -topk8 -nocrypt -in "$TMP_KEY" -out "$SEC_DIR/server.key"
rm -f "$TMP_KEY"

cat > "$TMP_EXT" << 'EOF'
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage = digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
extendedKeyUsage = serverAuth
subjectAltName = @alt_names

[alt_names]
DNS.1 = localhost
IP.1 = 127.0.0.1
IP.2 = 0.0.0.0
IP.3 = 217.60.2.146
EOF

openssl x509 -req -in "$TMP_CSR" \
  -CA "$SCRIPT_DIR/rootCA.pem" \
  -CAkey "$SCRIPT_DIR/rootCA.key" \
  -CAcreateserial \
  -out "$SEC_DIR/server.crt" \
  -days 3650 \
  -extfile "$TMP_EXT"

rm -f "$TMP_CSR" "$TMP_EXT" "$SCRIPT_DIR/rootCA.srl"

echo "=== Done! Generated: ==="
echo "  - $SCRIPT_DIR/rootCA.pem"
echo "  - $SCRIPT_DIR/rootCA.key"
echo "  - $SCRIPT_DIR/rootCA-loc.pem"
echo "  - $SCRIPT_DIR/rootCA-loc.key"
echo "  - $SEC_DIR/server.crt"
echo "  - $SEC_DIR/server.key"

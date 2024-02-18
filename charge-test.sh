#!/bin/sh

siege -c 150 -r 10 -t10s --content-type="application/json" 'http://127.0.0.1:9999/clientes/2/transacoes POST {
  "valor": 10000,
  "tipo": "d",
  "descricao": "Depósito"
}'

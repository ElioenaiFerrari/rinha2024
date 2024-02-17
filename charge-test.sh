#!/bin/sh

siege -c 200 -r 1 -t30s --content-type="application/json" 'http://127.0.0.1:9999/clientes/3/transacoes POST {
  "valor": 1000,
  "tipo": "d",
  "descricao": "Depósito em dinheiro"
}'

CREATE TABLE wallets (
  id SERIAL PRIMARY KEY,
  saldo INT DEFAULT 0,
  limite INT DEFAULT 0
);

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  wallet_id INT REFERENCES wallets(id),
  valor INT DEFAULT 0,
  tipo VARCHAR(1),
  descricao TEXT,
  realizada_em TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO
  wallets (id, saldo, limite)
VALUES
  (1, 0, 100000),
  (2, 0, 80000),
  (3, 0, 1000000),
  (4, 0, 10000000),
  (5, 0, 500000);
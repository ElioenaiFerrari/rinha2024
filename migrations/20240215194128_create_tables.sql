CREATE TABLE wallets (
  id SERIAL PRIMARY KEY,
  saldo INT DEFAULT 0,
  limite INT DEFAULT 0,
  limite_usado INT DEFAULT 0
);

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  wallet_id INT REFERENCES wallets(id) NOT NULL,
  valor INT NOT NULL,
  tipo VARCHAR(1) NOT NULL,
  descricao VARCHAR(10) NOT NULL,
  realizada_em TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX transactions_wallet_id_index ON transactions (wallet_id);

CREATE INDEX transactions_realizada_em_index ON transactions (realizada_em);

INSERT INTO
  wallets (id, saldo, limite)
VALUES
  (1, 0, 100000),
  (2, 0, 80000),
  (3, 0, 1000000),
  (4, 0, 10000000),
  (5, 0, 500000);
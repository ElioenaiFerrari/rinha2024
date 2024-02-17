use actix_web::{
    get,
    middleware::{Compress, Logger},
    post,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Wallet {
    #[serde(skip)]
    pub id: i32,
    pub limite: i32,
    pub saldo: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct WalletView {
    pub limite: i32,
    pub saldo: i32,
    pub data_extrato: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    #[serde(skip)]
    pub id: i32,
    #[serde(skip)]
    pub wallet_id: i32,
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
    pub realizada_em: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    pub valor: i32,
    pub tipo: String,
    pub descricao: String,
}

struct State {
    pub pool: Pool<Postgres>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MainView {
    pub saldo: WalletView,
    pub ultimas_transacoes: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorView {
    pub message: &'static str,
}

#[get("clientes/{id}/extrato")]
async fn get_transactions(state: Data<State>, path: Path<i32>) -> impl Responder {
    let wallet_id = path.into_inner();

    let wallet = match sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE id = $1")
        .bind(&wallet_id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(wallet) => wallet,
        Err(_) => {
            return HttpResponse::NotFound().json(ErrorView {
                message: "wallet not found",
            });
        }
    };

    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE wallet_id = $1 ORDER BY realizada_em DESC LIMIT 10",
    )
    .bind(&wallet_id)
    .fetch_all(&state.pool)
    .await
    .expect("failed to fetch transactions");

    HttpResponse::Ok().json(MainView {
        saldo: WalletView {
            limite: wallet.limite,
            saldo: wallet.saldo,
            data_extrato: chrono::Local::now().naive_local(),
        },
        ultimas_transacoes: transactions,
    })
}

#[post("/clientes/{id}/transacoes")]
async fn create_transaction(
    state: Data<State>,
    path: Path<i32>,
    request: Json<Request>,
) -> impl Responder {
    let wallet_id = path.into_inner();

    if request.tipo.ne("d") && request.tipo.ne("c") {
        return HttpResponse::BadRequest().json(ErrorView {
            message: "invalid transaction type",
        });
    }

    if request.descricao.len() > 10 || request.descricao.is_empty() {
        return HttpResponse::BadRequest().json(ErrorView {
            message: "description too long",
        });
    }

    if request.valor < 1 {
        return HttpResponse::BadRequest().json(ErrorView {
            message: "invalid value",
        });
    }

    let wallet = match sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE id = $1")
        .bind(&wallet_id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(wallet) => wallet,
        Err(_) => {
            return HttpResponse::NotFound().json(ErrorView {
                message: "wallet not found",
            });
        }
    };

    let new_balance = wallet.saldo - request.valor;
    if request.tipo.eq("d") {
        if wallet.limite.lt(&new_balance.abs()) {
            return HttpResponse::UnprocessableEntity().json(ErrorView {
                message: "insufficient funds",
            });
        }

        sqlx::query("UPDATE wallets SET saldo = $1 WHERE id = $2")
            .bind(&new_balance)
            .bind(&wallet.id)
            .execute(&state.pool)
            .await
            .expect("failed to update wallet");
    } else {
        sqlx::query("UPDATE wallets SET saldo = $1 WHERE id = $2")
            .bind(&new_balance)
            .bind(&wallet.id)
            .execute(&state.pool)
            .await
            .expect("failed to update wallet");
    }

    sqlx::query("INSERT INTO transactions (wallet_id, valor, tipo, descricao) VALUES ($1, $2, $3, $4) RETURNING *")
        .bind(&wallet.id)
        .bind(&request.valor)
        .bind(&request.tipo)
        .bind(&request.descricao)
        .fetch_one(&state.pool)
        .await
        .expect("failed to insert transaction");

    HttpResponse::Ok().json(wallet)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("env DATABASE_URL not found");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "%a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\"",
            ))
            .wrap(Compress::default())
            .app_data(Data::new(State { pool: pool.clone() }))
            .service(create_transaction)
            .service(get_transactions)
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}

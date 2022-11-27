use std::env;

use clap::Parser;
use cli::{Args, Commands};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

mod cli;
mod commands;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| {
        panic!("Error connecting to {}", database_url)
    })
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let mut conn = establish_connection();

    match args.command {
        Commands::User(cmd) => cmd.run(&mut conn),
    }

    // use openlms_core::schema::users::dsl::*;

    // let results =
    //     users.load::<User>(conn).expect("error loading users");

    // for user in results {
    //     eprint!("User = {:?}", user);
    // }
}

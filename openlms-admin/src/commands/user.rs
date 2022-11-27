use anyhow::Result;
use clap::{Parser, Subcommand};
use diesel::PgConnection;
use openlms_core::models::{NewUser, User};

/// Commands to add, delete or modify user accounts.
#[derive(Parser, Debug)]
pub struct UserCmd {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Adds a new user account.
    Add {
        username: String,

        #[arg(short, long)]
        password: Option<String>,
    },
}

impl UserCmd {
    pub fn run(&self, conn: &mut PgConnection) -> Result<()> {
        match self.command {
            Commands::Add { .. } => self.add(conn),
        }
    }

    pub fn add(&self, conn: &mut PgConnection) -> Result<()> {
        let Commands::Add { username, password } = &self.command;
        let password: String = match password {
            Some(password) => password.into(),
            None => rpassword::prompt_password("password: ").unwrap(),
        };

        let mut new_user = NewUser::new(username, &password);
        let user = User::create(conn, &mut new_user);
        eprintln!("user = {:?}", user);

        Ok(())
    }
}

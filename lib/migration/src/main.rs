use std::{env, fs};
use std::io::Write;
use tokio_postgres::{Client, NoTls};

use colored::Colorize;
use time::format_description;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!(r"..\..\backend\database\migrations");
}

#[tokio::main]
async fn main() {
    // get a client
    let (client, connection) =
        tokio_postgres::connect("host=142.93.230.221 user=postgres password=mauFJcuf5dhRMQrjj dbname=kartinggroningen", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("{}", dump_schema(&client).await);

    migrate(client).await;
}


pub async fn migrate(mut client: Client) {
    // get a runner for the migrations
    let runner = embedded::migrations::runner();
    println!("{}", "Running migrations...".on_cyan());

    // apply the migrations and get the applied migrations so we can check if they are new
    let now_applied = match runner.run_async(&mut client).await {
        Ok(report) => {
            // dump the schema
            println!("{}", "Dumping schema...".on_cyan());
            dump_schema_to_file(r"..\..\backend\database\schema.sql", &client);

            //
            let ams = report.applied_migrations();
            // check if they have applied
            if !ams.is_empty() {
                println!("Applied {} migrations:", ams.len());
            } else {
                println!("No migrations applied");
            }

            ams.to_owned()
        }

        Err(e) => {
            println!("Failed to migrate: {}", &e.kind().to_string().red());

            if e.report().is_none() {
                Vec::new().to_owned()
            } else {
                e.report().unwrap().applied_migrations().to_owned()
            }
        }
    };

    // show the applioed migrations in the terminal.
    let applied = runner.get_applied_migrations_async(&mut client).await.unwrap();

    for m in runner.get_migrations() {
        let mut succeed = None;

        match m.applied_on() {
            Some(d) => {
                succeed = Some(d.to_owned());
            }
            None => {
                // check if it is in applied
                let ap = applied.iter().filter(|a| m.name() == a.name()).next();
                if !ap.is_none() {
                    succeed = Some(ap.unwrap().applied_on().unwrap().to_owned());
                }
            }
        };

        let s: String;
        if succeed.is_none() {
            s = String::from("Failed");
        } else {
            let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]").unwrap();
            s = succeed.unwrap().format(&format).unwrap();
        }

        let msg = format!("\t- {}{} {}: ({})", m.prefix(), m.version(), m.name(), s);

        if succeed.is_none() {
            println!("{}", msg.cyan());
        } else {
            let ap = now_applied.iter().filter(|a| m.name() == a.name()).next();

            if !ap.is_none() {
                println!("{}", msg.green());
            } else {
                println!("{}", msg.yellow());
            }
        }
    }


    let last = runner.get_last_applied_migration_async(&mut client).await;
    match last {
        Ok(l) => {
            if l.is_none() {
                println!("\nCould not determine database version");
                return;
            }
            let l = l.unwrap();

            println!("\nDatabase version: {}{} {}.", l.prefix(), l.version(), l.name())
        }
        Err(_) => {
            println!("\nCould not determine database version");
        }
    }
}


async fn dump_schema_to_file(file_path: &str, client: &Client) {
    let schema = dump_schema(&client).await;

    // open the file
    let mut f = fs::OpenOptions::new().write(true).open(file_path).unwrap();
    // clear the file
    f.set_len(0).unwrap();

    // write the schema to the file
    f.write_all(schema.as_bytes()).unwrap();
    f.flush().unwrap();

}


pub async fn dump_schema(client: &Client) -> String {
    let mut path = env::current_dir().unwrap().display()
        .to_string();
    path.push_str(r"\src\schema_dump.sql");

    let q = fs::read_to_string(&path).unwrap();

    let rows = client
        .query(&q, &[])
        .await.unwrap();

    let mut schema = String::new();
    for stmt in rows {
        schema.push_str(stmt.get(0));
        schema.push_str("\n");
    }

    return schema;
}

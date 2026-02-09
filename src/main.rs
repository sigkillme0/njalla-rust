use clap::{Parser, Subcommand};
use njalla::{NewRecord, NewServer, NjallaClient};

#[derive(Debug, Parser)]
#[command(name = "njalla", about = "cli toolkit for njal.la")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    #[command(subcommand, about = "domain operations")]
    Domain(DomainCmd),
    #[command(subcommand, about = "dns record operations")]
    Record(RecordCmd),
    #[command(subcommand, about = "server operations")]
    Server(ServerCmd),
}

#[derive(Debug, Subcommand)]
enum DomainCmd {
    #[command(about = "list all domains")]
    List,
    #[command(about = "get domain details")]
    Get { domain: String },
    #[command(about = "search available domains")]
    Find { query: String },
    #[command(about = "register a new domain")]
    Register {
        domain: String,
        #[arg(default_value = "1")]
        years: u32,
    },
    #[command(about = "check async task status")]
    CheckTask { id: String },
}

#[derive(Debug, Subcommand)]
enum RecordCmd {
    #[command(about = "list dns records for a domain")]
    List { domain: String },
    #[command(about = "add a dns record")]
    Add {
        domain: String,
        #[arg(short, long)]
        name: String,
        #[arg(short = 't', long = "type")]
        record_type: String,
        #[arg(short, long)]
        content: String,
        #[arg(long, default_value = "3600")]
        ttl: u32,
        #[arg(short, long)]
        priority: Option<u32>,
    },
    #[command(about = "edit a dns record")]
    Edit {
        domain: String,
        id: String,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short = 't', long = "type")]
        record_type: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
        #[arg(long)]
        ttl: Option<u32>,
        #[arg(short, long)]
        priority: Option<u32>,
    },
    #[command(about = "remove a dns record")]
    Remove { domain: String, id: String },
}

#[derive(Debug, Subcommand)]
enum ServerCmd {
    #[command(about = "list all servers")]
    List,
    #[command(about = "list available os images")]
    Images,
    #[command(about = "list available server types")]
    Types,
    #[command(about = "add a new server")]
    Add {
        name: String,
        #[arg(short = 't', long = "type")]
        server_type: String,
        #[arg(short, long)]
        os: String,
        #[arg(short, long)]
        ssh_key: String,
        #[arg(short, long, default_value = "1")]
        months: u32,
    },
    #[command(about = "stop a server")]
    Stop { id: String },
    #[command(about = "start a server")]
    Start { id: String },
    #[command(about = "restart a server")]
    Restart { id: String },
    #[command(about = "factory reset a server (destroys data)")]
    Reset {
        id: String,
        #[arg(short, long)]
        os: String,
        #[arg(short, long)]
        ssh_key: String,
        #[arg(short = 't', long = "type")]
        server_type: String,
    },
    #[command(about = "remove a server (destroys data)")]
    Remove { id: String },
}

fn dump<T: serde::Serialize>(val: &T) -> njalla::error::Result<()> {
    let s = serde_json::to_string_pretty(val)?;
    println!("{s}");
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let client = match NjallaClient::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to init client: {e}");
            eprintln!("set NJALLA_API_TOKEN in .env or environment");
            std::process::exit(1);
        }
    };

    if let Err(e) = run(cli, &client).await {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

async fn run(cli: Cli, client: &NjallaClient) -> njalla::error::Result<()> {
    match cli.cmd {
        Cmd::Domain(cmd) => run_domain(cmd, client).await,
        Cmd::Record(cmd) => run_record(cmd, client).await,
        Cmd::Server(cmd) => run_server(cmd, client).await,
    }
}

async fn run_domain(cmd: DomainCmd, client: &NjallaClient) -> njalla::error::Result<()> {
    match cmd {
        DomainCmd::List => dump(&client.list_domains().await?)?,
        DomainCmd::Get { domain } => dump(&client.get_domain(&domain).await?)?,
        DomainCmd::Find { query } => dump(&client.find_domains(&query).await?)?,
        DomainCmd::Register { domain, years } => {
            let task = client.register_domain(&domain, years).await?;
            println!("registration task started: {task}");
            println!("poll with: njalla domain check-task {task}");
        }
        DomainCmd::CheckTask { id } => {
            let status = client.check_task(&id).await?;
            println!("{status}");
        }
    }
    Ok(())
}

async fn run_record(cmd: RecordCmd, client: &NjallaClient) -> njalla::error::Result<()> {
    match cmd {
        RecordCmd::List { domain } => dump(&client.list_records(&domain).await?)?,
        RecordCmd::Add {
            domain,
            name,
            record_type,
            content,
            ttl,
            priority,
        } => {
            let rec = NewRecord {
                name,
                record_type,
                content,
                ttl,
                priority,
            };
            dump(&client.add_record(&domain, &rec).await?)?;
        }
        RecordCmd::Edit {
            domain,
            id,
            name,
            record_type,
            content,
            ttl,
            priority,
        } => {
            let records = client.list_records(&domain).await?;
            let existing = records
                .iter()
                .find(|r| r.id.as_deref() == Some(&id))
                .ok_or_else(|| njalla::Error::NotFound(format!("record {id} in {domain}")))?;
            let patched = njalla::Record {
                id: Some(id),
                name: name.unwrap_or_else(|| existing.name.clone()),
                record_type: record_type.unwrap_or_else(|| existing.record_type.clone()),
                content: content.unwrap_or_else(|| existing.content.clone()),
                ttl: ttl.unwrap_or(existing.ttl),
                priority: priority.or(existing.priority),
            };
            client.edit_record(&domain, &patched).await?;
            println!("record updated");
        }
        RecordCmd::Remove { domain, id } => {
            client.remove_record(&domain, &id).await?;
            println!("record {id} removed");
        }
    }
    Ok(())
}

async fn run_server(cmd: ServerCmd, client: &NjallaClient) -> njalla::error::Result<()> {
    match cmd {
        ServerCmd::List => dump(&client.list_servers().await?)?,
        ServerCmd::Images => dump(&client.list_server_images().await?)?,
        ServerCmd::Types => dump(&client.list_server_types().await?)?,
        ServerCmd::Add {
            name,
            server_type,
            os,
            ssh_key,
            months,
        } => {
            let srv = NewServer {
                name,
                server_type,
                os,
                ssh_key,
                months,
            };
            dump(&client.add_server(&srv).await?)?;
        }
        ServerCmd::Stop { id } => dump(&client.stop_server(&id).await?)?,
        ServerCmd::Start { id } => dump(&client.start_server(&id).await?)?,
        ServerCmd::Restart { id } => dump(&client.restart_server(&id).await?)?,
        ServerCmd::Reset {
            id,
            os,
            ssh_key,
            server_type,
        } => {
            dump(
                &client
                    .reset_server(&id, &os, &ssh_key, &server_type)
                    .await?,
            )?;
        }
        ServerCmd::Remove { id } => dump(&client.remove_server(&id).await?)?,
    }
    Ok(())
}

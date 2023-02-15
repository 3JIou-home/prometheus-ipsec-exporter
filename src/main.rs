use axum::routing::get;
use axum::Router;
use
clap::Parser;
use std::net::SocketAddr;
use std::process::Command;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short, long)]
    bind: String,
}

async fn ipsec_metrics() -> String {
    let ipsec_status = Command::new("sh")
        .arg("-c")
        .arg("ipsec status | awk {'print $1'} | grep -v 'Security' | cut -d \"{\" -f1 | cut -d \"[\" -f1 | uniq  | wc -l")
        .output()
        .expect("Failed to execute command: `ipsec status`");

    let tunnel_count = std::str::from_utf8(&ipsec_status.stdout)
        .unwrap_or("0")
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    let ip_xfrm_state = Command::new("sh")
        .arg("-c")
        .arg("ip xfrm state | grep 'src' |wc -l")
        .output()
        .expect("Failed to execute command: `ip xfrm state`");

    let connections_count = std::str::from_utf8(&ip_xfrm_state.stdout)
        .unwrap_or("0")
        .trim()
        .parse::<u32>()
        .unwrap_or(0);

    format!(
        "ipsec.vpn.active_tunnels {}\nipsec.vpn.active_connection: {}\n",
        tunnel_count, connections_count
    )
}

#[tokio::main]
async fn main() {
    let socket = &Args::parse().bind;
    // Init socket from config file.
    let server: SocketAddr = socket.parse().unwrap();
    // Init router.
    let app = Router::new().route("/metrics", get(ipsec_metrics));
    // Start server.
    axum::Server::bind(&server)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

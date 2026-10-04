use tokio::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    loop {
        println!("Starting SSH tunnel...");

        let mut ssh = Command::new("ssh")
            .args([
                "-N",
                "-o", "BatchMode=yes",
                "-o", "ServerAliveInterval=30",
                "-o", "ServerAliveCountMax=3",
                "-o", "ExitOnForwardFailure=yes",
                "-L", "9000:127.0.0.1:8000",
                "mark-cook@localhost",
            ])
            .spawn()?;

        let status = ssh.wait().await?;

        println!("SSH tunnel terminated: {status}");
        println!("Retrying in 5 seconds...");

        sleep(Duration::from_secs(5)).await;
    }
}
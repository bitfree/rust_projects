#[derive(Debug)]
use aya::Bpf;
use aya::programs::TracePoint;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the BPF object from the compiled eBPF bytecode
    let mut bpf = Bpf::load_file("trace.o")?;

    // Load and attach the TracePoint program
    let mut tracepoint: TracePoint = bpf.program_mut("tracepoint_sched_switch")?
        .try_into()
        .map_err(|_| "Failed to convert to TracePoint")?;
    tracepoint.load()?;
    tracepoint.attach("sched", "sched_switch")?;

    // Create a channel to receive events from the BPF program
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Clone the sender to move into the poll task
    let poll_tx = tx.clone();

    // Spawn a task to poll events from the BPF program
    let poll_handle = task::spawn(async move {
        loop {
            match bpf.poll(Duration::from_millis(100)) {
                Ok(event) => {
                    // Process the event as needed
                    let log = format!("{:?}", event);
                    if poll_tx.send(log).await.is_err() {
                        break;
                    }
                }
                Err(e) => eprintln!("Error polling BPF: {}", e),
            }
        }
    });

    // Spawn a task to write logs to a file
    let write_handle = task::spawn(async move {
        let mut file = File::create("trace.log").expect("Failed to create log file");
        while let Some(log) = rx.recv().await {
            if let Err(e) = writeln!(file, "{}", log) {
                eprintln!("Error writing to log file: {}", e);
            }
        }
    });

    // Wait for Ctrl+C signal to gracefully shutdown
    signal::ctrl_c().await?;
    println!("Shutting down...");

    // Dropping the sender will close the channel
    drop(tx);

    // Wait for tasks to complete
    poll_handle.await?;
    write_handle.await?;

    Ok(())
}

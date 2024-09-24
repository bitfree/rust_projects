use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworkExt};
use std::{thread, time::Duration};

fn main() {
    // 새로운 System 객체 생성
    let mut sys = System::new_all();

    // 정보를 업데이트하고 5초마다 출력하는 루프
    loop {
        // 시스템 정보 업데이트
        sys.refresh_all();

        // CPU 정보 출력
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        println!("CPU Usage: {:.2}%", cpu_usage);

        // 메모리 정보 출력
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        println!("Memory Usage: {} / {} KB", used_memory, total_memory);

        // 디스크 정보 출력
        for disk in sys.disks() {
            println!(
                "Disk {}: {} / {} bytes used",
                disk.name().to_string_lossy(),
                disk.total_space() - disk.available_space(),
                disk.total_space()
            );
        }

        // 네트워크 정보 출력
        for (interface_name, data) in sys.networks() {
            println!(
                "Network Interface {}: received {} bytes, transmitted {} bytes",
                interface_name,
                data.received(),
                data.transmitted()
            );
        }

        // 5초 동안 대기
        thread::sleep(Duration::from_secs(5));
        println!("--------------------------------------------");
    }
}


/*
fn summarize_fleet(fleet: &[Server]) -> FleetSummary {
    let mut healthy = Vec::new();
    let mut degraded = Vec::new();
    let mut failed = Vec::new();
    let avg_power = fleet.iter().map(|server| server.power_draw()).sum() / fleet.len();
    let max_temp = fleet.iter().map(|server| server.max_temperature()).max();

    for server in fleet {
        match server.health_status() {
            Health::Healthy => healthy.push(server.id.clone()),
            Health::Degraded(reason) => degraded.push((server.id.clone(), reason)),
            Health::Failed(err) => failed.push((server.id.clone(), err)),
        }
    }

    FleetSummary {
        healthy,
        degraded,
        failed,
        avg_power,
        max_temp,
    }
}
*/

fn main() {
    println!("Hello, world!");
}

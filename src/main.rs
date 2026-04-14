use rand::Rng;
use constraint_theory_core::PythagoreanManifold;
fn main() {
    let mut rng = rand::thread_rng();
    let manifold = PythagoreanManifold::new(1024);
    for k in 4..21 {
        let mut rigid_count = 0;
        let mut cohesive_count = 0;
        for _ in 0..1000 {
            let mut edges = 0;
            let mut graph: Vec<Vec<bool>> = vec![vec![false; 1024]; 1024];
            for i in 0..1024 {
                for j in 0..k {
                    let neighbor = (i + j) % 1024;
                    graph[i][neighbor] = true;
                    edges += 1;
                }
            }
            edges /= 2;
            let is_rigid = edges >= 2 * 1024 - 3 && k >= 12;
            if is_rigid {
                rigid_count += 1;
                let mut cohesion_metric = 0.0;
                for i in 0..1024 {
                    let mut position = [0.0; 2];
                    position[0] = rng.gen();
                    position[1] = rng.gen();
                    let (_position, distance) = manifold.snap(position);
                    cohesion_metric += distance;
                }
                if cohesion_metric < 100.0 {
                    cohesive_count += 1;
                }
            }
        }
        let percent_rigid = (rigid_count as f64) / 1000.0;
        let percent_cohesive = (cohesive_count as f64) / 1000.0;
        println!({} {} {} {}, k, percent_rigid, percent_cohesive, (rigid_count as f64) / (cohesive_count as f64 + 1.0));
    }
}

use rand::seq::SliceRandom;
use necklace_permutations::necklace_perm;
use tsplib::TspData;

fn main() {
    let tsp_path = std::path::Path::new("a280x.tsp");
    let tsp_data = TspData::load(&tsp_path).unwrap();
    println!("{:?}", tsp_data);

    let mut route = Vec::<usize>::with_capacity(tsp_data.dimension);
    for i in 0..tsp_data.dimension {
        route.push(tsp_data.nodes[i].node_no);
    }

    println!("*** Random Search ***");
    let mut total_dist_min = f32::MAX;
    let mut rng = rand::thread_rng();
    for n in 0..1814400 {
        (&mut route[1..]).shuffle(&mut rng);
        let total_dist_cur = tsp_data.calc_distance(&route);
        if total_dist_cur < total_dist_min {
            total_dist_min = total_dist_cur;
        }
        if (total_dist_min - 180.90567) < 0.1 {
            println!("{:8}: total_dist_min={:.1}: Found {:?}", n + 1, total_dist_min, route);
            break;
        }
        if n % 100000 == 0 {
            println!("{:8}: total_dist_min={:.1}: {:?}", n + 1, total_dist_min, route);
        }
    }
    println!("total_dist_min={:?}", total_dist_min);
    route.clear();

    println!("*** Brute Force ***");
    for i in 0..tsp_data.dimension {
        route.push(tsp_data.nodes[i].node_no);
    }
    (&mut route[1..]).shuffle(&mut rng);

    let mut n = 0;
    for route_cur in necklace_perm(route) {
        let total_dist_cur = tsp_data.calc_distance(&route_cur);
        if total_dist_cur < total_dist_min {
            total_dist_min = total_dist_cur;
        }
        if (total_dist_min - 180.90567) < 0.1 {
            println!("{:8}: total_dist_min={:.1}: Found {:?}", n + 1, total_dist_min, route_cur);
            break;
        }
        if n % 100000 == 0 {
            println!("{:8}: total_dist_min={:.1}: {:?}", n + 1, total_dist_min, route_cur);
        }
        n += 1;
    }
    println!("total_dist_min={:?}", total_dist_min);

}

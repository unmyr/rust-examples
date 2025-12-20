use rand::seq::SliceRandom;
use plotters::prelude::*;
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
    let mut rng = rand::rng();
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
    let mut result_cur: Vec<usize> = route.clone();
    for route_cur in necklace_perm(route) {
        let total_dist_cur = tsp_data.calc_distance(&route_cur);
        if total_dist_cur < total_dist_min {
            total_dist_min = total_dist_cur;
        }
        if (total_dist_min - 180.90567) < 0.1 {
            println!("{:8}: total_dist_min={:.1}: Found {:?}", n + 1, total_dist_min, route_cur);
            result_cur = route_cur.clone();
            break;
        }
        if n % 100000 == 0 {
            println!("{:8}: total_dist_min={:.1}: {:?}", n + 1, total_dist_min, route_cur);
        }
        n += 1;
    }
    println!("total_dist_min={:?}", total_dist_min);

    let mut series: Vec<(f32, f32)> = Vec::with_capacity(result_cur.len());
    let mut p_min = (f32::MAX, f32::MAX);
    let mut p_max = (f32::MIN, f32::MIN);
    for i in result_cur {
        series.push((tsp_data.nodes[i-1].x as f32, tsp_data.nodes[i-1].y as f32));
        if (tsp_data.nodes[i-1].x as f32) < p_min.0 {
            p_min.0 = tsp_data.nodes[i-1].x as f32;
        }
        if (tsp_data.nodes[i-1].x as f32) > p_max.0 {
            p_max.0 = tsp_data.nodes[i-1].x as f32;
        }
        if (tsp_data.nodes[i-1].y as f32) < p_min.1 {
            p_min.1 = tsp_data.nodes[i-1].y as f32;
        }
        if (tsp_data.nodes[i-1].y as f32) > p_max.1 {
            p_max.1 = tsp_data.nodes[i-1].y as f32;
        }
    }
    println!("series={:?}", series);
    let root = BitMapBackend::new("./images/tsp.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption("This is our first plot", ("Arial",40).into_font())
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d((p_min.0-1.) .. (p_max.0 + 10.), (p_min.1 - 1.) .. (p_max.1 + 1.)).unwrap();

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw().unwrap();

    // And we can draw something in the drawing area
    chart.draw_series(LineSeries::new(
        series.clone(),
        &RED,
    )).unwrap();
    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        series,
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("Arial", 10).into_font());
        },
    )).unwrap();
}

// use plotters::backend;
use plotters::prelude::*;
use rand_distr::num_traits;

// Parameters of individual layers in neural network training
#[derive(Clone)]
struct LayerParams<F: num_traits::Float> {
    weight: ndarray::Array2<F>,
    bias: ndarray::Array2<F>,
}

// Implement constructor for LayerParams
impl<F: num_traits::Float> LayerParams<F> {
    pub fn new(weight: ndarray::Array2<F>, bias: ndarray::Array2<F>) -> LayerParams<F> {
        LayerParams { weight, bias }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let program_name = match args.get(0) {
        Some(arg0) => std::path::Path::new(arg0)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        None => "plotters_ml_linear_mapping",
    };

    // Data series to plot
    let mut lt_series: Vec<Vec<((f32, f32), (f32, f32))>> = vec![];
    let mut sig1_series: Vec<Vec<((f32, f32), (f32, f32))>> = vec![];
    let mut sig2_series: Vec<Vec<((f32, f32), (f32, f32))>> = vec![];
    let mut l1_out_sig_series: Vec<Vec<((f32, f32), (f32, f32))>> = vec![];

    // Define multiple sets of layer parameters for testing
    let mut all_layer_params: Vec<Vec<LayerParams<f32>>> = vec![];

    // Original data: xor_reg_scratch_20260203_224105_ng_L02_sigmoid.jsonl
    let mut layer_params: Vec<LayerParams<f32>> = Vec::new();
    let weight_2xk = ndarray::arr2(&[[2.5040, -6.0380], [4.6175, 6.7296]]);
    let bias_kx1 = ndarray::arr2(&[[-1.7836], [-1.1378]]);
    layer_params.push(LayerParams::new(weight_2xk.clone(), bias_kx1.clone()));
    let weight_2xk = ndarray::arr2(&[[4.1965, 4.4564]]);
    let bias_kx1 = ndarray::arr2(&[[-4.4478]]);
    layer_params.push(LayerParams::new(weight_2xk.clone(), bias_kx1.clone()));
    all_layer_params.push(layer_params.clone());

    // Original data: xor_reg_scratch_20260206_013211_ok_L02_sigmoid.jsonl
    let mut layer_params: Vec<LayerParams<f32>> = Vec::new();
    let weight_2xk = ndarray::arr2(&[[-3.7951, -3.7615], [-5.8363, -5.6243]]);
    let bias_kx1 = ndarray::arr2(&[[5.5450], [2.1126]]);
    layer_params.push(LayerParams::new(weight_2xk.clone(), bias_kx1.clone()));
    let weight_2xk = ndarray::arr2(&[[7.3841, -7.7409]]);
    let bias_kx1 = ndarray::arr2(&[[-3.3478]]);
    layer_params.push(LayerParams::new(weight_2xk.clone(), bias_kx1.clone()));
    all_layer_params.push(layer_params.clone());

    let test_inputs_kxn: ndarray::Array2<f32> =
        ndarray::arr2(&[[0., 0.], [0., 1.], [1., 0.], [1., 1.]]).reversed_axes();

    // Layer 1 output plot range
    let (mut l1_x_min, mut l1_x_max) = (-0.1f32, 1.1f32);
    let (mut l1_y_min, mut l1_y_max) = (-0.1f32, 1.1f32);
    for layer_params in all_layer_params.iter() {
        let test_layer1_outputs_2xn =
            layer_params[0].weight.dot(&test_inputs_kxn) + &layer_params[0].bias;
        let test_layer1_outputs_2xn_sigmoid =
            test_layer1_outputs_2xn.mapv(|x| 1.0 / (1.0 + (-x).exp()));
        println!("Layer 1 outputs:\n{:?}", test_layer1_outputs_2xn);
        println!(
            "Layer 1 outputs after sigmoid:\n{:?}",
            test_layer1_outputs_2xn_sigmoid
        );
        let test_layer2_outputs_1xn =
            layer_params[1].weight.dot(&test_layer1_outputs_2xn_sigmoid) + &layer_params[1].bias;
        println!("Layer 2 outputs:\n{:?}", test_layer2_outputs_1xn);

        let data_points = (0..4)
            .map(|n| {
                (
                    (test_inputs_kxn[[0, n]], test_inputs_kxn[[1, n]]),
                    (
                        test_layer1_outputs_2xn[[0, n]],
                        test_layer1_outputs_2xn[[1, n]],
                    ),
                )
            })
            .collect::<Vec<((f32, f32), (f32, f32))>>();
        lt_series.push(data_points);
        let data_points = (0..4)
            .map(|n| {
                (
                    (test_inputs_kxn[[0, n]], test_inputs_kxn[[1, n]]),
                    (
                        test_layer1_outputs_2xn[[0, n]],
                        test_layer1_outputs_2xn_sigmoid[[0, n]],
                    ),
                )
            })
            .collect::<Vec<((f32, f32), (f32, f32))>>();
        sig1_series.push(data_points);
        let data_points = (0..4)
            .map(|n| {
                (
                    (test_inputs_kxn[[0, n]], test_inputs_kxn[[1, n]]),
                    (
                        test_layer1_outputs_2xn[[1, n]],
                        test_layer1_outputs_2xn_sigmoid[[1, n]],
                    ),
                )
            })
            .collect::<Vec<((f32, f32), (f32, f32))>>();
        sig2_series.push(data_points);

        // Layer 1 output series
        let data_points = (0..4)
            .map(|n| {
                if l1_x_min > test_layer1_outputs_2xn_sigmoid[[0, n]] {
                    l1_x_min = test_layer1_outputs_2xn_sigmoid[[0, n]];
                }
                if l1_x_max < test_layer1_outputs_2xn_sigmoid[[0, n]] {
                    l1_x_max = test_layer1_outputs_2xn_sigmoid[[0, n]];
                }
                if l1_y_min > test_layer1_outputs_2xn_sigmoid[[1, n]] {
                    l1_y_min = test_layer1_outputs_2xn_sigmoid[[1, n]];
                }
                if l1_y_max < test_layer1_outputs_2xn_sigmoid[[1, n]] {
                    l1_y_max = test_layer1_outputs_2xn_sigmoid[[1, n]];
                }
                (
                    (test_inputs_kxn[[0, n]], test_inputs_kxn[[1, n]]),
                    (
                        test_layer1_outputs_2xn_sigmoid[[0, n]],
                        test_layer1_outputs_2xn_sigmoid[[1, n]],
                    ),
                )
            })
            .collect::<Vec<((f32, f32), (f32, f32))>>();
        l1_out_sig_series.push(data_points);
    }

    // Define the range for x and y axes
    // Linear transformation plot range
    let (lt_x_min, lt_x_max) = (-10f32, 10f32);
    let (lt_y_min, lt_y_max) = (-10f32, 10f32);
    // Sigmoid plot range
    let (sig_x_min, sig_x_max) = (-10f32, 12f32);
    let (sig_y_min, sig_y_max) = (0f32, 1f32);

    // Expand range of layer 1 output plot a bit
    (0..all_layer_params.len()).for_each(|series_index| {
        // Adjust layer 1 output plot range according to the decision boundary
        let a = &all_layer_params[series_index][1].weight[[0, 0]];
        let b = &all_layer_params[series_index][1].weight[[0, 1]];
        let c = &all_layer_params[series_index][1].bias[[0, 0]];
        if l1_x_min > -(b * l1_y_max + c) / a {
            l1_x_min = -(b * l1_y_max + c) / a;
        }
        if l1_x_min > -(b * l1_y_min + c) / a {
            l1_x_min = -(b * l1_y_min + c) / a;
        }
        if l1_x_max < -(b * l1_y_min + c) / a {
            l1_x_max = -(b * l1_y_min + c) / a;
        }
        if l1_x_max < -(b * l1_y_max + c) / a {
            l1_x_max = -(b * l1_y_max + c) / a;
        }
        if l1_y_min > -(a * l1_x_max + c) / b {
            l1_y_min = -(a * l1_x_max + c) / b;
        }
        if l1_y_min > -(a * l1_x_min + c) / b {
            l1_y_min = -(a * l1_x_min + c) / b;
        }
        if l1_y_max < -(a * l1_x_min + c) / b {
            l1_y_max = -(a * l1_x_min + c) / b;
        }
        if l1_y_max < -(a * l1_x_max + c) / b {
            l1_y_max = -(a * l1_x_max + c) / b;
        }
    });

    // For coordinates (i, j), different color palettes are set for (0,0), (0,1), (1,0), and (1,1).
    let colors: Vec<Vec<RGBAColor>> = vec![
        vec![
            Palette99::pick(3).mix(1.),
            Palette99::pick(3).mix(0.7),
            Palette99::pick(3).mix(0.5),
            Palette99::pick(3).mix(0.3),
        ],
        vec![
            Palette99::pick(4).mix(1.),
            Palette99::pick(4).mix(0.7),
            Palette99::pick(4).mix(0.5),
            Palette99::pick(4).mix(0.3),
        ],
    ];

    // Create a new drawing area
    let image_path_buf = std::path::PathBuf::from("../images").join(format!("{program_name}.png"));
    let image_size: (u32, u32) = (1536, 680);
    let root_area = BitMapBackend::new(&image_path_buf, image_size).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // Split the drawing area into multiple sub-areas
    let drawing_areas = root_area
        .split_evenly((2, 1))
        .iter()
        .map(|area| area.split_evenly((1, 5)))
        .collect::<Vec<_>>();

    let caption_normal_font: FontDesc<'static> = ("Arial", 20).into_font();
    // let caption_normal_height: u32 = caption_normal_font.get_size() as u32;

    let caption_small_font: FontDesc<'static> = ("Arial", 14).into_font();
    let caption_small_height: u32 = caption_small_font.get_size() as u32;

    (0..drawing_areas.len()).for_each(|series_index| {
        // After this point, we should be able to draw construct a chart context
        let mut chart_lm = ChartBuilder::on(&drawing_areas[series_index][0])
            .margin(10)
            .x_label_area_size(20)
            .y_label_area_size(40)
            .caption(
                format!("Linear Mapping {}", series_index + 1),
                caption_normal_font.clone(),
            )
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(lt_x_min..lt_x_max, lt_y_min..lt_y_max)
            .unwrap();
        let sig_areas = &drawing_areas[series_index][1].split_evenly((2, 1));
        let mut sig_charts: Vec<ChartContext<_, _>> = sig_areas
            .iter()
            .map(|area| {
                ChartBuilder::on(area)
                    .margin(10)
                    .x_label_area_size(20)
                    .y_label_area_size(40)
                    .caption(
                        format!("Sigmoid Function {}", series_index + 1),
                        caption_normal_font.clone(),
                    )
                    // Finally attach a coordinate on the drawing area and make a chart context
                    .build_cartesian_2d(sig_x_min..sig_x_max, sig_y_min..sig_y_max)
                    .unwrap()
            })
            .collect();

        // Draw mesh for linear mapping plot and sigmoid plots
        chart_lm
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .y_labels(5)
            // We can also change the format of the label text
            .y_label_formatter(&|x| format!("{:.1}", x))
            .draw()
            .unwrap();
        (0..sig_charts.len()).for_each(|i| {
            sig_charts[i]
                .configure_mesh()
                // We can customize the maximum number of labels allowed for each axis
                .x_labels(5)
                .y_labels(5)
                // We can also change the format of the label text
                .y_label_formatter(&|x| format!("{:.1}", x))
                .draw()
                .unwrap();
        });

        // A curve showing the output layer threshold is overlaid on the graph.
        let x1_samples = image_size.0 / 5;
        let a = &all_layer_params[series_index][1].weight[[0, 0]];
        let b = &all_layer_params[series_index][1].weight[[0, 1]];
        let c = &all_layer_params[series_index][1].bias[[0, 0]];
        let (x_min, x_max) = (0.00001, 0.9);
        let x1_step = (x_max - x_min) / (x1_samples as f32);
        let logit = |x: f32| (x / (1_f32 - x)).ln();
        let mut input_series: Vec<(f32, f32)> = vec![];
        for i in 0..x1_samples {
            // aσ(w11 x1 + w12 x2 + b1) + bσ(w21 x1 + w22 x2 + b1) + c = 0
            let l1_out_sigmoid_x1 = x_min + (i as f32) * x1_step;
            let l1_out_sigmoid_x2 = -(a * l1_out_sigmoid_x1 + c) / b;
            if l1_out_sigmoid_x2 < 0.0 {
                continue;
            }
            let l1_out_x1 = logit(l1_out_sigmoid_x1);
            let l1_out_x2 = logit(l1_out_sigmoid_x2);
            let l1_out = ndarray::arr2(&[[l1_out_x1], [l1_out_x2]]);
            let w00 = all_layer_params[series_index][0].weight[[0, 0]];
            let w01 = all_layer_params[series_index][0].weight[[0, 1]];
            let w10 = all_layer_params[series_index][0].weight[[1, 0]];
            let w11 = all_layer_params[series_index][0].weight[[1, 1]];
            let w_norm = w00 * w11 - w01 * w10;
            let w_inv = ndarray::arr2(&[[w11, -w01], [-w10, w00]]) / w_norm;
            let l1_in = w_inv.dot(&(&l1_out - &all_layer_params[series_index][0].bias));

            let p = (l1_in[[0, 0]], l1_in[[1, 0]]);
            input_series.push(p);
        }
        chart_lm
            .draw_series(LineSeries::new(input_series, &Palette99::pick(5).mix(0.5)))
            .unwrap();

        // Linear mapping plot
        chart_lm
            .draw_series(PointSeries::of_element(
                &lt_series[series_index],
                5,
                &RED,
                &|v, s, _st| {
                    // Coordinate before transformation
                    let p0 = v.0;
                    // Coordinate after transformation
                    let p1 = v.1;
                    let color = match (p0.0 as u8, p0.1 as u8) {
                        (0, 0) => colors[0][0].clone(),
                        (0, 1) => colors[0][1].clone(),
                        (1, 0) => colors[0][2].clone(),
                        (1, 1) => colors[0][3].clone(),
                        _ => BLACK.mix(0.5).clone(),
                    };
                    return EmptyElement::at(p1)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,Into::<ShapeStyle>::into(&color).filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", p0), (10, 0), ("Arial", 10).into_font());
                },
            ))
            .unwrap();

        // Sigmoid function plot
        let sigmoid = |x: f32| 1.0 / (1.0 + (-x).exp());
        (0..sig_charts.len()).for_each(|i| {
            let chart_sig = &mut sig_charts[i];

            chart_sig
                .draw_series(LineSeries::new(
                    (0..=1000).map(|i| {
                        let x = sig_x_min + (sig_x_max - sig_x_min) * (i as f32) / 1000.0;
                        (x, sigmoid(x))
                    }),
                    &BLUE,
                ))
                .unwrap();
            chart_sig
                .draw_series(PointSeries::of_element(
                    &sig1_series[series_index],
                    5,
                    &Palette99::pick(3).mix(0.9),
                    &|v, s, _st| {
                        let p0 = v.0;
                        let p1 = v.1;
                        let color = match (p0.0 as u8, p0.1 as u8) {
                            (0, 0) => colors[i][0].clone(),
                            (0, 1) => colors[i][1].clone(),
                            (1, 0) => colors[i][2].clone(),
                            (1, 1) => colors[i][3].clone(),
                            _ => BLACK.mix(0.5).clone(),
                        };
                        return EmptyElement::at(p1)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,Into::<ShapeStyle>::into(&color).filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", p0), (10, 0), ("Arial", 10).into_font());
                    },
                ))
                .unwrap();
        });

        // Plot output layer results
        // ax + by + c = 0
        let a = &all_layer_params[series_index][1].weight[[0, 0]];
        let b = &all_layer_params[series_index][1].weight[[0, 1]];
        let c = &all_layer_params[series_index][1].bias[[0, 0]];
        let mut chart_l1_out_sigmoid = ChartBuilder::on(&drawing_areas[series_index][2])
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .caption(
                format!(
                    "Compare Layer 1 outputs {}: ({a:.1})x + ({b:.1})y + ({c:.1}) = 0",
                    series_index + 1
                ),
                caption_small_font.clone(),
            )
            // Finally attach a coordinate on the drawing area and make a chart context
            .build_cartesian_2d(l1_x_min..l1_x_max, l1_y_min..l1_y_max)
            .unwrap();
        chart_l1_out_sigmoid
            .configure_mesh()
            // We can customize the maximum number of labels allowed for each axis
            .x_labels(5)
            .y_labels(5)
            // We can also change the format of the label text
            .y_label_formatter(&|x| format!("{:.1}", x))
            .x_desc("Layer 1 output-1 after Sigmoid")
            .y_desc("Layer 1 output-2 after Sigmoid")
            .draw()
            .unwrap();

        let step = if l1_x_max - l1_x_min < l1_y_max - l1_y_min {
            (l1_x_max - l1_x_min) / 20.0
        } else {
            (l1_y_max - l1_y_min) / 20.0
        };
        let x_axis = (l1_x_min..l1_x_max).step(step);
        chart_l1_out_sigmoid
            .draw_series(LineSeries::new(
                x_axis.values().map(|x| {
                    // ax + by + c = 0
                    let a = &all_layer_params[series_index][1].weight[[0, 0]];
                    let b = &all_layer_params[series_index][1].weight[[0, 1]];
                    let c = &all_layer_params[series_index][1].bias[[0, 0]];
                    let y = -(a * x + c) / b;
                    (x, y)
                }),
                &BLUE,
            ))
            .unwrap();
        chart_l1_out_sigmoid
            .draw_series(PointSeries::of_element(
                &l1_out_sig_series[series_index],
                5,
                &GREEN,
                &|v, s, _st| {
                    // Coordinate before transformation
                    let p0 = v.0;
                    // Coordinate after transformation
                    let p1 = v.1;
                    let color = match (p0.0 as u8, p0.1 as u8) {
                        (0, 0) => colors[1][0].clone(),
                        (0, 1) => colors[1][1].clone(),
                        (1, 0) => colors[1][2].clone(),
                        (1, 1) => colors[1][3].clone(),
                        _ => BLACK.mix(0.5).clone(),
                    };
                    return EmptyElement::at(p1)    // We want to construct a composed element on-the-fly
                + Circle::new((0,0),s,Into::<ShapeStyle>::into(&color).filled()) // At this point, the new pixel coordinate is established
                + Text::new(format!("{:?}", p0), (10, 0), ("Arial", 10).into_font());
                },
            ))
            .unwrap();

        // Draw sigmoid(w1*x + b0) + sigmoid(w2*x + b2)
        let (x_min, x_max): (f32, f32) = (-0.1, 1.1);
        let (y_min, y_max): (f32, f32) = (-0.1, 1.1);
        let (z_min, z_max): (f32, f32) = (-0.1, 1.1);
        let x_data_range = 0_f32..1_f32;
        let y_data_range = 0_f32..1_f32;
        let caption_style = plotters::style::TextStyle::from(caption_small_font.clone()).color(&BLACK);

        let (caption_area, drawing_area) =
            &drawing_areas[series_index][3].split_vertically(caption_small_height as u32);
        caption_area
            .titled("Predicted XOR Approximation: (0..1, 0..1)", &caption_style)
            .unwrap();

        let mut cc = ChartBuilder::on(&drawing_area)
            .margin_left(30)
            .margin_right(10)
            .margin_bottom(30)
            .build_cartesian_3d(x_min..x_max, y_min..y_max, z_min..z_max)
            .unwrap();

        cc.with_projection(|mut p| {
            p.pitch = 0.0 as f64;
            p.yaw = 0.0 as f64;
            p.scale = 1.2 as f64;
            p.into_matrix() // build the projection matrix
        });

        cc.configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .x_formatter(&|x| format!("x={x:.1}"))
            .y_formatter(&|y| format!("y={y:.1}"))
            .z_formatter(&|_z| "".to_string())
            .draw()
            .unwrap();

        let sigmoid = |x: f32| -> f32 { 1.0 / (1.0 + (-x).exp()) };
        cc.draw_series(
            SurfaceSeries::xoy(
                x_data_range.step(0.01).values(),
                y_data_range.step(0.01).values(),
                |x, y| {
                    let inputs = ndarray::arr2(&[[x as f32], [y as f32]]);
                    let w1_out = &all_layer_params[series_index][0].weight.dot(&inputs)
                        + &all_layer_params[series_index][0].bias;
                    let w1_out_sigmoid = w1_out.mapv(|v| sigmoid(v));
                    let w2_out = &all_layer_params[series_index][1]
                        .weight
                        .dot(&w1_out_sigmoid)
                        + &all_layer_params[series_index][1].bias;
                    w2_out[[0, 0]]
                },
            )
            .style_func(&|&v| (VulcanoHSL::get_color(v * 1.0)).into()),
        )
        .unwrap();

        let color_idx = 4;
        let line_color = Palette99::pick(color_idx).mix(0.9);
        let legend_color = line_color.clone();
        let (x_min, x_max): (f32, f32) = (-0.1, 1.1);
        let (y_min, y_max): (f32, f32) = (-4.1, 3.2);

        let caption_style = plotters::style::TextStyle::from(caption_small_font.clone()).color(&BLACK);

        let (caption_area, drawing_area) =
            &drawing_areas[series_index][4].split_vertically(caption_small_height as u32);
        caption_area
            .titled("Predicted XOR Approximation: x=y", &caption_style)
            .unwrap();

        let mut cc = ChartBuilder::on(&drawing_area)
            .margin(5)
            .margin_top(caption_small_height + 5)
            .margin_bottom(30)
            .set_label_area_size(LabelAreaPosition::Left, 30)
            .set_label_area_size(LabelAreaPosition::Bottom, 10)
            .set_label_area_size(LabelAreaPosition::Right, 40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();

        cc.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()
            .unwrap();

        let label_name = "predicted z";

        let sigmoid = |x: f32| -> f32 { 1.0 / (1.0 + (-x).exp()) };
        cc.draw_series(LineSeries::new(
            (x_min..x_max).step(0.01).values().map(|x| {
                let inputs = ndarray::arr2(&[[x as f32], [x as f32]]);
                let w1_out = &all_layer_params[series_index][0].weight.dot(&inputs)
                    + &all_layer_params[series_index][0].bias;
                let w1_out_sigmoid = w1_out.mapv(|v| sigmoid(v));
                let w2_out = &all_layer_params[series_index][1]
                    .weight
                    .dot(&w1_out_sigmoid)
                    + &all_layer_params[series_index][1].bias;
                (x, w2_out[[0, 0]])
            }),
            line_color.stroke_width(2),
        ))
        .unwrap()
        .label(label_name)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &legend_color));

        // Draw legend
        cc.configure_series_labels()
            .border_style(BLACK)
            .label_font(("Calibri", 20))
            .draw()
            .unwrap();
    });
    Ok(())
}

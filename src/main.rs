extern crate svg;
use svg::node::element;

extern crate rand;
use rand::Rng;

use noise::{NoiseFn, OpenSimplex};


fn polygon(start_x: i32, start_y: i32, size: f64, sides: i32, angle_offset: f64) -> element::path::Data {
    let mut data = element::path::Data::new()
        .move_to((start_x, start_y));

    for i in 0..sides {
        let angle = angle_offset + (std::f64::consts::PI + 2.0 * std::f64::consts::PI * i as f64 / sides as f64);
        let x = size * angle.sin();
        let y = size * angle.cos();

        data = data.line_by((x, y));
    }

    return data;
}

fn main() {
    let doc_size_x = 1920;
    let doc_size_y = 1080;

    let mut rng = rand::thread_rng();
    let noise = OpenSimplex::new();

    let mut paths = Vec::new();

    let offset_x = rng.gen_range(0.0, 1000000.0);
    let offset_y = rng.gen_range(0.0, 1000000.0);

    let back_size = rng.gen_range(5, 50);
    let back_sides = rng.gen_range(3, 10);

    let max_coords = doc_size_x + doc_size_y * doc_size_y / back_size;

    let color_multi = rng.gen_range(30.0, 180.0);

    for y in (0..doc_size_y).step_by(back_size) {
        for x in (0..doc_size_x).step_by(back_size) {
            print!("\rDrawing background {:.2}% ({}/{})", (x as f64 + y as f64 * doc_size_y as f64 / back_size as f64) / max_coords as f64 * 100.0, x + y * doc_size_y / back_size, max_coords);

            //let data = polygon(x, y, 5.0 + 1.0, 4);
            let data = polygon(x as i32, y as i32, back_size as f64 + 1.0, back_sides, rng.gen_range(-360.0, 360.0));

            let color_h = (noise.get([(offset_x + x as f64) * 0.001, (offset_y + y as f64) * 0.001]) + 1.0) * color_multi; //25.0;
            let color_s = 50;
            let color_l = 50;

            let color_str = format!("hsl({}, {}%, {}%)", color_h, color_s, color_l);

            let path = element::Path::new()
                .set("fill", color_str)
                .set("stroke", "none")
                .set("stroke-width", 0)
                .set("d", data);

            paths.push(path);
        }
    }

    print!("\rDrawing background {}% ({}/{})\n", 100, max_coords, max_coords);

    let polygons = rng.gen_range(0, 100);

    for i in 0..polygons {
        print!("\rPlacing polygons {:.2}% ({}/{})", (i + 1) / polygons * 100, i + 1, polygons);

        let start_x = rng.gen_range(10, doc_size_x - 10) as i32;
        let start_y = rng.gen_range(10, doc_size_y - 10) as i32;

        let size = rng.gen_range(1.0, 50.0);

        let color_h = rng.gen_range(0, 358);
        let color_s = rng.gen_range(30, 70);
        let color_l = rng.gen_range(30, 70);

        let color_str = format!("hsl({}, {}%, {}%)", color_h, color_s, color_l);

        let sides = rng.gen_range(3, 20);

        let data = polygon(start_x, start_y, size, sides, 0.0);

        let path = element::Path::new()
            .set("fill", color_str)
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data);

        paths.push(path);
    }

    print!("\rPlacing polygons {}% ({}/{})\n", 100, polygons, polygons);

    print!("Generating document (Step 1/3)");

    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, 1920, 1080));

    print!("\rGenerating document (Step 2/3)");

    for p in paths {
        document = document.add(p);
    }

    print!("\rGenerating document (Step 3/3)");

    svg::save("out.svg", &document);

    let save_string: String = rng.sample_iter(&rand::distributions::Alphanumeric)
        .take(8)
        .collect();

    svg::save(format!("out/{}.svg", save_string), &document);
}

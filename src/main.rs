use std::fs::File;
use std::io;
use std::io::Write;

use dla::{Dla, Vec3};

fn main() -> io::Result<()> {
    // let iterations = 10_000_000;
    let iterations = 1_000;
    let c: Rgb = [0.1, 0.3, 0.1];
    let spawn_offset = 10;

    let seeds = vec![Vec3::new(0, 0, 0)];

    // cross
    // for i in 1..=100 {
    //     seeds.extend_from_slice(&[
    //         Vec3::new(i, 0, 0),
    //         Vec3::new(0, i, 0),
    //         Vec3::new(0, 0, i),
    //         Vec3::new(-i, 0, 0),
    //         Vec3::new(0, -i, 0),
    //         Vec3::new(0, 0, -i),
    //     ]);
    // }

    let mut dla = Dla::new(spawn_offset, seeds).unwrap();

    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        dla.add(&mut rng);
    }

    let mut out = File::create("dla.pov")?;

    let scene_bbox = dla.bbox();
    let away_dist = (scene_bbox.lower().norm2() as f64).sqrt() as i64;
    let camera_pos = Vec3::new(0, 0, scene_bbox.lower().z - away_dist);

    writeln!(
        out,
        r#"
#version 3.7;

#include "colors.inc"

global_settings {{ assumed_gamma 1.0 }}
#default{{ finish {{ ambient 0.1 diffuse 0.9 }} }}

background {{ color Black }}

// scene bbox <{}, {}, {}> <{}, {}, {}>

camera {{
  location <{}, {}, {}>
  look_at <0, 0, 0>
}}
"#,
        scene_bbox.lower().x,
        scene_bbox.lower().y,
        scene_bbox.lower().z,
        scene_bbox.upper().x,
        scene_bbox.upper().y,
        scene_bbox.upper().z,
        camera_pos.x,
        camera_pos.y,
        camera_pos.z,
    )?;

    writeln!(
        out,
        "light_source {{ <{}, {}, {}> color White}}",
        (scene_bbox.lower() - away_dist).x,
        (scene_bbox.lower() - away_dist).y,
        camera_pos.z
    )?;
    writeln!(
        out,
        "light_source {{ <{}, {}, {}> color White}}",
        (scene_bbox.upper() + away_dist).x,
        (scene_bbox.upper() + away_dist).y,
        camera_pos.z
    )?;

    writeln!(out, "\nunion {{")?;
    for p in dla.cells() {
        writeln!(out, "  sphere {{ <{}, {}, {}>, 1 }}", p.x, p.y, p.z)?;
    }

    writeln!(
        out,
        r#"  texture {{
    pigment {{ color rgb<{}, {}, {}> }}
    finish {{ phong 0.5 }}
  }}
}}"#,
        c[0], c[1], c[2]
    )?;

    println!(
        r#"The DLA system was correctly generated. It contains {} particles.

The final state of the system has been saved as a PovRay scene (dla.pov) which
is possible to render with a povray invocation like the following

`povray +A +W1600 +H1200 dla.pov`
"#,
        dla.len()
    );

    Ok(())
}

pub type Rgb = [f64; 3];
pub fn lerp_rgb(a: Rgb, b: Rgb, t: f64) -> Rgb {
    [
        a[0] + (b[0] - a[0]) * t,
        a[1] + (b[1] - a[1]) * t,
        a[2] + (b[2] - a[2]) * t,
    ]
}

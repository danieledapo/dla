use std::fs::File;
use std::io;
use std::io::Write;

use dla::{Dla, Vec3};

fn main() -> io::Result<()> {
    let mut seeds = vec![Vec3::new(0, 0, 0)];

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

    let mut dla = Dla::new(seeds).unwrap();

    let mut rng = rand::thread_rng();
    // for _ in 0..10_000_000 {
    for _ in 0..100_000 {
        dla.add(&mut rng);
    }

    dbg!(dla.len());

    let mut out = File::create("dla.pov")?;

    let scene_bbox = dla.bbox();
    let camera_pos = Vec3::new(0, 0, scene_bbox.lower().z - 100);

    writeln!(
        out,
        r#"
#version 3.7;

#include "colors.inc"

global_settings {{ assumed_gamma 1.0 }}

#default{{ finish {{ ambient 0.1 diffuse 0.9 }} }}

// background {{ color Black }}

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
        (scene_bbox.lower() - 20).x,
        (scene_bbox.lower() - 20).y,
        (scene_bbox.lower() - 20).z
    )?;
    writeln!(
        out,
        "light_source {{ <{}, {}, {}> color White}}",
        (scene_bbox.lower() - 20).x,
        (scene_bbox.lower() - 20).y,
        (scene_bbox.upper() + 20).z
    )?;
    writeln!(
        out,
        "light_source {{ <{}, {}, {}> color White}}",
        (scene_bbox.upper() + 20).x,
        (scene_bbox.upper() + 20).y,
        (scene_bbox.upper() + 20).z
    )?;
    writeln!(
        out,
        "light_source {{ <{}, {}, {}> color White}}",
        (scene_bbox.upper() + 20).x,
        (scene_bbox.upper() + 20).y,
        (scene_bbox.lower() - 20).z
    )?;

    writeln!(out, "\nunion {{")?;
    for p in dla.cells() {
        writeln!(out, "  sphere {{ <{}, {}, {}>, 1 }}", p.x, p.y, p.z)?;
    }
    writeln!(
        out,
        r#"  texture {{
    pigment {{ color rgb<1,0.65,0> }}
    finish {{ phong 0.5 }}
  }}
}}"#
    )?;

    Ok(())
}

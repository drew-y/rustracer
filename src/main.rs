mod vec3;
use vec3::{ Vec3 };

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {} \n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3 {
                x: f64::from(i) / f64::from(nx),
                y: f64::from(j) / f64::from(ny),
                z: 0.2
            };
            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;
            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}

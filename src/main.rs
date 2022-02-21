mod cube;
mod sphere;

fn main() {
    let cube = cube::Cube::construct(5.0);
    println!("{cube:?}");
    let sphere = sphere::Sphere::construct(5.0);
    println!(
        "{sphere:?}, radius computed from volume: {},radius computed from surface: {}",
        sphere.get_radius_from_volume(),
        sphere.get_radius_from_surface()
    )
}

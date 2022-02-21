mod cone;
mod cube;
mod cylinder;
mod round;
mod sphere;
mod square_pyramid;
mod theorem_of_pythagoras;

fn main() {
    let cube = cube::Cube::construct(5.0);
    println!("{cube:?}");
    let sphere = sphere::Sphere::construct(5.0);
    println!(
        "{sphere:?}, radius computed from volume: {},radius computed from surface: {}",
        sphere.get_radius_from_volume(),
        sphere.get_radius_from_surface()
    );
    let cylinder = cylinder::Cylinder::construct(5.0, 5.0);
    println!("{cylinder:?}");
    let cone = cone::Cone::construct(5.0, 5.0);
    println!("{cone:?}");
    let square_pyramid = square_pyramid::Pyramid::construct(5.0, 5.0);
    println!("{square_pyramid:?}");
    let right_triangle = theorem_of_pythagoras::RightTriangle::construct_with_hypothenuse(5.0, 3.0);
    println!("{right_triangle:?}");
}

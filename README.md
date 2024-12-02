**Ray Tracing Project Documentation**

> Overview

In this project, you will implement a basic Ray Tracer, a method used to render 3D scenes into 2D images by simulating the path of light. The ray tracer will be able to render simple objects (spheres, cubes, planes, cylinders) with lighting and shadows. You can view the rendered images from different angles by adjusting the camera.

> Features

1. Basic Objects: Supports rendering of spheres, cubes, planes, and cylinders.
2. Camera Control: Move and rotate the camera to view the scene from different perspectives.
3. Lighting: Includes basic light management with shadows and brightness adjustments.
4. Image Output: Images are generated in .ppm format (800x600 resolution).
5. Bonus: Support for reflections, refractions, textures, and particles.

> Code Usage

To use the ray tracer, you need to:

1. Create a New Object

```rust
Copier le code
let sphere = Sphere::new(center, radius);
let cube = Cube::new(origin, size);
let plane = Plane::new(point, normal);
let cylinder = Cylinder::new(base_center, height, radius);
```

2. Adjust Brightness

```rust
Copier le code
scene.set_light_brightness(0.7);
```

3. Change Camera Position

```rust
Copier le code
camera.set_position(1.0, 2.0, -5.0);
camera.set_angle(45.0);
```

4. Adjust the texture 

```rust
pub const MIRROR_MATERIAL: Material = Material {
    albedo: Vec3::new(0.9, 0.9, 0.9),  // Couleur gris argenté
    fuzziness: 0.0,                    // Pas de flou
    reflectivity: 1.0,                 // Réflexion parfaite
};
```

5. Render the Scene Run the following command to generate the image:

```bash
Copier le code
cargo run > output.ppm
```

* Image Format Example (.ppm)

```plaintext
Copier le code
P3
800 600
255
0 0 0 255 0 255 ...
```

The format specifies the pixel data for the rendered image, starting with the header, which includes the image type (P3), width and height, and maximum color value (255).

> Bonus Features

You can enhance the ray tracer with these additional features:

* Reflections: Make objects reflect light.
* Textures: Apply textures to objects.
* Particles: Add particle effects to your scene.

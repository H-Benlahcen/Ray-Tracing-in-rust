use std::fs::File;
use std::io::Write;
use solo_rt::{Vec3, Ray, Hittable,Sphere,Cube,Flat,Cylinder, calculate_color_with_material, camera_rot, MIRROR_MATERIAL};

// Fonction pour calculer la lumière diffusée selon le modèle Lambertien
fn main() {
    // Définir la position de la source lumineuse
    let light_pos = Vec3::new(-5.0, 5.0, -3.0);  // Lumière située en haut à gauche

    // Liste des objets dans la scène
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Cylinder::new(Vec3::new(-1.5, 0.0, -5.1), 0.5, 1.5)));
    objects.push(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -7.0), 0.5)));
    objects.push(Box::new(Cube::new(Vec3::new(1.5, -0.5, -5.0), Vec3::new(2.5, 0.5, -4.0),MIRROR_MATERIAL)));
    objects.push(Box::new(Flat::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0))));
  

    generate_image("allobject.ppm", &objects, light_pos, Vec3::new(1.0, 1.0, 0.0));
    generate_image("sphere.ppm", &[Box::new(Sphere::new(Vec3::new(0.0, 0.0, -7.0), 0.5))], light_pos, Vec3::new(1.0, 1.0, 0.0));
    generate_image("flat&cube&lowbright.ppm", &[Box::new(Cube::new(Vec3::new(1.5, -0.5, -5.0), Vec3::new(2.5, 0.5, -4.0), MIRROR_MATERIAL)), Box::new(Flat::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)))], light_pos, Vec3::new(1.0, 1.0, 0.0));
    generate_image("allobject&othercam.ppm", &objects, light_pos, Vec3::new(-2.0, 1.0, 0.0));
}

fn generate_image(filename: &str, objects: &[Box<dyn Hittable>], light_pos: Vec3, camera_pos: Vec3) {
    let mut pixels = Vec::with_capacity(1600 * 1200); // Augmenter la capacité pour plus de pixels

    // Définir les angles de rotation en radians pour les axes X, Y, Z
    let angle_x = 0.0_f32.to_radians(); // rotation X
    let mut angle_y  = 0.0_f32.to_radians(); // rotation Y
    if filename == "allobject&othercam.ppm" {
        angle_y = -30.0_f32.to_radians();
    }
    let angle_z = 0.0_f32.to_radians(); // rotation Z

    // Génération de l'image avec la haute résolution
    for j in (0..1200).rev() {
        for i in 0..1600 {
            // Calcul de la direction initiale (sans rotation)
            let dir = Vec3::new(
                (i as f32 - (1600 as f32 / 2.0)) / (1600 as f32 / 2.0),
                (j as f32 - (1200 as f32 / 2.0)) / (1200 as f32 / 2.0),
                -1.0,
            );
            
            // Appliquer la rotation en utilisant la fonction camera_rot
            let rotated_dir = camera_rot(dir, angle_x, angle_y, angle_z);
            
            // Créer le rayon avec la direction rotatée
            let ray = Ray::new(camera_pos, rotated_dir);
            
            // Calculer la couleur pour ce rayon et l'ajouter aux pixels
            pixels.push(calculate_color_with_material(ray, objects, light_pos));
        }
    }
    
    // Redimensionner pour une image 800x600
    let downscaled_pixels = downscale(pixels, 1600, 1200, 800, 600);
    
    // Écrire l'image redimensionnée
    print_ppm(filename, &downscaled_pixels);
}

// Function to write pixels to a PPM file
pub fn print_ppm(filename: &str, pixels: &Vec<(u8, u8, u8)>) {
    let mut file = File::create(filename).expect("Unable to create file");
    writeln!(file, "P3").unwrap();
    writeln!(file, "800 600").unwrap();  // Taille finale fixée à 800x600
    if filename == "flat&cube&lowbright.ppm"{writeln!(file, "500").unwrap();} else {writeln!(file, "255").unwrap();}
    for &(r, g, b) in pixels {
        writeln!(file, "{} {} {}", r, g, b).unwrap();
    }
}
fn downscale(pixels: Vec<(u8, u8, u8)>, src_width: usize, src_height: usize, dest_width: usize, dest_height: usize) -> Vec<(u8, u8, u8)> {
    let mut downscaled_pixels = Vec::with_capacity(dest_width * dest_height);
    
    let x_ratio = src_width as f32 / dest_width as f32;
    let y_ratio = src_height as f32 / dest_height as f32;

    for j in 0..dest_height {
        for i in 0..dest_width {
            let src_x = (i as f32 * x_ratio).floor() as usize;
            let src_y = (j as f32 * y_ratio).floor() as usize;
            
            let pixel = pixels[src_y * src_width + src_x];
            downscaled_pixels.push(pixel);
        }
    }
    
    downscaled_pixels
}
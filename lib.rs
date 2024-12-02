use std::ops::{Sub, Mul, Add};
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(*self)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            Vec3::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vec3::new(0.0, 0.0, 0.0)  // Retourne un vecteur nul si la longueur est zéro
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            // Génère un vecteur aléatoire dans le cube [-1,1] pour chaque composante
            let p = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            );
            // Si le vecteur est dans la sphère unitaire, on le retourne
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

// Implémentation du trait Sub pour Vec3
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<'a> Sub for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<'a> Add for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Implémentation du trait Mul pour Vec3 (pour la multiplication par un scalaire)
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

// Implémentation du trait Mul pour les références à Vec3
impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3)>;
    fn color(&self) -> (u8, u8, u8);
    fn material(&self) -> Option<&Material>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3)> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                let hit_point = ray.origin + ray.direction * t;
                let normal = (hit_point - self.center).normalize();
                return Some((t, normal));
            }
        }
        None
    }

    fn color(&self) -> (u8, u8, u8) {
        (255, 0, 0)  // Rouge pour la sphère
    }

    fn material(&self) -> Option<&Material> {
        None
    }
}

pub struct Cube {
    min: Vec3,
    max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, material: Material) -> Cube {
        Cube { min, max, material }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3)> {
        let inv_dir = Vec3::new(1.0 / ray.direction.x, 1.0 / ray.direction.y, 1.0 / ray.direction.z);

        let t1 = (self.min.x - ray.origin.x) * inv_dir.x;
        let t2 = (self.max.x - ray.origin.x) * inv_dir.x;
        let t3 = (self.min.y - ray.origin.y) * inv_dir.y;
        let t4 = (self.max.y - ray.origin.y) * inv_dir.y;
        let t5 = (self.min.z - ray.origin.z) * inv_dir.z;
        let t6 = (self.max.z - ray.origin.z) * inv_dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax >= tmin && tmin >= t_min && tmax <= t_max {
            let mut normal = Vec3::new(0.0, 0.0, 0.0);
            if tmin == t1 {
                normal = Vec3::new(-1.0, 0.0, 0.0);
            } else if tmin == t2 {
                normal = Vec3::new(1.0, 0.0, 0.0);
            } else if tmin == t3 {
                normal = Vec3::new(0.0, -1.0, 0.0);
            } else if tmin == t4 {
                normal = Vec3::new(0.0, 1.0, 0.0);
            } else if tmin == t5 {
                normal = Vec3::new(0.0, 0.0, -1.0);
            } else if tmin == t6 {
                normal = Vec3::new(0.0, 0.0, 1.0);
            }
            return Some((tmin, normal));
        }
        None
    }

    fn color(&self) -> (u8, u8, u8) {
        (0, 0, 255)
    }

    fn material(&self) -> Option<&Material> {
        Some(&MIRROR_MATERIAL)
    }
}

pub struct Flat {
    origin: Vec3,
    normal: Vec3,
}

impl Flat {
    pub fn new(origin: Vec3, normal: Vec3) -> Flat {
        Flat { origin, normal }
    }
}

impl Hittable for Flat {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3)> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() > 1e-6 {
            let t = (self.origin - ray.origin).dot(self.normal) / denom;
            if t >= t_min && t <= t_max {
                return Some((t, self.normal));
            }
        }
        None
    }

    fn color(&self) -> (u8, u8, u8) {
        (0, 255, 0)  // Vert pour le plan
    }

    fn material(&self) -> Option<&Material> {
        None
    }
}

pub struct Cylinder {
    center: Vec3,
    radius: f32,
    height: f32,
}

impl Cylinder {
    pub fn new(center: Vec3, radius: f32, height: f32) -> Cylinder {
        Cylinder { center, radius, height }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(f32, Vec3)> {
        let oc = ray.origin - self.center;
        let a = ray.direction.x * ray.direction.x + ray.direction.y * ray.direction.y;
        let b = 2.0 * (oc.x * ray.direction.x + oc.y * ray.direction.y);
        let c = oc.x * oc.x + oc.y * oc.y - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        for &t in &[t0, t1] {
            if t > t_min && t < t_max {
                let hit_point = ray.origin + ray.direction * t;
                if hit_point.z >= self.center.z - self.height / 2.0 && hit_point.z <= self.center.z + self.height / 2.0 {
                    let normal = Vec3::new(hit_point.x - self.center.x, hit_point.y - self.center.y, 0.0).normalize();
                    return Some((t, normal));
                }
            }
        }

        None
    }

    fn color(&self) -> (u8, u8, u8) {
        (255, 0, 255)
    }

    fn material(&self) -> Option<&Material> {
        None
    }
}

pub fn is_in_shadow(point: Vec3, light_pos: Vec3, objects: &[Box<dyn Hittable>]) -> bool {
    let light_dir = (light_pos - point).normalize();
    let light_ray = Ray::new(point + light_dir * 0.001, light_dir);

    for obj in objects {
        if obj.hit(&light_ray, 0.001, f32::MAX).is_some() {
            return true;
        }
    }
    false
}

pub fn calculate_color(ray: Ray, objects: &[Box<dyn Hittable>], light_pos: Vec3) -> (u8, u8, u8) {
    for obj in objects {
        if let Some((t, normal)) = obj.hit(&ray, 0.001, f32::MAX) {
            let hit_point = ray.origin + ray.direction * t;

            if is_in_shadow(hit_point, light_pos, objects) {
                return (0, 0, 0);
            }

            let light_dir = (light_pos - hit_point).normalize();
            let intensity = lambertian_lighting(normal, light_dir);
            let (r, g, b) = obj.color();
            let r = (r as f32 * intensity).min(255.0) as u8;
            let g = (g as f32 * intensity).min(255.0) as u8;
            let b = (b as f32 * intensity).min(255.0) as u8;
            return (r, g, b);
        }
    }
    (0, 0, 0)
}

pub struct Material {
    albedo: Vec3,       // Couleur de base (gris argenté pour le chrome)
    fuzziness: f32,     // Niveau de flou dans la réflexion (0.0 = parfait, >0.0 = légèrement flou)
    reflectivity: f32,  // Niveau de réflexion (1.0 pour du chrome parfait)
}
pub const MIRROR_MATERIAL: Material = Material {
    albedo: Vec3::new(0.9, 0.9, 0.9),  // Couleur gris argenté
    fuzziness: 0.0,                    // Pas de flou
    reflectivity: 1.0,                 // Réflexion parfaite
};
impl Material {
    fn scatter(&self, ray_in: &Ray, hit_point: &Vec3, normal: &Vec3) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray_in.direction.normalize(), *normal);
        let scattered_direction = reflected + Vec3::random_in_unit_sphere() * self.fuzziness;

        // Vérifie si le rayon dispersé est toujours dans l'hémisphère du normal
        if scattered_direction.dot(*normal) > 0.0 {
            let scattered = Ray {
                origin: *hit_point,
                direction: scattered_direction.normalize(),
            };

            // Retourne le rayon dispersé et applique l'albedo (couleur du matériau)
            let attenuation = self.albedo * self.reflectivity;  // Atténuation par reflectivity et albedo

            // Renvoie le rayon dispersé et l'atténuation (qui correspond à la couleur)
            return Some((scattered, attenuation));
        }
        None
    }
}

// Fonction de réflexion
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * (2.0 * v.dot(n))
}

pub fn calculate_color_with_material(ray: Ray, objects: &[Box<dyn Hittable>], light_pos: Vec3) -> (u8, u8, u8) {
    for obj in objects {
        if let Some((t, normal)) = obj.hit(&ray, 0.001, f32::MAX) {
            let hit_point = ray.origin + ray.direction * t;

            // Check for materials and scatter rays accordingly
            if let Some(material) = obj.material() {  // Assuming you have a material method
                if let Some((scattered_ray, _)) = material.scatter(&ray, &hit_point, &normal) {
                    return calculate_color(scattered_ray, objects, light_pos);
                }
            }

            if is_in_shadow(hit_point, light_pos, objects) {
                return (0, 0, 0);
            }

            let light_dir = (light_pos - hit_point).normalize();
            let intensity = lambertian_lighting(normal, light_dir);
            let (r, g, b) = obj.color();
            let r = (r as f32 * intensity).min(255.0) as u8;
            let g = (g as f32 * intensity).min(255.0) as u8;
            let b = (b as f32 * intensity).min(255.0) as u8;
            return (r, g, b);
        }
    }
    (0, 0, 0)
}

fn lambertian_lighting(normal: Vec3, light_dir: Vec3) -> f32 {
    let intensity = normal.dot(light_dir);
    intensity.max(0.025)
}
pub fn camera_rot(rot: Vec3, x: f32, y: f32, z: f32) -> Vec3 {
    let (cos_x, sin_x) = (x.cos(), x.sin());
    let (cos_y, sin_y) = (y.cos(), y.sin());
    let (cos_z, sin_z) = (z.cos(), z.sin());

    // Appliquer la rotation autour de l'axe X
    let rotated_x = Vec3::new(
        rot.x,
        rot.y * cos_x - rot.z * sin_x,
        rot.y * sin_x + rot.z * cos_x,
    );

    // Appliquer la rotation autour de l'axe Y
    let rotated_y = Vec3::new(
        rotated_x.x * cos_y + rotated_x.z * sin_y,
        rotated_x.y,
        -rotated_x.x * sin_y + rotated_x.z * cos_y,
    );

    // Appliquer la rotation autour de l'axe Z
    Vec3::new(
        rotated_y.x * cos_z - rotated_y.y * sin_z,
        rotated_y.x * sin_z + rotated_y.y * cos_z,
        rotated_y.z,
    )
}

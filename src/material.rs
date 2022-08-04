use crate::prelude::*;
use rand::Rng;

pub struct HitBounce {
    pub ray: Ray,
    pub attenuation: ColorRGB,
}

pub trait Scatter {
    fn scatter(ray: &Ray, hit: &HitRecord) -> Option<HitBounce>;
}

pub enum Material {
    None,
    Lambertan(ColorRGB),
    Metal(ColorRGB, f32),
    Dielectric(f32),
}

impl Scatter for Material {
    fn scatter(ray: &Ray, hit: &HitRecord) -> Option<HitBounce> {
        match *hit.material {
            Material::None => Some(HitBounce {
                ray: Ray::new(hit.point, hit.normal),
                attenuation: ColorRGB::new(0.5, 0.5, 0.5),
            }),
            Material::Lambertan(color) => {
                let mut scatter_dir = hit.normal + Vector3D::random(-1.0, 1.0).unit();
                if scatter_dir.is_near_zero() {
                    scatter_dir = hit.normal;
                }
                Some(HitBounce {
                    ray: Ray::new(hit.point, scatter_dir),
                    attenuation: color.clone(),
                })
            }
            Material::Metal(color, fuzz) => {
                let reflected_dir = reflect(&ray.direction, &hit.normal);
                let fuzzy_reflected_dir = reflected_dir + Vector3D::unit_sphere_sample() * fuzz;
                let attenuation = color.clone();
                if fuzzy_reflected_dir.dot(&hit.normal).abs() < 10e-8 {
                    return None;
                }
                Some(HitBounce {
                    ray: Ray::new(hit.point, fuzzy_reflected_dir),
                    attenuation,
                })
            }
            Material::Dielectric(refraction_index) => {
                let mut ri = refraction_index;
                if hit.is_front_face {
                    ri = 1.0 / ri;
                }

                let cos_theta = (-ray.direction).dot(&hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;
                let angle_too_steep =
                    shlick_reflectance(cos_theta, ri) > rand::thread_rng().gen_range(0.0..1.0);

                let direction = if cannot_refract || angle_too_steep {
                    reflect(&ray.direction, &hit.normal)
                } else {
                    refract(&ray.direction, &hit.normal, ri)
                };
                Some(HitBounce {
                    ray: Ray::new(hit.point, direction),
                    attenuation: ColorRGB::new(1.0, 1.0, 1.0),
                })
            }
        }
    }
}

// normal vec is unit
pub fn reflect(vec: &Vector3D, normal: &Vector3D) -> Vector3D {
    vec.clone() - normal.clone() * vec.dot(normal) * 2.0
}

pub fn refract(vec: &Vector3D, normal: &Vector3D, refraction_index: f32) -> Vector3D {
    let vec = vec.clone();
    let normal = normal.clone();
    let cos_theta_incoming = (-vec).dot(&normal).min(1.0);

    let r_out_perp = refraction_index * (vec + cos_theta_incoming * normal);
    let r_out_para = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;
    r_out_perp + r_out_para
}

#[inline]
pub fn shlick_reflectance(cos_theta: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 * r0 + (1.0 - r0 * r0) * (1.0 - cos_theta).powi(5)
}

use crate::prelude::*;
use rand::Rng;

/// Structure describing reflected / refracted ray
///
pub struct HitBounce {
    pub ray: Ray,
    pub attenuation: ColorRGB,
}

/// trait for all materials that can produce a HitBounce
/// on their body intersection with a ray
///
pub trait Scatter {
    fn scatter(ray: &Ray, hit: &HitRecord) -> Option<HitBounce>;
}

/// Enumeration of basic mateirials
///
pub enum Material {
    /// 100% reflects normal with gray color
    None,
    /// 100% reflects fully diffuse ray with albedo color
    Lambertan(ColorRGB),
    /// 100% reflects normal + in a fuzzyway ray with albedo color
    Metal(ColorRGB, f32),
    /// depending on the cangle can relfect and refract
    Dielectric(f32),
}

impl Scatter for Material {
    fn scatter(ray: &Ray, hit: &HitRecord) -> Option<HitBounce> {
        match hit.material {
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
                    attenuation: *color,
                })
            }
            Material::Metal(color, fuzz) => {
                let reflected_dir = reflect(&ray.direction, &hit.normal);
                let fuzzy_reflected_dir = reflected_dir + Vector3D::unit_sphere_sample() * *fuzz;
                if fuzzy_reflected_dir.dot(&hit.normal).abs() < 10e-8 {
                    return None;
                }
                Some(HitBounce {
                    ray: Ray::new(hit.point, fuzzy_reflected_dir),
                    attenuation: *color,
                })
            }
            Material::Dielectric(refraction_index) => {
                let mut ri = *refraction_index;
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

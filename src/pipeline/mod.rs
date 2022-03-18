pub mod barycentric;
pub mod frame;
pub mod shaders;

use num_traits::abs;

use crate::matrix::vector::*;

use self::{barycentric::BarycentricCoord, frame::Frame, shaders::ShaderProgram};

pub fn draw<const N: usize, const M: usize>(
    frame: &mut Frame<f32, N, M>,
    shader: Box<dyn ShaderProgram>,
    vertices: &[Vector3],
) {
    // run vertex shader
    let mut vertex_positions = Vec::from(vertices.clone());
    for v in vertex_positions.iter_mut() {
        *v = shader.vert(v);
    }

    let tris = assemble_triangles(&vertex_positions);

    let ndc_box = BoundingBox {
        x: -1f32,
        y: -1f32,
        z: -1f32,
        xmax: 1f32,
        ymax: 1f32,
        zmax: 1f32,
    };

    let pixel_dist_h = 1f32 / M as f32;
    let pixel_half_dist_h = pixel_dist_h / 2f32;
    let pixel_dist_v = 1f32 / N as f32;
    let pixel_half_dist_v = pixel_dist_v / 2f32;

    let get_screen_coord = |x: f32, y: f32| -> (usize, usize) {
        (
            (((x + 1f32) / 2f32 + pixel_half_dist_h) / pixel_dist_h) as usize,
            (((y + 1f32) / 2f32 + pixel_half_dist_v) / pixel_dist_v) as usize,
        )
    };

    for t in tris {
        let b = t.calc_bounding_box().intersect(&ndc_box);

        let (startx, starty) = get_screen_coord(b.x, b.y);
        let (endx, endy) = get_screen_coord(b.xmax, b.ymax); // indices after last pixel
        for y in starty..endy {
            for x in startx..endx {
                let pos = Vector2::new2(
                    (x as f32 * pixel_dist_h + pixel_half_dist_h) * 2f32 - 1f32,
                    (y as f32 * pixel_dist_v + pixel_half_dist_v) * 2f32 - 1f32,
                );
                if t.contains_point_2d(&pos) {
                    // run fragment shader
                    let coord = t.calc_barycentric_coord(pos);
                    frame.color[y][x] = shader.frag(&coord);
                }
            }
        }
    }
}

fn assemble_triangles(vertices: &[Vector3]) -> Vec<Triangle> {
    let mut tris = Vec::new();
    for i in (0..vertices.len()).step_by(3) {
        tris.push(Triangle {
            verts: [&vertices[i], &vertices[i + 1], &vertices[i + 2]],
        })
    }

    tris
}

struct Triangle<'a> {
    verts: [&'a Vector3; 3],
}

impl<'a> Triangle<'a> {
    fn calc_bounding_box(&self) -> BoundingBox {
        let mut b = BoundingBox {
            x: f32::MAX,
            y: f32::MAX,
            z: f32::MAX,
            xmax: f32::MIN,
            ymax: f32::MIN,
            zmax: f32::MIN,
        };

        for v in &self.verts {
            if b.x > *v.x() {
                b.x = *v.x();
            }
            if b.y > *v.y() {
                b.y = *v.y();
            }
            if b.z > *v.z() {
                b.z = *v.z();
            }

            if b.xmax < *v.x() {
                b.xmax = *v.x();
            }
            if b.ymax < *v.y() {
                b.ymax = *v.y();
            }
            if b.zmax < *v.z() {
                b.zmax = *v.z();
            }
        }

        b
    }

    /// ignores z axis
    fn contains_point_2d(&self, p: &Vector2) -> bool {
        let va = Vector2::from_vector3(self.verts[0]);
        let vb = Vector2::from_vector3(self.verts[1]);
        let vc = Vector2::from_vector3(self.verts[2]);

        let dir1 = Self::turn(&va, &vb, p);
        let dir2 = Self::turn(&vb, &vc, p);
        let dir3 = Self::turn(&vc, &va, p);

        dir1 >= 0f32 && dir2 >= 0f32 && dir3 >= 0f32 || dir1 <= 0f32 && dir2 <= 0f32 && dir3 <= 0f32
    }

    fn calc_barycentric_coord(&self, p: Vector2) -> BarycentricCoord {
        // let abx = self.verts[1].x() - self.verts[0].x();
        // let aby = self.verts[1].y() - self.verts[0].y();
        // let acx = self.verts[2].x() - self.verts[0].x();
        // let acy = self.verts[2].y() - self.verts[0].y();

        // let c = (p.x() + (1f32 - p.y()) * abx / aby - 1f32) / (acx - acy * abx / aby);
        // let b = (p.y() - c * acy - 1f32) / aby;
        // let a = 1f32 - b - c;

        // BarycentricCoord::new(a, b, c)

        let vert_a = Vector2::from_vector3(self.verts[0]);
        let vert_b = Vector2::from_vector3(self.verts[1]);
        let vert_c = Vector2::from_vector3(self.verts[2]);

        let area_abp = Self::area(&vert_a, &vert_b, &p);
        let area_bcp = Self::area(&vert_b, &vert_c, &p);
        let area_cap = Self::area(&vert_c, &vert_a, &p);

        let area = Self::area(&vert_a, &vert_b, &vert_c);

        BarycentricCoord::new(area_bcp / area, area_cap / area, area_abp / area)
    }

    fn turn(a: &Vector2, b: &Vector2, c: &Vector2) -> f32 {
        (a.y() - b.y()) * (c.x() - b.x()) + (b.x() - a.x()) * (c.y() - b.y())
    }

    fn area(a: &Vector2, b: &Vector2, c: &Vector2) -> f32 {
        abs(a.x() * (b.y() - c.y()) + b.x() * (c.y() - a.y()) + c.x() * (a.y() - b.y())) / 2f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_turn() {
        let a = Vector2::new2(0f32, 0f32);
        let b = Vector2::new2(1f32, 2f32);
        let c = Vector2::new2(2f32, 1f32);

        assert!(Triangle::turn(&a, &b, &c) * Triangle::turn(&a, &c, &b) < 0f32);
    }
}

#[derive(Clone, Copy)]
struct BoundingBox {
    x: f32,
    y: f32,
    z: f32,
    xmax: f32,
    ymax: f32,
    zmax: f32,
}

impl BoundingBox {
    fn intersect(&self, other: &BoundingBox) -> BoundingBox {
        let mut res = *other;
        if self.x > res.x {
            res.x = self.x;
        }
        if self.y > res.y {
            res.y = self.y;
        }
        if self.z > res.z {
            res.z = self.z;
        }

        if self.xmax < res.xmax {
            res.xmax = self.xmax;
        }
        if self.ymax < res.ymax {
            res.ymax = self.ymax;
        }
        if self.zmax < res.zmax {
            res.zmax = self.zmax;
        }

        res
    }
}

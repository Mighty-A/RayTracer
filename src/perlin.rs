use crate::rtweekend::*;
use crate::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ran_vec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    fn permute(tmp: &mut [i32; POINT_COUNT], n: i32) {
        for k in 0..n - 1 {
            let i = (n - 1 - k) as usize;
            let target = random_int(0, i as i32) as usize; //[0, i]
            tmp.swap(i, target);
        }
    }
    fn perlin_generate_perm() -> [i32; POINT_COUNT] {
        let mut tmp = [0 as i32; POINT_COUNT];
        #[allow(clippy::needless_range_loop)]
        for i in 0..POINT_COUNT {
            tmp[i] = i as i32;
        }
        Perlin::permute(&mut tmp, POINT_COUNT as i32);

        tmp
    }
    pub fn new() -> Self {
        let mut tmp = [Vec3::zero(); POINT_COUNT];
        for i in tmp.iter_mut() {
            *i = Vec3::random(-1.0, 1.0).unit();
        }
        Self {
            ran_vec: tmp,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * (c[i][j][k] * weight_v);
                }
            }
        }
        accum
    }

    //#[allow(clippy::many_single_char_names)]
    pub fn noise(&self, p: &Point) -> f64 {
        let uu = p.x - p.x.floor();
        let vv = p.y - p.y.floor();
        let ww = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c_array = [[[Vec3::zero(); 2]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c_array[di][dj][dk] = self.ran_vec[(self.perm_x
                        [((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::perlin_interp(&c_array, uu, vv, ww)
    }

    pub fn turb(&self, p: &Point, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

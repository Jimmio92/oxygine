use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    write_vec2_swizzle(Path::new(&out_dir));
    write_vec3_swizzle(Path::new(&out_dir));
    write_vec4_swizzle(Path::new(&out_dir));
}

fn write_vec2_swizzle(path: &Path) {
    let axes = ['x', 'y'];

    let path = path.join("vec2_swizzle.rs");

    let mut f = File::create(&path).unwrap();

    write!(f, "/// Swizzle-like functions for generic Vec2 (GENERATED)\n").unwrap();
    write!(f, "impl<T> Vec2<T> where T: Copy {{\n").unwrap();

    // one
    for x in axes.iter() {
        write!(f, "\t/// Returns {}\n", x).unwrap();
        write!(f, "\tpub fn {}(&self) -> T {{\n", x).unwrap();
        write!(f, "\t\tself.{}\n", x).unwrap();
        write!(f, "\t}}\n").unwrap();
    }

    // two
    for x in axes.iter() {
        for y in axes.iter() {
            write!(f, "\t/// Returns new Vec2 with x set to `{}` and y set to `{}`\n", x, y).unwrap();
            write!(f, "\tpub fn {}{}(&self) -> Self {{\n", x, y).unwrap();
            write!(f, "\t\tVec2::from(self.{}, self.{})\n", x, y).unwrap();
            write!(f, "\t}}\n").unwrap();
        }
    }

    write!(f, "}}\n").unwrap();
}

fn write_vec3_swizzle(path: &Path) {
    let axes = ['x', 'y', 'z'];

    let path = path.join("vec3_swizzle.rs");

    let mut f = File::create(&path).unwrap();

    write!(f, "/// Swizzle-like functions for generic Vec3 (GENERATED)\n").unwrap();
    write!(f, "impl<T> Vec3<T> where T: Copy {{\n").unwrap();

    // one
    for x in axes.iter() {
        write!(f, "\t/// Returns {}\n", x).unwrap();
        write!(f, "\tpub fn {}(&self) -> T {{\n", x).unwrap();
        write!(f, "\t\tself.{}\n", x).unwrap();
        write!(f, "\t}}\n").unwrap();
    }

    // two
    for x in axes.iter() {
        for y in axes.iter() {
            write!(f, "\t/// Returns new Vec2 with x set to `{}` and y set to `{}`\n", x, y).unwrap();
            write!(f, "\tpub fn {}{}(&self) -> super::Vec2<T> {{\n", x, y).unwrap();
            write!(f, "\t\tsuper::Vec2::from(self.{}, self.{})\n", x, y).unwrap();
            write!(f, "\t}}\n").unwrap();
        }
    }

    // three
    for x in axes.iter() {
        for y in axes.iter() {
            for z in axes.iter() {
                write!(f, "\t/// Returns new Vec3 with x set to `{}`, y set to `{}` and z set to `{}`\n", x, y, z).unwrap();
                write!(f, "\tpub fn {}{}{}(&self) -> super::Vec3<T> {{\n", x, y, z).unwrap();
                write!(f, "\t\tsuper::Vec3::from(self.{}, self.{}, self.{})\n", x, y, z).unwrap();
                write!(f, "\t}}\n").unwrap();
            }
        }
    }

    write!(f, "}}\n").unwrap();
}

fn write_vec4_swizzle(path: &Path) {
    let axes = ['x', 'y', 'z', 'w'];

    let path = path.join("vec4_swizzle.rs");

    let mut f = File::create(&path).unwrap();

    write!(f, "/// Swizzle-like functions for generic Vec4 (GENERATED)\n").unwrap();
    write!(f, "impl<T> Vec4<T> where T: Copy {{\n").unwrap();

    // one
    for x in axes.iter() {
        write!(f, "\t/// Returns {}\n", x).unwrap();
        write!(f, "\tpub fn {}(&self) -> T {{\n", x).unwrap();
        write!(f, "\t\tself.{}\n", x).unwrap();
        write!(f, "\t}}\n").unwrap();
    }

    // two
    for x in axes.iter() {
        for y in axes.iter() {
            write!(f, "\t/// Returns new Vec2 with x set to `{}` and y set to `{}`\n", x, y).unwrap();
            write!(f, "\tpub fn {}{}(&self) -> super::Vec2<T> {{\n", x, y).unwrap();
            write!(f, "\t\tsuper::Vec2::from(self.{}, self.{})\n", x, y).unwrap();
            write!(f, "\t}}\n").unwrap();
        }
    }

    // three
    for x in axes.iter() {
        for y in axes.iter() {
            for z in axes.iter() {
                write!(f, "\t/// Returns new Vec3 with x set to `{}`, y set to `{}` and z set to `{}`\n", x, y, z).unwrap();
                write!(f, "\tpub fn {}{}{}(&self) -> super::Vec3<T> {{\n", x, y, z).unwrap();
                write!(f, "\t\tsuper::Vec3::from(self.{}, self.{}, self.{})\n", x, y, z).unwrap();
                write!(f, "\t}}\n").unwrap();
            }
        }
    }

    // four
    for x in axes.iter() {
        for y in axes.iter() {
            for z in axes.iter() {
                for w in axes.iter() {
                    write!(f, "\t/// Returns new Vec4 with x set to `{}`, y set to `{}`, z set to `{}` and w set to `{}`\n", x, y, z, w).unwrap();
                    write!(f, "\tpub fn {}{}{}{}(&self) -> Self {{\n", x, y, z, w).unwrap();
                    write!(f, "\t\tVec4::from(self.{}, self.{}, self.{}, self.{})\n", x, y, z, w).unwrap();
                    write!(f, "\t}}\n").unwrap();
                }
            }
        }
    }

    write!(f, "}}\n").unwrap();
}


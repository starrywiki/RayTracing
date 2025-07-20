// src/mesh.rs
use crate::aabb::Aabb;
use crate::bvh::BvhNode;
use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{DiffuseLight, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::rtw_image::RtwImage;
use crate::texture::{ImageTexture, SolidColor, Texture};
use crate::triangle::Triangle;
use crate::vec3::{cross, dot, Point3, Vec3};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
pub struct Mesh {
    bvh_root: Arc<dyn Hittable + Send + Sync>,
    bbox: Aabb,
}
impl Mesh {
    pub fn new(file_path: &str) -> Self {
        eprintln!("Loading GLTF model with external textures: {}", file_path);

        // 获取模型文件所在的目录
        let model_path = Path::new(file_path);
        if !model_path.exists() {
            panic!(
                "[MESH] PANIC: Model file does not exist at path: {}",
                file_path
            );
        }
        let model_dir = model_path
            .parent()
            .expect("Model path must have a parent directory.");

        let (doc, buffers, gltf_images) =
            gltf::import(model_path).expect("Failed to import GLTF file.");

        let mut material_cache: HashMap<usize, Arc<dyn Material + Send + Sync>> = HashMap::new();
        let mut texture_cache: HashMap<usize, Arc<dyn Texture + Send + Sync>> = HashMap::new();
        let default_material = Arc::new(Lambertian::new(Color::new(0.8, 0.0, 0.8)));

        let mut triangle_objects: Vec<Arc<dyn Hittable + Send + Sync>> = Vec::new();

        for mesh in doc.meshes() {
            for primitive in mesh.primitives() {
                // --- 智能材质创建 ---
                let material_index = primitive.material().index();
                let rust_mat = match material_index {
                    Some(index) => {
                        //  检查材质缓存
                        if let Some(cached_mat) = material_cache.get(&index) {
                            Arc::clone(cached_mat)
                        } else {
                            //  如果无缓存，则创建新材质
                            let gltf_material = primitive.material();
                            let pbr = gltf_material.pbr_metallic_roughness();

                            // 检查是否有自发光属性
                            let is_emissive = gltf_material.emissive_factor()[0] > 0.0
                                || gltf_material.emissive_factor()[1] > 0.0
                                || gltf_material.emissive_factor()[2] > 0.0;

                            let new_mat: Arc<dyn Material + Send + Sync> = if is_emissive {
                                // 创建发光材质
                                let emissive_strength = 5.0; // 放大亮度以获得明显效果
                                let color = Color::from_slice(&gltf_material.emissive_factor())
                                    * emissive_strength;
                                Arc::new(DiffuseLight::new_from_color(color))
                            } else {
                                // 创建 PBR 材质 (Metal or Lambertian)
                                // 首先确定纹理：是图片还是纯色？
                                let texture = get_or_create_texture(
                                    &pbr,
                                    &mut texture_cache,
                                    &doc,
                                    &buffers,
                                    model_dir,
                                );
                                // texture_cache.insert(
                                //     pbr.base_color_texture().unwrap().texture().index(),
                                //     Arc::clone(&texture),
                                // );

                                // 根据金属性决定创建 Metal 还是 Lambertian
                                let metallic = pbr.metallic_factor();
                                let roughness = pbr.roughness_factor();
                                if metallic > 0.5 {
                                    Arc::new(Metal::new_from_texture(texture, roughness as f64))
                                } else {
                                    Arc::new(Lambertian::new_from_texture(texture))
                                }
                            };

                            material_cache.insert(index, Arc::clone(&new_mat));
                            new_mat
                        }
                    }
                    None => Arc::clone(&default_material),
                };

                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                if let Some(pos_iter) = reader.read_positions() {
                    let positions: Vec<Point3> = pos_iter
                        .map(|p| Point3::new(p[0] as f64, p[1] as f64, p[2] as f64))
                        .collect();

                    if let Some(index_iter) = reader.read_indices() {
                        let indices: Vec<u32> = index_iter.into_u32().collect();

                        for face_indices in indices.chunks_exact(3) {
                            let p0 = positions[face_indices[0] as usize];
                            let p1 = positions[face_indices[1] as usize];
                            let p2 = positions[face_indices[2] as usize];
                            let tri = Arc::new(Triangle::new(p0, p1, p2, Arc::clone(&rust_mat)));
                            triangle_objects.push(tri);
                        }
                    }
                }
            }
        }
        eprintln!(
            "[MESH] Model loaded with {} triangles. Building BVH...",
            triangle_objects.len()
        );
        let bvh_root = Arc::new(BvhNode::new(&mut triangle_objects));
        let bbox = bvh_root.bounding_box();
        eprintln!("[MESH] BVH built.");
        Self { bvh_root, bbox }
    }
}

fn get_or_create_texture(
    pbr: &gltf::material::PbrMetallicRoughness,
    cache: &mut HashMap<usize, Arc<dyn Texture + Send + Sync>>,
    doc: &gltf::Document,
    buffers: &[gltf::buffer::Data],
    model_dir: &Path,
) -> Arc<dyn Texture + Send + Sync> {
    if let Some(texture_info) = pbr.base_color_texture() {
        let texture_index = texture_info.texture().index();
        if let Some(cached_tex) = cache.get(&texture_index) {
            return Arc::clone(cached_tex);
        }

        let gltf_texture = doc.textures().nth(texture_index).unwrap();

        match gltf_texture.source().source() {
            // 处理外部文件 (URI)
            gltf::image::Source::Uri { uri, .. } => {
                let image_path = model_dir.join(uri);
                // eprintln!(
                //     // "[TEXTURE] Attempting to load texture from URI: {:?}",
                //     image_path
                // );

                // 直接使用 image::open，绕过 RtwImage 的 search_paths
                match image::open(&image_path) {
                    Ok(loaded_image) => {
                        // 使用新加的构造函数
                        let rtw_image = RtwImage::from_image(loaded_image);
                        let image_tex = ImageTexture::from_rtw_image(rtw_image);

                        // eprintln!("[TEXTURE] SUCCESS: Loaded from URI.");
                        let new_tex: Arc<dyn Texture + Send + Sync> = Arc::new(image_tex);
                        cache.insert(texture_index, Arc::clone(&new_tex));
                        return new_tex;
                    }
                    Err(e) => {}
                }
            }
            // 处理嵌入式纹理
            gltf::image::Source::View { view, mime_type } => {
                // eprintln!(
                //     // "[TEXTURE] Loading embedded texture (mime type: {})...",
                //     mime_type
                // );
                let buffer_data = &buffers[view.buffer().index()];
                let image_data_slice = &buffer_data[view.offset()..view.offset() + view.length()];

                match image::load_from_memory(image_data_slice) {
                    Ok(loaded_image) => {
                        // 使用新加的构造函数
                        let rtw_image = RtwImage::from_image(loaded_image);
                        let image_tex = ImageTexture::from_rtw_image(rtw_image);

                        // eprintln!("[TEXTURE] SUCCESS: Loaded embedded image.");
                        let new_tex: Arc<dyn Texture + Send + Sync> = Arc::new(image_tex);
                        cache.insert(texture_index, Arc::clone(&new_tex));
                        return new_tex;
                    }
                    Err(e) => {}
                }
            }
        }
    }

    // 以上都失败，则使用纯色作为后备
    // eprintln!("[TEXTURE] FALLBACK: Using solid color for this material.");
    let color_factor = pbr.base_color_factor();
    Arc::new(SolidColor::new(Color::new(
        color_factor[0] as f64,
        color_factor[1] as f64,
        color_factor[2] as f64,
    )))
}

impl Hittable for Mesh {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        self.bvh_root.hit(r, ray_t, rec)
    }
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

use inline_spirv::*;
use std::collections::{HashMap, HashSet};
use super::*;

macro_rules! gen_entries(
    ($stage:ident, $src:expr, $lang:ident) => {{
        static SPV: &'static [u32] = inline_spirv!($src, $stage, $lang, vulkan1_2);
        ReflectConfig::new()
            .spv(SPV)
            .combine_img_samplers(true)
            .reflect()
            .unwrap()
    }}
);
macro_rules! gen_one_entry(
    ($stage:ident, $glsl:expr) => {{
        let entries = gen_entries!($stage, $glsl, glsl);
        assert_eq!(entries.len(), 1, "expected 1 entry point, found {}", entries.len());
        entries[0].clone()
    }}
);
macro_rules! gen_one_entry_hlsl(
    ($stage:ident, $hlsl:expr) => {{
        let entries = gen_entries!($stage, $hlsl, hlsl);
        assert_eq!(entries.len(), 1, "expected 1 entry point, found {}", entries.len());
        entries[0].clone()
    }}
);

#[test]
fn test_basic_shader_stage() {
    macro_rules! x(
        ($stage:ident, $exec_model:ident) => {{
            let entry = gen_one_entry!($stage, "#version 450 core\nvoid main() {}");
            assert_eq!(entry.exec_model, ExecutionModel::$exec_model, "shader stage execution model mismatched");
            assert!(entry.vars.is_empty(), "unexpected specialization");
        }}
    );
    x!(vert, Vertex);
    x!(frag, Fragment);
}
// For maximal compatibility interface varriables like the output of vertex
// buffer is not forced empty, because it requires prior knowledge from the
// users, that this shader is a vertex shader. The reflection algorithm should
// be able to extract metadata for ANY type of shaders, including those haven't
// been created yet.
#[test]
fn test_vs_input_loc() {
    let entry = gen_one_entry!(vert, r#"
        #version 450 core
        layout(location=0, component=0)
        in uint a;
        layout(location=0, component=1)
        in vec3 b;
        layout(location=1)
        in vec4 c;
        layout(location=3)
        in uvec4 d;
        layout(location=4, component=2)
        in uvec2 e;
        void main() { gl_Position = vec4(a, b) + c + vec4(d) + vec4(e,0,0); }
    "#);
    let locations = entry.vars
        .into_iter()
        .filter_map(|x| if let Variable::Input { location, .. } = x { Some(location) } else { None })
        .collect::<HashSet<_>>();
    assert!(locations.contains(&InterfaceLocation::new(0, 0)));
    assert!(locations.contains(&InterfaceLocation::new(0, 1)));
    assert!(locations.contains(&InterfaceLocation::new(1, 0)));
    assert!(locations.contains(&InterfaceLocation::new(3, 0)));
    assert!(locations.contains(&InterfaceLocation::new(4, 2)));
    assert!(!locations.contains(&InterfaceLocation::new(0, 2)));
    assert!(!locations.contains(&InterfaceLocation::new(1, 1)));
}
#[test]
fn test_fs_output_loc() {
    // Note that fragment shader location-sharing outputs must have the same
    // type.
    let entry = gen_one_entry!(frag, r#"
        #version 450 core
        layout(location=0, component=0)
        out float a;
        layout(location=0, component=1)
        out vec3 b;
        layout(location=1)
        out vec4 c;
        layout(location=3)
        out uvec4 d;
        layout(location=4, component=2)
        out uvec2 e;
        void main() { a = 0; b = vec3(0,0,0); c = vec4(0,0,0,0); d = uvec4(0,0,0,0); e = uvec2(0,0); }
    "#);
    let locations = entry.vars
        .into_iter()
        .filter_map(|x| if let Variable::Output { location, .. } = x { Some(location) } else { None })
        .collect::<HashSet<_>>();
    assert!(locations.contains(&InterfaceLocation::new(0, 0)));
    assert!(locations.contains(&InterfaceLocation::new(0, 1)));
    assert!(locations.contains(&InterfaceLocation::new(1, 0)));
    assert!(locations.contains(&InterfaceLocation::new(3, 0)));
    assert!(locations.contains(&InterfaceLocation::new(4, 2)));
    assert!(!locations.contains(&InterfaceLocation::new(0, 2)));
    assert!(!locations.contains(&InterfaceLocation::new(1, 1)));
}
#[test]
fn test_spec_consts() {
    let entry = gen_one_entry!(geom, r#"
        #version 450 core
        layout(points) in;
        layout(points, max_vertices=1) out;
        layout(constant_id=233) const float a = 0;
        layout(constant_id=234) const float b = 0;
        layout(constant_id=235) const float c = 0;
        void main() { gl_Position = vec4(a,b,c,0); EmitVertex(); EndPrimitive(); }
    "#);
    let spec_ids = entry.vars
        .into_iter()
        .filter_map(|x| x.spec_id())
        .collect::<HashSet<_>>();
    assert!(spec_ids.contains(&233));
    assert!(spec_ids.contains(&234));
    assert!(spec_ids.contains(&235));
    assert!(!spec_ids.contains(&0));
    assert!(!spec_ids.contains(&1));
    assert!(!spec_ids.contains(&236));
}
#[test]
fn test_desc_tys() {
    let entry = gen_one_entry!(frag, r#"
        #version 450 core
        layout(std140, set=0, binding=0)
        uniform A {
            uint a;
        } aa;
        layout(std430, binding=1)
        buffer B {
            float b;
        } bb;
        layout(set=1, binding=0)
        uniform sampler2D c;
        layout(rgba32f, set=3, binding=4) readonly
        uniform image2D d;
        layout(input_attachment_index=3, set=1, binding=3)
        uniform subpassInput e;
        layout(binding=3)
        uniform samplerBuffer f;
        layout(rgba32f, set=3, binding=5) writeonly
        uniform image2D g;
        void main() {
            bb.b = 0.0;
            vec4 x = texelFetch(f, 0) + vec4(aa.a,0,0,0) + bb.b * texture(c, vec2(0,0)) + subpassLoad(e) + imageLoad(d, ivec2(0,0));
            imageStore(g, ivec2(0,0), x);
        }
    "#);
    let desc_binds = entry.vars
        .into_iter()
        .filter_map(|x| {
            if let Variable::Descriptor { desc_bind, desc_ty, .. } = x {
                Some((desc_bind, desc_ty))
            } else { None }
        })
        .collect::<HashMap<_, _>>();
    assert!(desc_binds.contains_key(&DescriptorBinding::new(0, 0)));
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(0, 0)).unwrap(), DescriptorType::UniformBuffer());
    assert!(desc_binds.contains_key(&DescriptorBinding::new(0, 1)));
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(0, 1)).unwrap(), DescriptorType::StorageBuffer(AccessType::ReadWrite));
    assert!(desc_binds.contains_key(&DescriptorBinding::new(1, 0)));
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(1, 0)).unwrap(), DescriptorType::CombinedImageSampler());
    assert!(desc_binds.contains_key(&DescriptorBinding::new(3, 4)));
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(3, 4)).unwrap(), DescriptorType::StorageImage(AccessType::ReadOnly));
    assert!(desc_binds.contains_key(&DescriptorBinding::new(3, 5)));
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(3, 5)).unwrap(), DescriptorType::StorageImage(AccessType::WriteOnly));
    assert!(desc_binds.contains_key(&DescriptorBinding::new(1, 3)));
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(1, 3)).unwrap(), DescriptorType::InputAttachment(3));
    assert!(desc_binds.contains_key(&DescriptorBinding::new(0, 3)));
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(0, 3)).unwrap(), DescriptorType::UniformTexelBuffer());
    assert!(!desc_binds.contains_key(&DescriptorBinding::new(0, 2)));
}
#[test]
fn test_push_consts() {
    let entry = gen_one_entry!(vert, r#"
        #version 450 core
        layout(push_constant)
        uniform A {
            float a;
        } aa;
        void main() { gl_Position = vec4(aa.a,0,0,0); }
    "#);
    entry.vars
        .into_iter()
        .filter_map(|x| { if let Variable::PushConstant { .. } = x { Some(()) } else { None } })
        .next()
        .unwrap();
}
#[test]
fn test_implicit_sampled_img() {
    // Currently only shaderc is outputting binding-sharing image and sampler.
    let _entry = gen_one_entry_hlsl!(vert, r#"
        [[vk::binding(0, 0)]]
        Texture2D img;
        [[vk::binding(0, 0)]]
        SamplerState samp;
        float4 main() : SV_POSITION { return img.Sample(samp, float2(0,0)); }
    "#);
}
#[test]
fn test_dyn_multibind() {
    let entry = gen_one_entry!(frag, r#"
        #version 450 core
        #extension GL_EXT_nonuniform_qualifier: enable
        
        layout(binding = 0, set = 0)
        uniform sampler2D arr_dyn[];
        layout(binding = 1, set = 0)
        uniform sampler2D arr[5];

        layout(location = 0)
        in flat uint xx;

        void main() {
            texture(arr[0], vec2(0,0)) + texture(arr_dyn[xx], vec2(0,0));
        }
    "#);
    let descs = entry.vars
        .into_iter()
        .filter_map(|x| {
            if let Variable::Descriptor { desc_bind, nbind, .. } = x {
                Some((desc_bind, nbind))
            } else { None }
        })
        .collect::<HashMap<_, _>>();
    assert_eq!(*descs.get(&DescriptorBinding::new(0, 0)).unwrap(), 0);
    assert_eq!(*descs.get(&DescriptorBinding::new(0, 1)).unwrap(), 5);
}
#[test]
fn test_spec_const_arrays() {
    static SPV: &'static [u32] = inline_spirv!(r#"
        #version 450 core

        layout(constant_id = 1)
        const double DOUBLE_NUM = 3.0;
        layout(constant_id = 2)
        const uint OFFSET = 2;
        layout(constant_id = 3)
        const uint NUM = 42;
        layout(constant_id = 4)
        const int PERMUTATION = 12;

        layout(binding = 0, set = 0)
        uniform sampler2D arr_spec[NUM * PERMUTATION + 1];

        layout(binding = 1, set = 0, std140)
        uniform Param {
            vec4 padding;
            vec4 trailing_array[NUM];
        } u;

        layout(location = 0)
        in flat uint xx;

        void main() {
            for (uint i = 0; i < NUM; i++) {
                texture(arr_spec[i], vec2(0,0)) + u.padding;
            }
        }
    "#, frag, vulkan1_2);
    let entries = ReflectConfig::new()
        .spv(SPV)
        .combine_img_samplers(true)
        .specialize(1, ConstantValue::F64(4.0))
        .specialize(3, ConstantValue::U32(7))
        .specialize(4, ConstantValue::I32(9))
        .reflect()
        .unwrap();
    assert_eq!(entries.len(), 1, "expected 1 entry point, found {}", entries.len());
    let entry = entries[0].clone();
    let spec_consts = entry.vars
        .iter()
        .filter_map(|x| {
            if let Variable::SpecConstant { spec_id, ty, .. } = x {
                Some((spec_id, ty.clone()))
            } else { None }
        })
        .collect::<HashMap<_, _>>();
    let descs = entry.vars
        .iter()
        .filter_map(|x| {
            if let Variable::Descriptor { desc_bind, nbind, ty, .. } = x {
                Some((desc_bind, (*nbind, ty.nbyte())))
            } else { None }
        })
        .collect::<HashMap<_, _>>();
    assert_eq!(spec_consts.len(), 1);
    assert_eq!(*spec_consts.get(&2).unwrap(), ty::Type::Scalar(ty::ScalarType::Unsigned(4)));
    assert_eq!(*descs.get(&DescriptorBinding::new(0, 0)).unwrap(), (64, None));
    assert_eq!(*descs.get(&DescriptorBinding::new(0, 1)).unwrap(), (1, Some(128)));
}
#[test]
fn test_ray_tracing() {
    let entry = gen_one_entry!(rgen, r#"
        #version 460 core
        #extension GL_EXT_ray_tracing: enable

        uniform accelerationStructureEXT acc;

        layout(location = 0) rayPayloadEXT vec4 payload;

        void main() {
            traceRayEXT(acc, gl_RayFlagsOpaqueEXT, 0xff, 0,
                0, 0, vec3(0, 0, 0), 0.0,
                vec3(0, 0, 0), 100.0f, 0);
        }
    "#);
    let desc_binds = entry.vars
        .into_iter()
        .filter_map(|x| {
            if let Variable::Descriptor { desc_bind, desc_ty, .. } = x {
                Some((desc_bind, desc_ty))
            } else { None }
        })
        .collect::<HashMap<_, _>>();
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(0, 0)).unwrap(), DescriptorType::AccelStruct());
}
#[test]
fn test_combine_image_sampler() {
    let entry = gen_one_entry_hlsl!(frag, r#"
        [[vk::binding(0, 0)]]
        Texture2D shaderTexture;
        [[vk::binding(1, 0)]]
        SamplerState SampleType;

        [[vk::binding(1, 1)]]
        Texture2D shaderTexture2;
        [[vk::binding(1, 1)]]
        SamplerState SampleType2;

        struct PixelInputType
        {
            float4 position : SV_POSITION;
            float2 tex : TEXCOORD0;
        };

        float4 main(PixelInputType input) : SV_TARGET
        {
            float4 textureColor, textureColor2;

            textureColor = shaderTexture.Sample(SampleType, input.tex) + shaderTexture2.Sample(SampleType2, input.tex);

            return textureColor;
        }
    "#);
    let desc_binds = entry.vars
        .into_iter()
        .filter_map(|x| {
            if let Variable::Descriptor { desc_bind, desc_ty, .. } = x {
                Some((desc_bind, desc_ty))
            } else { None }
        })
        .collect::<HashMap<_, _>>();
    assert_eq!(desc_binds.len(), 3);
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(0, 0)).unwrap(), DescriptorType::SampledImage());
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(0, 1)).unwrap(), DescriptorType::Sampler());
    assert_eq!(*desc_binds.get(&DescriptorBinding::new(1, 1)).unwrap(), DescriptorType::CombinedImageSampler());
}

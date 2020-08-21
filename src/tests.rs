use glsl_to_spirv_macros::*;
use glsl_to_spirv_macros_impl::*;
use std::collections::HashSet;
use super::*;

macro_rules! gen_entries(
    ($stage_fn:ident, $glsl:expr) => {{
        static SPV: &'static [u8] = $stage_fn!($glsl);
        SpirvBinary::from(SPV).reflect().unwrap()
    }}
);
macro_rules! gen_one_entry(
    ($stage_fn:ident, $glsl:expr) => {{
        let entries = gen_entries!($stage_fn, $glsl);
        assert_eq!(entries.len(), 1, "expected 1 entry point, found {}", entries.len());
        entries[0].clone()
    }}
);

#[test]
fn test_basic_shader_stage() {
    macro_rules! x(
        ($stage_fn:ident, $exec_model:ident) => {{
            let entry = gen_one_entry!($stage_fn, "#version 450 core\nvoid main() {}");
            assert_eq!(entry.exec_model, ExecutionModel::$exec_model, "shader stage execution model mismatched");
            assert!(entry.spec.spec_consts().next().is_none(), "unexpected specialization");
            assert!(entry.inputs().next().is_none(), "unexpected input");
            assert!(entry.outputs().next().is_none(), "unexpected output");
            assert!(entry.descs().next().is_none(), "unexpected descriptor binding");
        }}
    );
    x!(glsl_vs, Vertex);
    x!(glsl_fs, Fragment);
}
// For maximal compatibility interface varriables like the output of vertex
// buffer is not forced empty, because it requires prior knowledge from the
// users, that this shader is a vertex shader. The reflection algorithm should
// be able to extract metadata for ANY type of shaders, including those haven't
// been created yet.
#[test]
fn test_vs_input_loc() {
    let entry = gen_one_entry!(glsl_vs, r#"
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
    let locations = entry.inputs()
        .map(|x| x.location)
        .collect::<HashSet<_>>();
    assert!(locations.contains(&InterfaceLocation(0, 0)));
    assert!(locations.contains(&InterfaceLocation(0, 1)));
    assert!(locations.contains(&InterfaceLocation(1, 0)));
    assert!(locations.contains(&InterfaceLocation(3, 0)));
    assert!(locations.contains(&InterfaceLocation(4, 2)));
    assert!(!locations.contains(&InterfaceLocation(0, 2)));
    assert!(!locations.contains(&InterfaceLocation(1, 1)));
    // Test for consistency.
    for input in entry.inputs() {
        let resolved = entry.resolve_input(entry.get_input_name(input.location).unwrap()).unwrap();
        assert_eq!(input, resolved);
        assert_eq!(resolved.ty, entry.get_input(input.location).unwrap());
    }
}
#[test]
fn test_fs_output_loc() {
    // Note that fragment shader location-sharing outputs must have the same
    // type.
    let entry = gen_one_entry!(glsl_fs, r#"
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
    let locations = entry.outputs()
        .map(|x| x.location)
        .collect::<HashSet<_>>();
    assert!(locations.contains(&InterfaceLocation(0, 0)));
    assert!(locations.contains(&InterfaceLocation(0, 1)));
    assert!(locations.contains(&InterfaceLocation(1, 0)));
    assert!(locations.contains(&InterfaceLocation(3, 0)));
    assert!(locations.contains(&InterfaceLocation(4, 2)));
    assert!(!locations.contains(&InterfaceLocation(0, 2)));
    assert!(!locations.contains(&InterfaceLocation(1, 1)));
    // Test for consistency.
    for output in entry.outputs() {
        let resolved = entry.resolve_output(entry.get_output_name(output.location).unwrap()).unwrap();
        assert_eq!(output, resolved);
        assert_eq!(resolved.ty, entry.get_output(output.location).unwrap());
    }
}
#[test]
fn test_spec_consts() {
    let entry = gen_one_entry!(glsl_gs, r#"
        #version 450 core
        layout(points) in;
        layout(points, max_vertices=1) out;
        layout(constant_id=233) const float a = 0;
        layout(constant_id=234) const float b = 0;
        layout(constant_id=235) const float c = 0;
        void main() { gl_Position = vec4(a,b,c,0); EmitVertex(); EndPrimitive(); }
    "#);
    let spec_ids = entry.spec.spec_consts()
        .map(|x| x.spec_id)
        .collect::<HashSet<_>>();
    assert!(spec_ids.contains(&233));
    assert!(spec_ids.contains(&234));
    assert!(spec_ids.contains(&235));
    assert!(!spec_ids.contains(&0));
    assert!(!spec_ids.contains(&1));
    assert!(!spec_ids.contains(&236));
    // Test for consistency.
    for spec_const in entry.spec.spec_consts() {
        let resolved = entry.spec.resolve_spec_const(
            entry.spec.get_spec_const_name(spec_const.spec_id).unwrap()
        ).unwrap();
        assert_eq!(spec_const, resolved);
        assert_eq!(spec_const.ty, entry.spec.get_spec_const(spec_const.spec_id).unwrap());
    }
}
#[test]
fn test_desc_tys() {
    let entry = gen_one_entry!(glsl_fs, r#"
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
        layout(input_attachment_index=0, set=1, binding=3)
        uniform subpassInput e;
        layout(std430, binding=3)
        buffer F {
            vec4 f;
        } ff;
        void main() {
            bb.b = 0.0;
            ff.f = vec4(aa.a,0,0,0) + bb.b * texture(c, vec2(0,0)) + subpassLoad(e) + imageLoad(d, ivec2(0,0));
        }
    "#);
    let desc_binds = entry.descs()
        .map(|x| x.desc_bind)
        .collect::<HashSet<_>>();
    assert!(desc_binds.contains(&DescriptorBinding(0, 0)));
    assert!(desc_binds.contains(&DescriptorBinding(0, 1)));
    assert!(desc_binds.contains(&DescriptorBinding(1, 0)));
    assert!(desc_binds.contains(&DescriptorBinding(3, 4)));
    assert!(desc_binds.contains(&DescriptorBinding(1, 3)));
    assert!(desc_binds.contains(&DescriptorBinding(0, 3)));
    assert!(!desc_binds.contains(&DescriptorBinding(0, 2)));
    // Test for consistency.
    for desc in entry.descs() {
        let resolved = entry.resolve_desc(entry.get_desc_name(desc.desc_bind).unwrap()).unwrap();
        assert_eq!(desc, resolved);
        assert_eq!(resolved.desc_ty, entry.get_desc(desc.desc_bind).unwrap());
        if desc.desc_bind == DescriptorBinding(0, 3) {
            assert_eq!(entry.get_desc_access(desc.desc_bind).unwrap(), AccessType::WriteOnly);
        } else if desc.desc_bind == DescriptorBinding(0, 1) {
            assert_eq!(entry.get_desc_access(desc.desc_bind).unwrap(), AccessType::ReadWrite);
        } else {
            assert_eq!(entry.get_desc_access(desc.desc_bind).unwrap(), AccessType::ReadOnly);
        }
    }
}
#[test]
fn test_push_consts() {
    let entry = gen_one_entry!(glsl_vs, r#"
        #version 450 core
        layout(push_constant)
        uniform A {
            float a;
        } aa;
        void main() { gl_Position = vec4(aa.a,0,0,0); }
    "#);
    let resolved = entry.resolve_push_const(
        entry.get_push_const_name().unwrap()
    ).unwrap();
    assert_eq!(entry.get_push_const().unwrap(), resolved.ty);
    assert_eq!(entry.get_push_const().unwrap().resolve(""), resolved.member_var_res);
}
#[test]
fn test_implicit_sampled_img() {
    use shaderc::{CompileOptions, SourceLanguage, ShaderKind, Compiler};
    // Currently only shaderc is outputting binding-sharing image and sampler.
    let src = r#"
        [[vk::bind(0, 0)]]
        Texture2D img;
        [[vk::bind(0, 0)]]
        SamplerState samp;
        float4 main() : SV_POSITION { return img.Sample(samp, float2(0,0)); }
    "#;
    let mut opt = CompileOptions::new().unwrap();
    opt.set_source_language(SourceLanguage::HLSL);
    let mut compiler = Compiler::new().unwrap();
    let out = compiler.compile_into_spirv(src, ShaderKind::Vertex, "<inline>", "main", Some(&opt))
        .unwrap();
    let spv: Vec<u32> = out.as_binary().into();
    SpirvBinary::from(spv).reflect().unwrap();
}
#[test]
fn test_dyn_multibind() {
    let entry = gen_one_entry!(glsl_fs, r#"
        #version 450 core
        #extension GL_EXT_nonuniform_qualifier: enable
        
        layout(binding = 0, set = 0)
        uniform sampler2D arr[];
        layout(location=0)
        in flat int xx;

        void main() {
            texture(arr[xx], vec2(0,0));
        }
    "#);
    assert_eq!(entry.get_desc(DescriptorBinding(0, 0)).unwrap().nbind(), 0);
}
// TODO: (penguinliong) Comprehensive type testing.

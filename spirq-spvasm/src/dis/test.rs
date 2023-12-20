use super::Disassembler;
use inline_spirv::include_spirv;
use pretty_assertions::assert_eq;
use spirq_core::parse::SpirvBinary;

#[test]
fn test_disassembler() {
    let actual = Disassembler::new();
    let spv = include_bytes!("../../../assets/moon.spv");
    let spvasm = actual.disassemble(&SpirvBinary::from(spv.as_ref())).unwrap();
    println!("{}", spvasm);
    let expect = r#"; SPIR-V
; Version: 1.0
; Generator: e0000
; Bound: 97
; Schema: 0
OpCapability Shader
OpCapability RuntimeDescriptorArray
OpExtension "SPV_EXT_descriptor_indexing"
%1 = OpExtInstImport "GLSL.std.450"
OpMemoryModel Logical GLSL450
OpEntryPoint Fragment %PSMain "PSMain" %in_var_POSITION0 %in_var_TEXCOORD0 %in_var_NORMAL0 %in_var_TEXCOORD1 %out_var_SV_Target0
OpExecutionMode %PSMain OriginUpperLeft
OpSource HLSL 660
OpName %type_PushConstant_PushConstant "type.PushConstant.PushConstant"
OpMemberName %type_PushConstant_PushConstant 0 "combined_matrix"
OpMemberName %type_PushConstant_PushConstant 1 "camera_pos"
OpName %constant "constant"
OpName %type_2d_image "type.2d.image"
OpName %tex "tex"
OpName %type_sampler "type.sampler"
OpName %samp "samp"
OpName %type_StructuredBuffer_MaterialInfo "type.StructuredBuffer.MaterialInfo"
OpName %MaterialInfo "MaterialInfo"
OpMemberName %MaterialInfo 0 "base_color_factor"
OpMemberName %MaterialInfo 1 "emissive_factor"
OpMemberName %MaterialInfo 2 "metallic_factor"
OpMemberName %MaterialInfo 3 "roughness_factor"
OpMemberName %MaterialInfo 4 "albedo_texture"
OpMemberName %MaterialInfo 5 "normal_texture"
OpMemberName %MaterialInfo 6 "emissive_texture"
OpName %infos "infos"
OpName %in_var_POSITION0 "in.var.POSITION0"
OpName %in_var_TEXCOORD0 "in.var.TEXCOORD0"
OpName %in_var_NORMAL0 "in.var.NORMAL0"
OpName %in_var_TEXCOORD1 "in.var.TEXCOORD1"
OpName %out_var_SV_Target0 "out.var.SV_Target0"
OpName %PSMain "PSMain"
OpName %type_sampled_image "type.sampled.image"
OpDecorate %in_var_TEXCOORD1 Flat
OpDecorate %in_var_POSITION0 Location 0
OpDecorate %in_var_TEXCOORD0 Location 1
OpDecorate %in_var_NORMAL0 Location 2
OpDecorate %in_var_TEXCOORD1 Location 3
OpDecorate %out_var_SV_Target0 Location 0
OpDecorate %tex DescriptorSet 0
OpDecorate %tex Binding 0
OpDecorate %samp DescriptorSet 0
OpDecorate %samp Binding 1
OpDecorate %infos DescriptorSet 0
OpDecorate %infos Binding 2
OpMemberDecorate %type_PushConstant_PushConstant 0 Offset 0
OpMemberDecorate %type_PushConstant_PushConstant 0 MatrixStride 16
OpMemberDecorate %type_PushConstant_PushConstant 0 RowMajor
OpMemberDecorate %type_PushConstant_PushConstant 1 Offset 64
OpDecorate %type_PushConstant_PushConstant Block
OpMemberDecorate %MaterialInfo 0 Offset 0
OpMemberDecorate %MaterialInfo 1 Offset 16
OpMemberDecorate %MaterialInfo 2 Offset 28
OpMemberDecorate %MaterialInfo 3 Offset 32
OpMemberDecorate %MaterialInfo 4 Offset 36
OpMemberDecorate %MaterialInfo 5 Offset 40
OpMemberDecorate %MaterialInfo 6 Offset 44
OpDecorate %_runtimearr_MaterialInfo ArrayStride 48
OpMemberDecorate %type_StructuredBuffer_MaterialInfo 0 Offset 0
OpMemberDecorate %type_StructuredBuffer_MaterialInfo 0 NonWritable
OpDecorate %type_StructuredBuffer_MaterialInfo BufferBlock
%uint = OpTypeInt 32 0
%uint_4294967295 = OpConstant %uint 4294967295
%int = OpTypeInt 32 1
%int_0 = OpConstant %int 0
%int_1 = OpConstant %int 1
%float = OpTypeFloat 32
%float_255 = OpConstant %float 255
%v3float = OpTypeVector %float 3
%float_1 = OpConstant %float 1
%28 = OpConstantComposite %v3float %float_1 %float_1 %float_1
%float_0 = OpConstant %float 0
%v4float = OpTypeVector %float 4
%mat4v4float = OpTypeMatrix %v4float 4
%type_PushConstant_PushConstant = OpTypeStruct %mat4v4float %v3float
%_ptr_PushConstant_type_PushConstant_PushConstant = OpTypePointer PushConstant %type_PushConstant_PushConstant
%type_2d_image = OpTypeImage %float 2D 2 0 0 1 Unknown
%_runtimearr_type_2d_image = OpTypeRuntimeArray %type_2d_image
%_ptr_UniformConstant__runtimearr_type_2d_image = OpTypePointer UniformConstant %_runtimearr_type_2d_image
%type_sampler = OpTypeSampler
%_ptr_UniformConstant_type_sampler = OpTypePointer UniformConstant %type_sampler
%MaterialInfo = OpTypeStruct %v4float %v3float %float %float %uint %uint %uint
%_runtimearr_MaterialInfo = OpTypeRuntimeArray %MaterialInfo
%type_StructuredBuffer_MaterialInfo = OpTypeStruct %_runtimearr_MaterialInfo
%_ptr_Uniform_type_StructuredBuffer_MaterialInfo = OpTypePointer Uniform %type_StructuredBuffer_MaterialInfo
%_ptr_Input_v3float = OpTypePointer Input %v3float
%v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
%_ptr_Input_uint = OpTypePointer Input %uint
%_ptr_Output_v4float = OpTypePointer Output %v4float
%void = OpTypeVoid
%43 = OpTypeFunction %void
%mat3v3float = OpTypeMatrix %v3float 3
%_ptr_Uniform_MaterialInfo = OpTypePointer Uniform %MaterialInfo
%_ptr_PushConstant_v3float = OpTypePointer PushConstant %v3float
%_ptr_UniformConstant_type_2d_image = OpTypePointer UniformConstant %type_2d_image
%type_sampled_image = OpTypeSampledImage %type_2d_image
%bool = OpTypeBool
%constant = OpVariable %_ptr_PushConstant_type_PushConstant_PushConstant PushConstant
%tex = OpVariable %_ptr_UniformConstant__runtimearr_type_2d_image UniformConstant
%samp = OpVariable %_ptr_UniformConstant_type_sampler UniformConstant
%infos = OpVariable %_ptr_Uniform_type_StructuredBuffer_MaterialInfo Uniform
%in_var_POSITION0 = OpVariable %_ptr_Input_v3float Input
%in_var_TEXCOORD0 = OpVariable %_ptr_Input_v2float Input
%in_var_NORMAL0 = OpVariable %_ptr_Input_v3float Input
%in_var_TEXCOORD1 = OpVariable %_ptr_Input_uint Input
%out_var_SV_Target0 = OpVariable %_ptr_Output_v4float Output
%float_0_00787401572 = OpConstant %float 0.00787401572
%50 = OpConstantComposite %v3float %float_0_00787401572 %float_0_00787401572 %float_0_00787401572
%float_n1_00787401 = OpConstant %float -1.00787401
%52 = OpConstantComposite %v3float %float_n1_00787401 %float_n1_00787401 %float_n1_00787401
%_ptr_Uniform_v4float = OpTypePointer Uniform %v4float
%uint_0 = OpConstant %uint 0
%_ptr_Uniform_v3float = OpTypePointer Uniform %v3float
%uint_1 = OpConstant %uint 1
%_ptr_Uniform_uint = OpTypePointer Uniform %uint
%uint_4 = OpConstant %uint 4
%uint_5 = OpConstant %uint 5
%uint_6 = OpConstant %uint 6
%PSMain = OpFunction %void None %43
%61 = OpLabel
%62 = OpLoad %v3float %in_var_POSITION0
%63 = OpLoad %v2float %in_var_TEXCOORD0
%64 = OpLoad %v3float %in_var_NORMAL0
%65 = OpLoad %uint %in_var_TEXCOORD1
%66 = OpAccessChain %_ptr_Uniform_MaterialInfo %infos %int_0 %65
%67 = OpAccessChain %_ptr_Uniform_v4float %66 %uint_0
%68 = OpLoad %v4float %67
%69 = OpAccessChain %_ptr_Uniform_v3float %66 %uint_1
%70 = OpLoad %v3float %69
%71 = OpAccessChain %_ptr_Uniform_uint %66 %uint_4
%72 = OpLoad %uint %71
%73 = OpAccessChain %_ptr_Uniform_uint %66 %uint_5
%74 = OpLoad %uint %73
%75 = OpAccessChain %_ptr_Uniform_uint %66 %uint_6
%76 = OpLoad %uint %75
%77 = OpExtInst %v3float %1 Normalize %64
%78 = OpAccessChain %_ptr_PushConstant_v3float %constant %int_1
%79 = OpLoad %v3float %78
%80 = OpFSub %v3float %79 %62
%81 = OpAccessChain %_ptr_UniformConstant_type_2d_image %tex %74
%82 = OpLoad %type_2d_image %81
%83 = OpLoad %type_sampler %samp
%84 = OpSampledImage %type_sampled_image %82 %83
%85 = OpImageSampleImplicitLod %v4float %84 %63 None
%86 = OpVectorShuffle %v3float %85 %85 0 1 2
%87 = OpVectorTimesScalar %v3float %86 %float_255
%88 = OpExtInst %v3float %1 Fma %87 %50 %52
%89 = OpAccessChain %_ptr_UniformConstant_type_2d_image %tex %72
%90 = OpLoad %type_2d_image %89
%91 = OpSampledImage %type_sampled_image %90 %83
%92 = OpImageSampleImplicitLod %v4float %91 %63 None
%93 = OpVectorShuffle %v3float %92 %92 0 1 2
%94 = OpAccessChain %_ptr_UniformConstant_type_2d_image %tex %76
%95 = OpLoad %type_2d_image %94
%96 = OpSampledImage %type_sampled_image %95 %83
%97 = OpImageSampleImplicitLod %v4float %96 %63 None
%98 = OpVectorShuffle %v3float %97 %97 0 1 2
%99 = OpFNegate %v3float %80
%100 = OpDPdx %v3float %99
%101 = OpDPdy %v3float %99
%102 = OpDPdx %v2float %63
%103 = OpDPdy %v2float %63
%104 = OpExtInst %v3float %1 Cross %101 %77
%105 = OpExtInst %v3float %1 Cross %77 %100
%106 = OpCompositeExtract %float %102 0
%107 = OpVectorTimesScalar %v3float %104 %106
%108 = OpCompositeExtract %float %103 0
%109 = OpVectorTimesScalar %v3float %105 %108
%110 = OpFAdd %v3float %107 %109
%111 = OpCompositeExtract %float %102 1
%112 = OpVectorTimesScalar %v3float %104 %111
%113 = OpCompositeExtract %float %103 1
%114 = OpVectorTimesScalar %v3float %105 %113
%115 = OpFAdd %v3float %112 %114
%116 = OpDot %float %110 %110
%117 = OpDot %float %115 %115
%118 = OpExtInst %float %1 NMax %116 %117
%119 = OpExtInst %float %1 Sqrt %118
%120 = OpFDiv %float %float_1 %119
%121 = OpVectorTimesScalar %v3float %110 %120
%122 = OpVectorTimesScalar %v3float %115 %120
%123 = OpCompositeConstruct %mat3v3float %121 %122 %77
%124 = OpTranspose %mat3v3float %123
%125 = OpINotEqual %bool %74 %uint_4294967295
OpSelectionMerge %126 None
OpBranchConditional %125 %127 %126
%127 = OpLabel
%128 = OpVectorTimesMatrix %v3float %88 %124
%129 = OpExtInst %v3float %1 Normalize %128
OpBranch %126
%126 = OpLabel
%130 = OpPhi %v3float %77 %61 %129 %127
%131 = OpExtInst %v3float %1 Normalize %28
%132 = OpDot %float %130 %131
%133 = OpExtInst %float %1 NMax %132 %float_0
%134 = OpVectorShuffle %v3float %68 %68 0 1 2
%135 = OpINotEqual %bool %72 %uint_4294967295
OpSelectionMerge %136 None
OpBranchConditional %135 %137 %136
%137 = OpLabel
%138 = OpFMul %v3float %134 %93
OpBranch %136
%136 = OpLabel
%139 = OpPhi %v3float %134 %126 %138 %137
%140 = OpINotEqual %bool %76 %uint_4294967295
OpSelectionMerge %141 None
OpBranchConditional %140 %142 %141
%142 = OpLabel
%143 = OpFMul %v3float %70 %98
OpBranch %141
%141 = OpLabel
%144 = OpPhi %v3float %70 %136 %143 %142
%145 = OpVectorTimesScalar %v3float %139 %133
%146 = OpFAdd %v3float %145 %144
%147 = OpCompositeExtract %float %146 0
%148 = OpCompositeExtract %float %146 1
%149 = OpCompositeExtract %float %146 2
%150 = OpCompositeConstruct %v4float %147 %148 %149 %float_1
OpStore %out_var_SV_Target0 %150
OpReturn
OpFunctionEnd"#;
    assert_eq!(expect, spvasm);
}

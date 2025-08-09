// Copyright (c) 2025 Gobley Contributors.

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vertex(
    @location(0) a_position: vec2<f32>,
    @location(1) a_color: vec3<f32>,
) -> VertexOutput {
    return VertexOutput(vec4<f32>(a_position, 0.0, 1.0), a_color);
}

struct FragmentOutput {
    @location(0) color: vec4<f32>,
}

@fragment
fn fragment(
    @location(0) a_color: vec3<f32>,
) -> FragmentOutput {
    return FragmentOutput(vec4<f32>(a_color, 1.0));
}
#import bevy_pbr::mesh_view_bindings

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
	var N = normalize(world_normal);
	var V = normalize(view.world_position.xyz - world_position.xyz);
	var NdotV = max(dot(N, V), 0.0001);
    var fresnel = clamp(1.0 - NdotV, 0.0, 1.0);
    fresnel = pow(fresnel, 1.5) * 1.5;
	var highlight_color = vec3<f32>(0.0, 0.3, 0.8);
	var body_color = vec3<f32>(1.0, 1.0, 1.0);

	var color = mix(body_color, highlight_color, fresnel);
    return vec4<f32>(color, 0.5);
}
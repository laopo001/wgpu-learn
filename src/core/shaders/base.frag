#version 450
#define use_NORMAL 1;
#define use_COLOR 1;
#define use_TEXCOORD0 1;
//split
#extension GL_ARB_separate_shader_objects : enable
#pragma vscode_glsllint_stage : frag //pragma to set STAGE to 'frag'
const float PI = 3.14159265359;
struct PointLight {
    vec3 position;
    float intensity; // 光的强度
    vec3 color;
};

struct SpotLight {
    vec3 position;
    float angle;
    vec3 color;
    float range;
    vec3 direction;
    float smoothness; // 平稳 平滑
    float intensity; // 光的强度
};
layout(set = 0, binding = 1) uniform sampler u_Sampler;

layout(std140, set = 0, binding = 2) uniform Args {
    layout(offset = 0) mat4 proj_view;
    layout(offset = 64) vec3 camera_pos;
    layout(offset = 76) int point_light_count;
    layout(offset = 80) PointLight point_lights[32];
    layout(offset = 1104) int spot_light_count;
    layout(offset = 1120) SpotLight spot_lights[32];
};
layout(set = 0, binding = 3) uniform texture2D u_pbrBaseColorTexture;

layout(set = 0, binding = 4) uniform texture2D u_pbrMetallicRoughnessTexture;

layout(set = 0, binding = 5) uniform texture2D u_pbrNormalTexture;

layout(set = 0, binding = 6) uniform texture2D u_pbrOcclusionTexture;

layout(set = 0, binding = 7) uniform texture2D u_pbrEmissiveTexture;

// pbr
layout(set = 0, binding = 8) uniform pbrInfo {
    // BaseColorTexture
    vec4 u_pbrBaseColorFactor;
    float u_pbrMetallicFactor;
    float u_pbrRoughnessFactor;
    // normalTexture
    float u_pbrNormalTextureScale;
    // OcclusionTexture
    float u_pbrOcclusionTextureStrength;
    //
    vec3 u_pbrEmissiveFactor;
    uint u_pbrAlphaMode;
    float u_pbrAlphaCutoff;
    bool u_pbrDoubleSided;
};
layout(set = 0, binding = 9) uniform texture2D u_pbrAlbedoTexture;

layout (location = 0) in vec3 v_POSITION;
#if defined (use_NORMAL)
layout (location = 1) in vec3 a_NORMAL;
#endif
#if defined (use_COLOR)
layout (location = 2) in vec4 a_COLOR;
#endif
#if defined(use_TEXCOORD0)
layout (location = 3) in vec2 v_TEXCOORD0; 
#endif
#if defined(use_TEXCOORD1)
layout (location = 4) in vec2 v_TEXCOORD1; 
#endif
layout(location = 5) out vec4 outColor;

vec2 getCurrTEXCOORD(uint index) {
    #if defined(use_TEXCOORD1)
        if(index==1) {
            return v_TEXCOORD1;
        }
    #endif
    #if defined(use_TEXCOORD0)
        if(index==0) {
            return v_TEXCOORD0;
        }
    #endif
    return v_TEXCOORD0;
}

vec4 getBaseColor() {
    vec4 baseColor = vec4(1.0, 1.0, 1.0, 1.0);
    #if defined(use_COLOR)
        baseColor = a_COLOR;
    #endif
    #if defined(use_pbrMetallicRoughnessInfo)
        baseColor = u_pbrBaseColorFactor;
    #endif
    #if defined(use_pbrBaseColorTexture) && defined(use_Sampler)  && defined(use_TEXCOORD0) && defined(use_pbrMetallicRoughnessInfo)
        outColor =  texture(sampler2D(u_pbrBaseColorTexture, u_Sampler), v_TEXCOORD0) * u_pbrBaseColorFactor;
    #endif
    return baseColor;
}

float geometry(float NdotV, float NdotL, float r2) {
    float a1 = r2 + 1.0;
    float k = a1 * a1 / 8.0;
    float denom = NdotV * (1.0 - k) + k;
    float ggx1 = NdotV / denom;
    denom = NdotL * (1.0 - k) + k;
    float ggx2 = NdotL / denom;
    return ggx1 * ggx2;
}

vec3 fresnel(float HdotV, vec3 fresnel_base) {
    return fresnel_base + (1.0 - fresnel_base) * pow(1.0 - HdotV, 5.0);
}

float normal_distribution(vec3 N, vec3 H, float a) {
    float a2 = a * a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return (a2 + 0.0000001) / denom;
}

vec3 compute_light(vec3 attenuation,
                   vec3 light_color,
                   vec3 view_direction,
                   vec3 light_direction,
                   vec3 albedo,
                   vec3 normal,
                   float roughness2,
                   float metallic,
                   vec3 fresnel_base) {

    vec3 halfway = normalize(view_direction + light_direction);
    float normal_distribution = normal_distribution(normal, halfway, roughness2);

    float NdotV = max(dot(normal, view_direction), 0.0);
    float NdotL = max(dot(normal, light_direction), 0.0);
    float HdotV = max(dot(halfway, view_direction), 0.0);
    float geometry = geometry(NdotV, NdotL, roughness2);


    vec3 fresnel = fresnel(HdotV, fresnel_base);
    vec3 diffuse = vec3(1.0) - fresnel;
    diffuse *= 1.0 - metallic;

    vec3 nominator = normal_distribution * geometry * fresnel;
    float denominator = 4 * NdotV * NdotL + 0.0001;
    vec3 specular = nominator / denominator;

    vec3 resulting_light = (diffuse * albedo / PI + specular) * light_color * attenuation * NdotL;
    return resulting_light;
}

void main() {
    vec4 baseColor = getBaseColor();
    if (baseColor.a == 0.0) discard;
    vec3 view_direction = camera_pos - v_POSITION;
    vec3 albedo = texture(sampler2D(u_pbrAlbedoTexture, u_Sampler), v_TEXCOORD0).rgb;
    vec2 metallic_roughness = texture(sampler2D(u_pbrMetallicRoughnessTexture, u_Sampler), v_TEXCOORD0).bg;
    float roughness = metallic_roughness.r * u_pbrRoughnessFactor;
    float metallic = metallic_roughness.g * u_pbrMetallicFactor;
    float roughness2 = roughness * roughness;
    vec3 fresnel_base = mix(vec3(0.04), albedo, metallic);
    vec3 emission = vec3(0.0);
    #if defined(has_pbrEmissiveTexture)
        emission = texture(sampler2D(u_pbrEmissiveTexture, u_Sampler), v_TEXCOORD0).rgb * u_pbrEmissiveFactor;
    #endif

    #if defined(has_pbrNormalTexture)
        vec3 normal = texture(sampler2D(u_pbrNormalTexture, u_Sampler), v_TEXCOORD0).rgb;
        normal = normalize((normal * 2 - 1) * vec3(u_pbrNormalTextureScale, u_pbrNormalTextureScale, 1.0)); // Convert [0, 1] to [-1, 1] and scale
        #define use_NORMAL 1;
    #else
        #if defined(use_NORMAL)
            vec3 normal = a_NORMAL;
        #endif
    #endif
    vec3 lighted = vec3(0.0);
    for (int i = 0; i < point_light_count; i++) {
        vec3 light_direction = point_lights[i].position - v_POSITION.xyz;
        // float light_direction_distance = length(light_direction);
        // float attenuation = point_lights[i].intensity / (light_direction_distance * light_direction_distance);
        float attenuation = point_lights[i].intensity / dot(light_direction, light_direction);

        vec3 light = compute_light(vec3(attenuation),
                                   point_lights[i].color,
                                   view_direction,
                                   normalize(light_direction),
                                   albedo,
                                   normal,
                                   roughness2,
                                   metallic,
                                   fresnel_base);

        lighted += light;
    }

    for (int i = 0; i < spot_light_count; i++) {
        vec3 light_vec = spot_lights[i].position - v_POSITION.xyz;
        vec3 normalized_light_vec = normalize(light_vec);

        // The distance between the current fragment and the "core" of the light
        float light_length = length(light_vec);

        // The allowed "length", everything after this won't be lit.
        // Later on we are dividing by this range, so it can't be 0
        float range = max(spot_lights[i].range, 0.00001);

        // get normalized range, so everything 0..1 could be lit, everything else can't.
        float normalized_range = light_length / max(0.00001, range);

        // The attenuation for the "range". If we would only consider this, we'd have a
        // point light instead, so we need to also check for the spot angle and direction.
        float range_attenuation = max(0.0, 1.0 - normalized_range);

        // this is actually the cosine of the angle, so it can be compared with the
        // "dotted" frag_angle below a lot cheaper.
        float spot_angle = max(spot_lights[i].angle, 0.00001);
        vec3 spot_direction = normalize(spot_lights[i].direction);
        float smoothness = 1.0 - spot_lights[i].smoothness;

        // Here we check if the current fragment is within the "ring" of the spotlight.
        float frag_angle = dot(spot_direction, -normalized_light_vec);

        // so that the ring_attenuation won't be > 1
        frag_angle = max(frag_angle, spot_angle);

        // How much is this outside of the ring? (let's call it "rim")
        // Also smooth this out.
        float rim_attenuation = pow(max((1.0 - frag_angle) / (1.0 - spot_angle), 0.00001), smoothness);

        // How much is this inside the "ring"?
        float ring_attenuation = 1.0 - rim_attenuation;

        // combine the attenuations and intensity
        float attenuation = range_attenuation * ring_attenuation * spot_lights[i].intensity;

        vec3 light = compute_light(vec3(attenuation),
                                   spot_lights[i].color,
                                   view_direction,
                                   normalized_light_vec,
                                   albedo,
                                   normal,
                                   roughness2,
                                   metallic,
                                   fresnel_base);
        lighted += light;
    }
    // outColor = baseColor;
    vec3 ambient_color = vec3(0.01, 0.01, 0.01);
    float ambient_occlusion = 1.0;
    vec3 ambient = ambient_color * albedo * ambient_occlusion;
    outColor = vec4(ambient + baseColor.xyz + lighted + emission, baseColor.a);
}
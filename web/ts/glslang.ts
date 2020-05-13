window.glslang = undefined;
export default async function () {
    if (window.glslang !== undefined) return window.glslang;
    // @ts-ignore
    const glslangModule = await import(/* webpackIgnore: true */ 'https://unpkg.com/@webgpu/glslang@0.0.7/web/glslang.js');
    window.glslang = await glslangModule.default();
    return window.glslang;
}

struct UiVertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) border_widths: vec4<f32>,
    @location(2) @interpolate(flat) size: vec2<f32>,
    @builtin(position) position: vec4<f32>
}

// // 输入参数
// @group(1) @binding(0)
// var<uniform>  DottedBorder
// @group(1) @binding(4)
// var inner_texture: texture_2d<f32>;
// @group(1) @binding(5)
// var texture_sampler: sampler;

// 输入参数
// 边框颜色
@group(1) @binding(0) var<uniform> border_color: vec4<f32>;
// 圆点半径（相对比例）
@group(1) @binding(1) var<uniform> dot_radius: f32;
// 圆点间距（相对比例）
@group(1) @binding(2) var<uniform> dot_spacing: f32;
// 边框宽度（相对比例，0~1）
@group(1) @binding(3) var<uniform> border_width: f32;
// 中间区域动态纹理
@group(1) @binding(4) var inner_texture: texture_2d<f32>;
@group(1) @binding(5) var texture_sampler: sampler;



// 片段着色器
@fragment
fn fragment(
    in: UiVertexOutput,
) -> @location(0) vec4<f32> {
    // 计算到边界的距离
    let border_start = border_width;
    let border_end = 1.0 - border_start;
    
    // 判断是否在边框区域
    let is_border = 
        in.uv.x < border_start || in.uv.x > border_end ||
        in.uv.y < border_start || in.uv.y > border_end;

    if (is_border) {
        // 计算局部 UV（映射到边框范围内）
        var local_uv_x: f32 = 0.0;
        var local_uv_y: f32 = 0.0;
        
        if (in.uv.x < border_start) {
            local_uv_x = in.uv.x / border_start;
        } else if (in.uv.x > border_end) {
            local_uv_x = (in.uv.x - border_end) / border_start;
        }
        
        if (in.uv.y < border_start) {
            local_uv_y = in.uv.y / border_start;
        } else if (in.uv.y > border_end) {
            local_uv_y = (in.uv.y - border_end) / border_start;
        }
        
        let local_uv = vec2<f32>(local_uv_x, local_uv_y);

        // 生成圆形点阵
        let grid_uv = fract(local_uv * (1.0 / dot_spacing));
        let dist = length(grid_uv - 0.5);
        let alpha = step(dist, dot_radius);

        return vec4<f32>(border_color.rgb, alpha * border_color.a);
    } else {
        // 映射内部 UV 到纹理坐标
        let inner_size = border_end - border_start;
        let normalized_uv = (in.uv - vec2<f32>(border_start)) / inner_size;
        return textureSample(inner_texture, texture_sampler, normalized_uv);
    }
}

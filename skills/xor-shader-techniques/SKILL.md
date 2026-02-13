---
name: xor-shader-techniques
description: |
  Use when implementing shader effects like turbulence, fluid, fire, smoke,
  procedural noise, starfields, volumetric rendering, raymarching, glow,
  antialiasing, fractal texturing, or optimizing shader performance with cheap alternatives.
  Triggers on: shader techniques, shader tricks, shader optimization,
  turbulence shader, fluid shader, fire shader, smoke shader,
  procedural noise, dot noise, gyroid noise,
  efficient chaos, star field, particle scatter,
  fractal texturing, LOD texturing, texture scaling,
  volumetric rendering, raymarching, glow effects,
  antialiasing, analytic antialiasing, fwidth,
  shader performance, cheap shader effects,
  GLSL tricks, shader math, shader formulas,
  Xor shader, GM Shaders, mini.gmshaders.com
---

# Xor Shader Techniques

Advanced shader programming techniques by Xor (@XorDev), covering efficient methods for procedural generation, noise, texturing, and visual effects.

## Triggers

- shader techniques, shader tricks, shader optimization
- turbulence shader, fluid shader, fire shader, smoke shader
- procedural noise, dot noise, gyroid noise
- efficient chaos, star field, particle scatter
- fractal texturing, LOD texturing, texture scaling
- volumetric rendering, raymarching, glow effects
- antialiasing, analytic antialiasing, fwidth
- shader performance, cheap shader effects
- GLSL tricks, shader math, shader formulas
- Xor shader, GM Shaders, mini.gmshaders.com

## Overview

This skill provides comprehensive guidance on advanced shader programming techniques discovered and refined by Xor through 14 years of shader development. These techniques emphasize performance, simplicity, and visual quality, offering cheap alternatives to expensive operations while maintaining impressive results.

**Core Philosophy:**
- Simple formulas for complex effects
- Performance-first approach
- Avoid expensive operations when possible
- Fake it well rather than simulate it perfectly
- Leverage mathematical properties (golden ratio, irrational numbers, rotation)

## Technique Catalog

### 1. Turbulence (Fluid Approximation)

**Use Cases:** Water, fire, dust, magic, wind, fog, smoke, clouds

**Core Concept:**
Approximate fluid dynamics using layered sine waves instead of solving Navier-Stokes equations. Each wave is rotated and scaled to break alignment patterns.

**Basic Formula:**
```glsl
// Number of turbulence waves
#define TURB_NUM 10.0
// Turbulence wave amplitude
#define TURB_AMP 0.7
// Turbulence wave speed
#define TURB_SPEED 0.3
// Turbulence frequency
#define TURB_FREQ 2.0
// Turbulence frequency multiplier
#define TURB_EXP 1.4

// Turbulence starting scale
float freq = TURB_FREQ;

// Turbulence rotation matrix (arbitrary angle, not 45° or 90°)
mat2 rot = mat2(0.6, -0.8, 0.8, 0.6);

// Loop through turbulence octaves
for(float i=0.0; i<TURB_NUM; i++)
{
    // Scroll along the rotated y coordinate
    float phase = freq * (pos * rot).y + TURB_SPEED*iTime + i;
    // Add a perpendicular sine wave offset
    pos += TURB_AMP * rot[0] * sin(phase) / freq;

    // Rotate for the next octave
    rot *= mat2(0.6, -0.8, 0.8, 0.6);
    // Scale down for the next octave
    freq *= TURB_EXP;
}
```

**Key Parameters:**
- `TURB_NUM`: Number of waves (8-10 for good results)
- `TURB_AMP`: Overall waviness (0.0-1.0)
- `TURB_FREQ`: Starting frequency (2.0-4.0)
- `TURB_EXP`: Frequency multiplier (1.3-1.6 for smooth, higher for detailed)

**Fire Variation:**
- Compress horizontally and stretch vertically initially
- Scroll upward
- Expand horizontally as it rises
- Creates upward expansion effect

**ShaderToy Demo:** https://shadertoy.com/view/wltcRS

---

### 2. Efficient Chaos (Fast Pseudo-Random Scattering)

**Use Cases:** Stars, rain, leaves, particles, object scattering

**Problem Solved:**
Traditional Worley noise requires sampling all neighboring cells (3^N complexity: 9 in 2D, 27 in 3D). This method works in any dimension with constant cost.

**Core Concept:**
Layer multiple grids with rotation and scaling. No hash functions or neighbor sampling needed.

**Basic Formula:**
```glsl
// Number of layers
#define LAYERS 5.0
// Overall scale
#define SCALE 0.1
// Overall brightness
#define BRIGHTNESS 0.04

// Layer shift (shifting factor)
#define LAYER_SHIFT 2.618
// Layer spacing (spacing factor)
#define LAYER_SCALE 0.6

// Output color
vec3 col = vec3(0.0);

// Starfield coordinates
vec2 c = fragCoord / iResolution.y / SCALE;

// Loop through layers
for(float i = 0.5 / LAYERS; i<1.0; i+=1.0 / LAYERS)
{
    // Rotate layer (Golden Angle: ~137.5°)
    c *= mat2(0.22252093, -0.97492791, 0.97492791, 0.22252093);

    // Get rotated coordinates
    vec2 p = c;

    // Apply layer shifting
    p += LAYER_SHIFT*i;

    // Adjust layer scaling
    p /= 1.0 + LAYER_SCALE*i;

    // Distance to cell center
    float len = length(mod(p, 2.0) - 1.0);
    // Light attenuation with circle cutoff
    float att = max(1.0-len, 0.0) / len;
    col += att;
}
```

**Breaking Patterns:**
```glsl
// Add waves to break visible axis lines (for 2 layers)
#define LAYER_WAVES 0.2
p += LAYER_WAVES*sin(p.yx);
```

**3D Version:**
Works identically in 3D without extra cost! Just use `vec3` and `mat3` for golden angle rotation.

**Key Advantages:**
- O(N) complexity instead of O(3^N)
- Works in any dimension
- No hash/rand functions needed
- No neighbor cell sampling

**ShaderToy Demo:** https://shadertoy.com/view/wfGyzR

---

### 3. Dot Noise (Cheap 3D Noise)

**Use Cases:** Clouds, water, terrain, procedural texturing, volumetric rendering

**Problem Solved:**
3D Value/Perlin/Simplex noise are expensive when sampled many times per pixel. This provides a fast alternative.

**Core Concept:**
Aperiodic gyroids using golden ratio and golden angle rotation. No hash, interpolation, or multi-sampling needed.

**Formula:**
```glsl
float dot_noise(vec3 p)
{
    // The golden ratio
    const float PHI = 1.618033988;

    // Rotating the golden angle on the vec3(1, phi, phi*phi) axis
    const mat3 GOLD = mat3(
        -0.571464913, +0.814921382, +0.096597072,
        -0.278044873, -0.303026659, +0.911518454,
        +0.772087367, +0.494042493, +0.399753815);

    // Gyroid with irrational orientations and scales
    return dot(cos(GOLD * p), sin(PHI * p * GOLD));
    // Ranges from [-3 to +3]
}
```

**Mathematical Background:**
- Gyroids: `dot(cos(p), sin(p.yzx))` creates wavy infinite shapes
- Golden ratio (φ): Most irrational number, prevents period alignment
- Golden angle rotation: Most irrational orientation
- Result: Aperiodic pseudo-noise without traditional noise functions

**Best Use Cases:**
- Low-scale noise on natural shapes
- Clouds and fluids
- Fractal noise layers (fast accumulation)
- Volumetric rendering (many samples per pixel)

**Limitations:**
- Visible patterns in flat, large-scale scenes
- Still composed of periodic waves (just rotated)
- Less suitable for terrain at large scales

**Fractal Layering:**
```glsl
float fractal_dot_noise(vec3 p)
{
    float result = 0.0;
    float amp = 1.0;
    float freq = 1.0;

    for(int i = 0; i < 4; i++)
    {
        result += amp * dot_noise(p * freq);
        amp *= 0.5;
        freq *= 2.0;
    }
    return result;
}
```

**ShaderToy Demo:** https://shadertoy.com/view/DfGGzm

---

### 4. Fractal Texturing (Scale-Consistent LOD)

**Use Cases:** Terrain rendering, large-scale games, natural textures

**Problem Solved:**
- Distance textures repeat too much (artificial)
- Close textures lose detail (blur)
- Traditional mipmapping doesn't solve repetition

**Core Concept:**
Scale textures inversely with depth, round to discrete LOD levels, and blend between levels.

**Formula:**
```glsl
vec4 fractal_texture(sampler2D tex, vec2 uv, float depth)
{
    float LOD = log(depth);
    float LOD_floor = floor(LOD);
    float LOD_fract = LOD - LOD_floor;

    vec2 uv1 = uv / exp(LOD_floor - 1.0);
    vec2 uv2 = uv / exp(LOD_floor + 0.0);
    vec2 uv3 = uv / exp(LOD_floor + 1.0);

    vec4 tex0 = texture2D(tex, uv1);
    vec4 tex1 = texture2D(tex, uv2);
    vec4 tex2 = texture2D(tex, uv3);

    return (tex1 + mix(tex0, tex2, LOD_fract)) * 0.5;
}
```

**Scale Calibration:**
```glsl
// Adjust based on: depth units, texture scale, screen resolution
depth *= 1e3 / iResolution.y;
```

**How It Works:**
1. Take log of depth to get LOD level
2. Round down to discrete levels
3. Sample at 3 LOD levels (floor-1, floor, floor+1)
4. Blend between levels based on fractional part
5. Average adjacent levels for smooth transition

**Best For:**
- Natural textures (grass, terrain, stone)
- Consistent detail at all distances

**Not Ideal For:**
- Structured textures (bricks, tiles) - edges won't blend naturally

**ShaderToy Demo:** https://shadertoy.com/view/NdVfzw

---

### 5. fwidth Outlines

**Use Cases:** Quick outlines, edge detection, cel shading

**Core Concept:**
Use derivative functions to detect edges almost for free. Works with any continuous gradient.

**Basic Formula:**
```glsl
// For any distance field or continuous function
float outline = step(fwidth(dist), abs(dist));

// Alternative with thickness control
float outline = smoothstep(0.0, fwidth(dist), abs(dist));
```

**Quality:**
Not the best quality, but extremely cheap. Works in a pinch for:
- Debug visualization
- Stylized rendering
- Quick edge detection

**Limitations:**
- Coarse 2x2 derivative blocks can cause artifacts
- Only works with continuous functions
- Not as precise as dedicated outline methods

---

### 6. Volumetric Raymarching (Glow Effects)

**Use Cases:** Clouds, fire, smoke, light rays, fog, volumetric lighting

**Core Concept:**
Similar to SDF raymarching, but use density fields instead of distance fields. Accumulate color samples along the ray.

**Density Field Example:**
```glsl
// Density field (Tunnel + irregular gyroid)
float volume(vec3 p)
{
    return 3.5 - 0.25*length(p.xy) + 0.5*dot(sin(p), cos(p*0.618).yzx);
}
```

**Glow Raymarch Loop:**
```glsl
#define BRIGHTNESS 0.002
#define STEPS 100.0

vec3 col = vec3(0.0);

for(float i = 0.0; i<STEPS; i++)
{
    // Glow density
    float vol = volume(pos);
    // Step forward
    pos += dir * vol;

    // Add the sample color (inverse square attenuation)
    col += vec3(3, 2, 1) / vol;
}

// Tanh tonemapping
col = tanh(BRIGHTNESS*col);
```

**Alpha Blending (for clouds/smoke):**
```glsl
color = mix(color, vec4(sample_rgb, 1), (1.0 - color.a) * sample_alpha);

// Stop when opaque (optimization)
if (color.a > 0.998) break;
```

**Depth of Field with Raymarching:**
```glsl
// Focus ratio should be small like 0.1 or 0.01
float focus = abs(depth - focal_point) * focal_ratio;
// Step through when out of focus
step_dist = max(step_dist - focus, focus);
```

**Key Differences from SDF Raymarching:**
- Use density instead of distance
- Accumulate color at every step (not just final intersection)
- Allow passthrough (never reach zero step size)
- Smaller steps in high-density areas

**ShaderToy Demos:**
- Glow: https://shadertoy.com/view/4dX3zH
- Clouds: https://shadertoy.com/view/lsBfzG

---

### 7. Analytic Anti-Aliasing

**Use Cases:** Smooth edges, filtering, preventing aliasing artifacts

**Problem:** Aliasing occurs when we have more color data than pixels can represent.

**Solution Levels:**

#### Level 1: SDF-Based (Cheapest, Best Quality)
For circles, lines, squares, or any signed distance field:

```glsl
// dist = distance to edge (in world units)
// texel = pixel size in world units
float gradient = clamp((radius - dist) / texel + 0.5, 0.0, 1.0);
```

**Requirements:**
- Accurate distance field
- Known pixel scale

#### Level 2: fwidth-Based (General Purpose)
For any continuous function with inconsistent gradients:

```glsl
// Approximate smooth edges from any continuous function
float antialias_l1(float d)
{
    // Divide d by its derivative width
    return clamp(0.5 + d / fwidth(d), 0.0, 1.0);
}

// Better quality version (L2 norm)
float antialias_l2(float d)
{
    // x and y derivatives
    vec2 dxy = vec2(dFdx(d), dFdy(d));
    // Get gradient width
    float width = length(dxy);
    // Calculate reciprocal scale (avoid division by 0!)
    float scale = width > 0.0 ? 1.0/width : 1e7;
    // Normalize the gradient d with its scale
    return clamp(0.5 + 0.7 * scale * d, 0.0, 1.0);
}
```

**Use For:**
- Distorted distance fields
- Noise functions
- Any continuous gradient

#### Level 3: Manual Derivatives (Edge Cases)
When 2x2 derivative blocks are too coarse or function is discontinuous:

```glsl
// For when derivatives must be manually calculated
float antialias_l2_dxy(float d, vec2 dxy)
{
    float width = length(dxy);
    float scale = width > 0.0 ? 1.0/width : 1e7;
    return clamp(0.5 + 0.7 * scale * d, 0.0, 1.0);
}

// Compute manual derivatives (3 samples)
float grad00 = grad(pos);
float grad10 = grad(pos + vec2(1, 0));
float grad01 = grad(pos + vec2(0, 1));
vec2 dxy = vec2(grad10, grad01) - grad00;

// Use with antialiasing
float aa = antialias_l2_dxy(grad00, dxy);
```

**Use For:**
- Discontinuous functions (floor, fract, step)
- Stripy patterns with many edges
- When fwidth produces artifacts
- Fine detail requiring precision

**Handling Discontinuities:**
- Use continuous version of gradient for derivative calculation
- Apply discontinuous function after anti-aliasing
- Example: `fract(grad) - 0.5` breaks derivatives → use `grad` for derivatives, then apply `fract`

**Key Principle:**
Derivatives only care about rate of change, not absolute value. Calculate derivatives on continuous functions even if final result is discontinuous.

**ShaderToy Demos:**
- Circle AA: https://shadertoy.com/view/XXXXXx
- Checkerboard: https://shadertoy.com/view/XXXXXx

---

## Additional Techniques

### Blocky Turbulence
Variation on standard turbulence with quantized waves for stylized effects.

**Reference:** https://x.com/XorDev/status/1975230424982183956

### Radial Noise
Easy radial noise patterns using normalized coordinates:

```glsl
snoise2D(normalize(p) * SCALE)
```

Creates natural radial patterns without complex math.

### Fast 3D Pixel Sorting
Efficient sorting for 3D ray ordering in volumetric rendering.

**ShaderToy:** https://shadertoy.com/view/mdfyzl

---

## Mathematical Constants

### Golden Ratio (φ)
```glsl
const float PHI = 1.618033988;
```

**Properties:**
- Most irrational number
- φ = (1 + sqrt(5)) / 2
- φ^2 = φ + 1

**Uses:**
- Aperiodic patterns
- Natural spacing
- Fibonacci spirals

**Reference:** https://mini.gmshaders.com/p/phi

### Golden Angle
```glsl
// ~137.5 degrees or ~2.4 radians
const float GOLDEN_ANGLE = 2.39996322972865332;

// Rotation matrix for golden angle
mat2 gold = mat2(
    0.22252093, -0.97492791,
    0.97492791,  0.22252093
);

// 3D rotation on vec3(1, phi, phi^2) axis
const mat3 GOLD_3D = mat3(
    -0.571464913, +0.814921382, +0.096597072,
    -0.278044873, -0.303026659, +0.911518454,
    +0.772087367, +0.494042493, +0.399753815
);
```

**Properties:**
- 360° - 360°/φ^2
- Most irrational angle
- Never aligns with itself after any number of rotations

**Uses:**
- Breaking tiling patterns
- Uniform distribution (sunflower seed pattern)
- Layer rotation

**Reference:** https://mini.gmshaders.com/i/139108917/golden-angle

---

## Performance Tips

### Avoid Expensive Operations
1. **atan/atan2**: Very expensive, often unnecessary
   - Converting vector → angle → vector is wasteful
   - Operations can usually be done directly on vectors

2. **Division**: More expensive than multiplication
   - Prefer: `x * 0.5` over `x / 2.0`
   - Pre-calculate reciprocals when dividing repeatedly

3. **Branching**: GPU-unfriendly
   - Use `mix()`, `step()`, `smoothstep()` instead of `if`
   - Compute both paths and blend when possible

4. **sqrt/length**: Moderate cost
   - Use squared distance when possible
   - `dot(v, v)` instead of `length(v)` for comparisons

### Optimize Sampling
- Reduce samples in raymarch loops
- Use adaptive step sizes (larger in empty space)
- Early exit when possible (opacity > 0.998)

### Leverage Built-ins
- `fwidth()` is nearly free (hardware support)
- Built-in derivatives faster than manual calculation
- Use swizzling instead of separate operations

---

## Tonemapping

**Tanh Tonemapping (Recommended):**
```glsl
col = tanh(BRIGHTNESS * col);
```

**Properties:**
- Smooth, natural rolloff
- Preserves color ratios
- Fast and cheap
- Maps [0, ∞) to [0, 1)

**Reference:** https://mini.gmshaders.com/p/tonemaps

---

## Resources

### Main Tutorial Site
**GM Shaders:** https://mini.gmshaders.com

### Key Articles
1. **Turbulence:** https://mini.gmshaders.com/p/turbulence
2. **Efficient Chaos:** https://mini.gmshaders.com/p/chaos
3. **Dot Noise:** https://mini.gmshaders.com/p/dot-noise
4. **Fractal Texturing:** https://mini.gmshaders.com/p/gm-shaders-mini-fractal-texturing-1408552
5. **Volumetric Raymarching:** https://mini.gmshaders.com/p/volumetric
6. **Anti-Aliasing:** https://mini.gmshaders.com/p/antialiasing
7. **FXAA:** https://mini.gmshaders.com/p/fxaa
8. **Derivatives:** https://mini.gmshaders.com/p/derivatives
9. **Raymarching:** https://mini.gmshaders.com/p/raymarching
10. **Signed Distance Fields:** https://mini.gmshaders.com/p/sdf

### Author
**Xor (@XorDev)**
- Twitter: https://x.com/XorDev
- ShaderToy: https://www.shadertoy.com/user/XorDev
- 14+ years of shader programming experience

### Related Techniques
- **Navier-Stokes Simulation:** https://shadertoy.com/view/XXXXXx (Gijs)
- **Vorticity Confinement:** Chimera's Breath by Nimitz
- **Fluid Simulation:** "Fast Fluid Dynamics Simulation on the GPU" by Mark J. Harris
- **Radiance Cascades:** https://mini.gmshaders.com/p/radiance-cascades

---

## Usage Guidelines

### When to Use Each Technique

**Turbulence:**
- Real-time fluid effects without simulation
- Memory-constrained environments
- Stylized or approximate fluids acceptable

**Efficient Chaos:**
- Scattering large numbers of objects
- 3D applications (huge performance benefit)
- Starfields, particle systems

**Dot Noise:**
- Volumetric rendering (many samples/pixel)
- Clouds, smoke, fog
- Low-scale natural textures
- Fractal noise layers

**Fractal Texturing:**
- Large-scale games with varying view distances
- Natural terrain and textures
- When detail consistency matters at all scales

**Volumetric Raymarching:**
- Atmospheric effects
- Glowing objects (fire, magic, lasers)
- Translucent materials (smoke, fog)

**Anti-Aliasing:**
- Level 1: Always for SDFs (extremely cheap)
- Level 2: General purpose, most situations
- Level 3: High-precision or discontinuous functions only

### Combining Techniques

**Example: Fire Effect**
```glsl
// 1. Use turbulence for fluid motion
vec2 uv = uv_input;
apply_turbulence(uv);

// 2. Sample noise with dot_noise (cheap 3D)
float density = dot_noise(vec3(uv, time));

// 3. Raymarch with glow accumulation
vec3 fire_color = volumetric_raymarch(density);

// 4. Apply antialiasing to edges
fire_color *= antialias_l2(edge_dist);

// 5. Tonemap final result
fire_color = tanh(BRIGHTNESS * fire_color);
```

**Example: Starfield**
```glsl
// Use efficient chaos for star positions
vec3 stars = vec3(0.0);
efficient_chaos_3d(pos, stars);

// Apply slight turbulence for movement
apply_light_turbulence(pos);

// No antialiasing needed (point lights)
```

---

## Common Pitfalls

1. **Division by zero in anti-aliasing**
   - Always check: `width > 0.0 ? 1.0/width : 1e7`

2. **Wrong texture scale in fractal texturing**
   - Calibrate: `depth *= scale / iResolution.y`
   - Adjust based on scene

3. **Too few turbulence layers**
   - Minimum 5-8 for good results
   - Less causes visible patterns

4. **Forgetting golden angle rotation**
   - Rotation breaks tiling patterns
   - Use golden angle for best distribution

5. **Using fwidth on discontinuous functions**
   - Calculate derivatives on continuous version
   - Apply discontinuity after anti-aliasing

6. **Expensive operations in loops**
   - Avoid atan, division in inner loops
   - Pre-calculate when possible

7. **Over-using manual derivatives**
   - Built-in derivatives are faster
   - Only use manual for edge cases

---

## Debug Techniques

### Visualize Turbulence Layers
```glsl
// Color each layer differently
col += LAYER_COLOR[i] * sin(phase);
```

### Check Anti-Aliasing Scale
```glsl
// Visualize derivative width
col = vec3(fwidth(dist) * 10.0);
```

### Verify Noise Patterns
```glsl
// Test at different scales
noise_val = dot_noise(p * scale);
col = vec3(noise_val * 0.5 + 0.5);
```

### Profile Raymarch Steps
```glsl
// Color by iteration count
col = vec3(float(iterations) / MAX_STEPS);
```

---

## Advanced Topics

### Seamless Loops
Use periodic functions with carefully chosen frequencies for seamless animation loops.

### Reading Math Papers
Break down complex papers into implementable components. Focus on core formulas, ignore excessive proofs.

**Reference:** https://mini.gmshaders.com/p/reading-math-papers

### Understanding Derivatives
Derivatives measure rate of change. In shaders:
- `dFdx(x)`: Change in x direction
- `dFdy(x)`: Change in y direction
- `fwidth(x)`: Total change (abs(dFdx) + abs(dFdy))

**Reference:** https://mini.gmshaders.com/p/derivatives

---

## Philosophy

Xor's approach to shader programming emphasizes:

1. **Simplicity**: Simple formulas often produce complex, beautiful results
2. **Performance**: Always consider the cost, especially in loops
3. **Approximation**: Faking effects is often better than accurate simulation
4. **Mathematics**: Leverage mathematical properties (golden ratio, irrationals)
5. **Iteration**: Refine techniques over years of experimentation
6. **Sharing**: Make knowledge accessible to others

**Quote from Xor:**
> "Over the years, I keep finding and developing new tricks for recreating complexity with simple formulas."

---

## Conclusion

These techniques represent years of refinement and optimization. They prioritize:
- Real-time performance
- Visual quality
- Simplicity of implementation
- Broad applicability

Master these fundamentals, then experiment and develop your own variations. The best shader techniques often come from curiosity and iteration.

For more tutorials and updates, visit **GM Shaders**: https://mini.gmshaders.com

---

## License

These techniques are documented for educational purposes based on publicly available tutorials by Xor. Original content and code examples are from GM Shaders (https://mini.gmshaders.com) and ShaderToy (https://shadertoy.com/user/XorDev).

**Credit:** All techniques by Xor (@XorDev)

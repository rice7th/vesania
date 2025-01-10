# API
- [ ] Implement DPI settigs (e.g. rendering at 2x the resolution)
- [ ] Implement Cubic beziers as approximations (i.e. Cubic beziers become a collection of quadratic ones)
- [ ] Implement common shapes (circles, squares, etc.)
- [ ] Create a path builder API

# Core
- [x] Curve splitting
- [ ] Line elision
- [ ] Rasterizer rework
    - [ ] Analytic AA (aka AAA)
- [ ] Implement stroking
    - [ ] Stroke positioning (Inside-center-outside)
    - [ ] Dashing
    - [ ] Start/End caps
    - [ ] Rounded corners
- [ ] New fill operations
    - [x] Solid
    - [ ] Linear gradient
    - [x] Radial gradient
    - [ ] Angular gradient
    - [ ] Mesh gradient
    - [ ] Texture
    - [x] Any shader -> Technically this supersedes everything above
- [ ] Path operations (not sure)
    - [ ] Union
    - [ ] Difference
    - [ ] Intersection
    - [ ] Exclusion

# Compositor
- [ ] Blending
    - [ ] Clipping
    - [ ] Filtering
    - [ ] Interpolation
    - [ ] Gamma correction
- [ ] Affine transforms
    - [ ] Translations
    - [ ] Rotations
    - [ ] Skews
    - [ ] Wraps & Buldges
- [ ] Fast rendering using tiles and caching
- [ ] More color spaces
    - [x] RGBA
    - [ ] CMYK?

# Text
- [ ] Support for OpenType and TrueType fonts
  - [ ] Variable fonts
  - [ ] Opentype features
  - [ ] Emoji rendering
  - [ ] Fast glyph caching
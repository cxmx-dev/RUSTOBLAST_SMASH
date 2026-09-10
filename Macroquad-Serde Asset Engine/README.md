# Macroquad-Serde Asset Engine

Made possible by **Macroquad**, **Serde**, and **Serde JSON**.

A Rust + Macroquad procedural asset engine that renders 2D, 2.5D, and 3D wireframe assets defined in JSON.

## 🎮 Control Scheme

| Input | Action |
| :--- | :--- |
| **Left Arrow** | Previous Asset File |
| **Right Arrow** | Next Asset File |
| **Left Click + Drag** | Orbit Camera (Rotate View) |
| **Mouse Scroll** | Zoom In / Out |
| **Hot Reload** | Edit any `.json` file in `Assets/` to see changes instantly! |

## 🚀 20 Things We Can Still Do

1.  **Filled Polygons**: Add a `filled: true` property to shapes to render solid colors instead of wireframes.
2.  **Lighting & Shading**: Implement basic diffuse/specular lighting for filled 3D shapes.
3.  **Textures**: Support mapping textures to `Poly` or `Cylinder` surfaces.
4.  **Particles**: Add a particle system for effects like engine trails, smoke, or venom drips.
5.  **Physics Integration**: Add rigid body physics so the spider can walk over terrain.
6.  **Procedural Generation**: Create a "Randomize" button to generate new spider variations on the fly.
7.  **Skeletal Animation**: Implement a bone hierarchy so legs move naturally relative to the body.
8.  **Inverse Kinematics (IK)**: Make the spider's feet pin to the ground while the body moves.
9.  **UI Overlay**: Add an immediate-mode GUI (egui) to tweak animation parameters in real-time.
10. **Export to OBJ/STL**: Allow saving the procedural creation as a 3D model file for printing.
11. **First-Person/Cockpit View**: Add a camera mode that sits inside the shape.
12. **Sound Effects**: Add audio that reacts to the animation speed or movement.
13. **Bloom / Post-Processing**: Add a "Neon Mode" with glowing lines.
14. **LOD System**: Automatically reduce polygon count for objects far away.
15. **VR Support**: Port the viewer to WebXR or OpenVR for immersive viewing.
16. **Scripting**: Allow embedding Lua or Rhai scripts in the JSON for complex logic.
17. **Multi-Select**: View multiple assets in the same scene.
18. **Environment**: Add a procedural terrain or city for the shapes to inhabit.
19. **Network Sync**: Allow two users to view the same session over the internet.
20. **Gif Recording**: One-button press to record a loop of the current animation.
21. **Filled Shapes**: Add a filled: bool (default false) to Poly/Circle/Cylinder/Triangle – render solid faces using draw_triangle_3d or mesh building for filled volumes.
22. **Basic Lighting**: Implement simple directional light + normal calculation on filled shapes for diffuse shading (use macroquad's gl for custom shaders if needed).
23. **Texture Mapping**: Add texture: String field to load images via load_texture and apply to cylinders/polys (start with UV unwrap for cylinders).
24. **Particle Effects**: Add a ParticleEmitter part type in JSON – position, rate, velocity, lifetime, color – rendered with macroquad particles.
25. **Skeletal Animation**: Introduce bones hierarchy in JSON (parent indices, rest poses) and animate via matrix palette (forward kinematics first).
26. **Inverse Kinematics**: Add IK chains (e.g., for spider legs) with target pins – solve with CCD or FABRIK for foot placement.
27. **Real-Time Parameter Tweaking**: Integrate egui (or macroquad's miniquad-sapp UI) for live sliders on animation speed/amplitude, colors, scales.
28. **Export to 3D Formats**: Add a keybind (e.g., E) to generate and save current design as OBJ or GLTF (triangulate polys, include cylinders).
29. **Multiple Camera Modes**: Add chase cam, first-person (attach to a point), or cockpit view inside hollow shapes.
30. **Audio Reactivity**: Load sound files and modulate animation amplitude/speed based on audio spectrum (use macroquad's audio API).
31. **Post-Processing Effects**: Add bloom/glow for lines (custom shader pass) or "neon mode" toggle.
32. **Level of Detail (LOD)**: Auto-reduce sides on distant shapes or switch to simpler representations based on camera distance.
33. **VR/AR Support**: Port to WebXR (macroquad supports WASM) or add stereo rendering for immersive viewing.
34. **Embedded Scripting**: Allow script: String field with Rhai/Lua for per-part custom logic (e.g., procedural offsets over time).
35. **Scene Composition**: Support multiple designs loaded simultaneously (scene.json referencing other assets with transforms).
36. **Procedural Randomization**: Add a "Randomize" key (R) that mutates parameters within ranges defined in JSON (min/max fields).
37. **Physics Simulation**: Integrate rapier (or simple verlet) so parts can react to gravity/forces (great for dangling chains).
38. **Animation Timelines**: Replace simple wiggle with keyframe tracks per part (position/rotation/scale over time).
39. **Gif/Video Recording**: Add keybind to capture frames and export animated GIF or MP4 (use macroquad's screenshot + external tool or in-memory encoding).
40. **Multiplayer Viewer**: Simple WebSocket sync so two instances can share the same camera/view and see edits in real-time.

The game crate loads these JSON files from `Assets/` using relative paths (project root, or one folder up). Save in this editor; the runtime picks them up on the next launch. No machine-absolute asset paths.

## Version History

2026-08-12
- Noted relative `Assets/` load used by the game crate.

### v0.5 - Rebranding
- **Changed**: Project renamed to **Macroquad-Serde Asset Engine**.
- **Updated**: Codebase references and Window Title updated to reflect the new name.

### v0.4 - 3D Volume Upgrade
- **Added**: `Cylinder` shape type with adjustable radius and sides for volumetric wireframes.
- **Updated**: `3D-spider.json` now features cylindrical legs instead of flat lines.

### v0.3 - The Third Dimension
- **Added**: Full 3D support with `Camera3D` and Orbit Controls.
- **Refactor**: Upgraded coordinate system to `Vec<f32>` to support `[x, y, z]` or `[x, y]`.
- **Added**: `3D-spider.json` asset demonstrating true 3D geometry.

### v0.2 - 2.5D Depth
- **Added**: `z` property to all shapes.
- **Added**: Depth scaling (parallax) for 2D assets.
- **Added**: `2D-spider.json` showing 2.5D layering.

### v0.1 - Animation & Hot Reloading
- **Added**: Procedural "Wiggle" animation for Lines.
- **Added**: JSON Hot-Reloading system.
- **Added**: Basic Shape Types (`Line`, `Triangle`, `Poly`, `Circle`).

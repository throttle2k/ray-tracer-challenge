
# Ray-tracer challenge
This is my implementation of the book [The Ray Tracer Challenge](http://raytracerchallenge.com/).
Everything is developed in Rust 🦀.

- [x] Tuples, Points, and Vectors
- [x] Drawing on a Canvas
- [x] Matrices
- [x] Matrix Transformations
- [x] Ray-Sphere Intersections
- [x] Light and Shading
- [x] Making a Scene
- [x] Shadows
- [x] Planes
- [x] Patterns
- [x] Reflection and Refraction
- [x] Cubes
- [x] Cylinders
- [x] Groups
- [ ] Triangles
- [ ] Constructive Solid Geometry (CSG)
- [ ] Next Steps

## Some points of attention
- The transformation matrices are pre-calculated for better efficiency
- Added parallelization for faster rendering times
- To handle tree-like structures, I've added a registry shared between the components via lazy-static
- Tried to use types as much as possible (the book uses only Tuples, I used specific types for Points and Vectors)
- Added an implementation of Octrees for better rendering performance

Still a work in progress, and most of the code must be refactored.

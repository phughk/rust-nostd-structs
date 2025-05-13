# Nostd Structs (and Algorithms)

The purpose of this crate is to provide datastructures and algorithms that are useful in environments without std.
This goes for embedded devices, no OS environments, WebAssembly, and code where you prefer stack allocation over heap.

The algorithms and datastructures themselves will be wide-ranging and fairly esoteric, but nontheless applicable.

This is project really created as a central place for community contributions.
I will be working on this myself for the time being, but the intention is that this becomes quite bigger than it
currently is.

# Features

Some of the features are described below.
Many features are not covered by the README, so you are encouraged to browse.
The code is entirely open source and available on github for browsing.
You can use the tests to understand how it should be used.

## Geometry

There is a 2D shape trait implemented by Triangles and Polygons.
Circle will be added soon.

The shapes can be rotated, moved, and combined into a convex hull.
A convex hull is useful for field of view calculations in 2d games.

It is also possible to find the closest point to a shape or check if a point is within a shape.

## Math

There are lookup tables for sin, cos, and tan.
The lookup table contains 3600 entries ranging from 0 to 2*pi.
The granularity also allows for moderately accurate radians calculations.

## Algebra

### Linear equations

- You can derive linear equations from 2 points.
- You can derive an orthogonal linear equation from a linear equation at a point.
- You can project a point onto a linear equation.

Linear equations serve as planes in 2D space and can be used for various field of view calculations.

## Colour

You can convert 1 bit pixels (on/off) to 5 bit pixels (16-colour mode).
This is useful for embedded devices with limited graphics capabilities.
It also allows for high compression of graphics data in 1bpp and then transforming to colour on device when it needs to
be rendered.

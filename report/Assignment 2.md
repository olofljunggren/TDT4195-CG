---
# This is a YAML preamble, defining pandoc meta-variables.
# Reference: https://pandoc.org/MANUAL.html#variables
# Change them as you see fit.
title: TDT4195 Assignment 2
author:
- Olof Ljunggren
date: \today # This is a latex command, ignored for HTML output
lang: en-US
papersize: a4
geometry: margin=4cm
toc: false
toc-title: "Table of Contents"
toc-depth: 2
numbersections: true
header-includes:
# The `atkinson` font, requires 'texlive-fontsextra' on arch or the 'atkinson' CTAN package
# Uncomment this line to enable:
#- '`\usepackage[sfdefault]{atkinson}`{=latex}'
colorlinks: true
links-as-notes: true
# The document is following this break is written using "Markdown" syntax
---

<!--
This is a HTML-style comment, not visible in the final PDF.
-->

# Visual computing fundamentals TDT4195 Assignment 2

## Task 1
Task 1: Per-Vertex Colors [0.5 points]

### a: Extend function create_vao to enable texture specification
Done
### b: Render a scene containing at least 3 different triangles, with diffrent vertex colours
For every fragment in a triangle OpenGL interpolates the colour value depending on the three verticies colours. This is done automatically by a mathematical formula which describes every point inside the triangle as a linear combination of the verticies.
![
    Five triangles with interpolated colours.
](images/interpolatedcolours.png)


## Task 2
Alpha Blending and Depth [0.5 points]

### a: Drawing triangles at diffrent depth.
![
    Swapped colours example 1.
](images/swappedcolors1.png)


### b: Swap colours and depth of the triangles.

1. Swap colours, what happens with the blended colours?
    - What happens is that the last drawn triangle have larger effect on the colours. The blending is not all the same when we draw the same triangles in diffrent order. Effectively swapping the colours means swapping the order the colours are drawn.

![
    Swapped colours example 2.
](images/swappedcolors2.png)
![
    Swapped colours example 3.
](images/swappedcolors3.png)

2. When does it occur?
    - Similarly as for the colour swap the order of the z makes a diffrence. There is no blending effect when the triangle with lowest z (closest to the viewer) are drawn first. Then it will override the other colours since they dont fulfill the depth test. This is where the depth buffer is used.
![
    Swapped z example 1.
](images/swappedz.png)


## Task 3
The Affine Transformation Matrix [0.7 points]

### a: Modify the vertex shader so that the coordinates is being multiplied by a 4x4 matrix. 
Done

### b: Change matrix varaibles to describe their function in the transformation matrix.


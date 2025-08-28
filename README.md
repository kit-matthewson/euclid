# Euclid
Euclid is a program written in Rust that allows the user to create geometric using compass and straightedge ('Euclidean Construction').

The intersections of constructed lines are calculated and used as snapping points so that subsequent lines can be placed accurately.
The display is created using [egui](https://github.com/emilk/egui) with a graph for plotting the lines.
Constructions can be saved or loaded from YAML files using [serde](https://github.com/serde-rs/serde), and a few examples are provided.

## Features
- Changing the colour and width of lines.
- Constructions using a straight edge: a line (of infinite length through two points) or a line segment (between two points).
- Constructions using a compass: a circle (with center point and through a point) or an arc (with center, radius point, and between two points).
- Changing the snap radius to intersections.
- Showing and hiding intersection points.
- Showing and hiding the axes.
- Inserting points at specific coordinates.
- A layer system, where lines can be added to layers and layers hidden.
- An undo/redo stack.

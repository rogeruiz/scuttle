# Scuttle

![Scuttle](scuttle.jpg)

This project contains the code to generate `mermaid` flow charts and diagrams
using the Draft US Web Design Standards.

## Installation

```shell
npm install
```

## Development

```shell
npm start
```

Then open http://localhost:1337/


### Application Sketch

- `source/diagrams` contains all the mermaid files to render
- each file is loaded into the `source/render.html` and blocked within a div
- `mermaid.API` is called to activate each section
- `click` and `hover` events are set for each part of the diagram, but do
  nothing but set console messages __todo__
- `source/stylesheets` files are compiled and saved to `public/render.css`
- `source/javascript` files are compiled and saved to `public/render.js`
- once each diagram is loaded into `source/render.html`

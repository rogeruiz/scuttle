import mermaid from 'mermaid';

console.log( 'mermaid', mermaid.version() );

// TODO This is very ugly, but for now it renders. This SVG creation should be
// done in the build step.
let ss = document.getElementById( 'js-stage' );

mermaid.mermaidAPI.render(
  'js-stage',
  ss.textContent.replace(/\n/g, '\\n'),
  function ( c ) {
    ss.innerHTML = c;
  }
);

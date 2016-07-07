import gulp from 'gulp';
import gutil from 'gulp-util';
import notifier from 'node-notifier';
import http from 'http';
import fs from 'fs';
import path from 'path';
import connect from 'connect';
import serveStatic from 'serve-static';
import Mustache from 'mustache';
import through from 'through2';

/*
 * @name default
 * @description The default gulp task.
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'default', ( done ) => {
  done();
} );

/*
 * @name stylesheets
 * @desc Compiles the Sass files under `source/stylesheets`.
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'stylesheets', ( done ) => {
  done();
} );

/*
 * @name javascript
 * @desc Bundles and transpiles the JavaScript files under `source/javascript`.
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'javascript', ( done ) => {
  done();
} );

/*
 * @name render
 * @desc Creates the final HTML page for rendering the mermaid diagrams.
 * @see { @link stylesheets }
 * @see { @link javascript }
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'render', [ 'stylesheets', 'javascript' ], ( done ) => {

  const htmlTemplate = fs.readFileSync( './source/html/render.html', 'utf-8' );

  return gulp.src( './source/diagrams/**.mmd' )
    .pipe( renderMermaid( htmlTemplate ) )
    .pipe( gulp.dest( './public/' ) );

} );

/*
 * @name server
 * @desc Runs a preview server for local development of mermaid diagrams.
 * @see { @link render }
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'server', [ 'render' ], ( done ) => {

  var port = 1337;

  connect()
    .use( serveStatic( path.join( __dirname, '/public' ), { fallthrough: false } ) )
    .use( ( error, request, response, next ) => {
      if ( error ) {
        logError( 'server', `${ error.statusCode } ${ error.path }` );
      }
      next();
    } )
    .listen( port, () => {
      logMessage( 'server', `Site available at http://localhost:${ port }/` );
    } );

} );

/*
 * @name export
 * @desc Exports a PNG using PhantomJS of the mermaid diagrams.
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'export', ( done ) => { done(); } );

/*
 * @name notify
 * @desc Wrapper around node-notify
 * @see { @link logError }
 * @see { @link logMessage }
 * @param { string } title - The title of the notification.
 * @param { string } message - The message fo the notification.
 */
const notify = ( title, message ) => {
  notifier.notify( {
    title: title,
    message: message,
    icon: 'scuttle-icon.jpg',
  } );
};

/*
 * @name logMessage
 * @desc Wrapper for gulp-util for logging task messages.
 * @param { string } task - The name of the task.
 * @param { string } message - The message for the task.
 */
const logMessage = ( task, message ) => {
  notify( task, message );
  gutil.log(
    gutil.colors.cyan( task ),
    gutil.colors.yellow( message )
  );
};

/*
 * @name logError
 * @desc Wrapper for gulp-util for logging task errors.
 * @param { string } task - The name of the task.
 * @param { string } message - The error message for the task.
 */
const logError = ( task, message ) => {
  notify( task, message );
  gutil.log(
    gutil.colors.red( task ),
    gutil.colors.yellow( message )
  );
};

/*
 * @name renderMermaid
 * @desc Gulp plugin for rendering Mermaid diagrams within a Mustache template
 * @param { string } template - The Mustache template
 * @return { stream } - A node stream wrapped in through2
 */
const renderMermaid = function ( template ) {

  const PluginError = gutil.PluginError;

  if ( ! template ) {
    throw new PluginError( 'gulp-render-mermaid', 'Missing a Mustache template.' );
  }

  template = new Buffer( template );

  return through.obj( function ( file, encoding, callback )  {

    if ( file.isNull() ) {
      return callback( null, file );
    }

    let fileName = path.basename( file.path, '.mmd' );
    logMessage( 'gulp-render-mermaid', `Processing ${ file.path }`);

    if ( file.isBuffer() ) {
      file.contents = new Buffer( Mustache.render( template.toString(), {
        'diagram-title': fileName,
        'diagram-contents': file.contents.toString(),
      } ) );

      file.path = path.join( path.dirname( file.path ), `${ fileName }.html` );
    }
    if ( file.isStream() ) {
      this.emit( 'error', new PluginError( 'gulp-render-mermaid', 'Streaming not supported' ) );
      return callback( null, file );
    }

    callback( null, file );
  } );
};

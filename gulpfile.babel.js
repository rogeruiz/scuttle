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
import browserify from 'browserify';
import source from 'vinyl-source-stream';
import buffer from 'vinyl-buffer';
import babelify from 'babelify';
import sass from 'gulp-sass';

/*
 * @name default
 * @description The default gulp task.
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'default', ( done ) => {
  done();
} );

gulp.task( 'copy:fonts', () => {
  return gulp.src( './node_modules/uswds/dist/fonts/**/*' )
    .pipe( gulp.dest( './public/fonts' ) );
} );

/*
 * @name stylesheets
 * @desc Compiles the Sass files under `source/stylesheets`.
 */
gulp.task( 'stylesheets', [ 'copy:fonts' ], () => {

  return gulp.src( './source/styles/render.scss' )
    .pipe( sass().on( 'error', sass.logError ) )
    .pipe( gulp.dest( './public' ) );

} );

/*
 * @name javascript
 * @desc Bundles and transpiles the JavaScript files under `source/javascript`.
 */
gulp.task( 'javascript', () => {

  const bundler = browserify( {
    entries: './source/javascript/start.js',
    debug: false,
    transform: [
      [
        babelify,
        {
          presets: [
            'es2015',
            'es2016'
          ],
        }
      ],
    ]
  } );

  return bundler.bundle()
    .pipe( source( 'render.js' ) )
    .pipe( gulp.dest( './public' ) );

} );

/*
 * @name render
 * @desc Creates the final HTML page for rendering the mermaid diagrams.
 * @see { @link stylesheets }
 * @see { @link javascript }
 */
gulp.task( 'render', [ 'stylesheets', 'javascript' ], () => {

  const htmlTemplate = fs.readFileSync( './source/html/render.html', 'utf-8' );

  return gulp.src( './source/diagrams/**.mmd' )
    .pipe( renderMermaid( htmlTemplate ) )
    .pipe( gulp.dest( './public/' ) );

} );

/*
 * @name render:list
 * @desc Render a list of all the diagrams available under `source/diagrams`.
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'render:list', ( done ) => {

  const htmlTemplate = fs.readFileSync( './source/html/index.html', 'utf-8' );

  fs.readdir( './source/diagrams', ( error, files ) => {

    let diagrams = files.map( ( file ) => {
      return {
        href: `${ path.basename( file, '.mmd' ) }.html`,
        name: file,
      };
    } );

    var listView = Mustache.render( htmlTemplate, {
      'diagram-list': diagrams,
    } );

    fs.writeFile( './public/index.html', listView, ( error ) => {
      if ( ! error ) {
        done();
      }
    } );

  } );

} );

/*
 * @name server
 * @desc Runs a preview server for local development of mermaid diagrams.
 * @see { @link render }
 * @param { function } done - Callback that signals the task is complete.
 */
gulp.task( 'server', [ 'render', 'render:list' ], ( done ) => {

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
 * @name logData
 * @desc Wrapper for gulp-util for logging task data.
 * @param { string } task - The name of the task.
 * @param { string } data - The data for the task.
 */
const logData = ( task, data ) => {
  gutil.log(
    gutil.colors.cyan( task ),
    gutil.colors.white( data )
  );
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

const ffi = require( 'ffi-napi' );
const ref = require( 'ref-napi' );
const is_dev = require( 'electron-is-dev' );

const cdylib_dir = is_dev ? "target/debug/" : "target/release/";
const cdylib_ext = 
    process.platform.startsWith( 'win' ) ? 'dll'
  : process.platform === 'linux' ? 'so'
  : process.platform === 'darwin' ? 'dylib'
  : null
  ;

const get_cdylib_path =
  ( libname ) => `${cdylib_dir}${libname}${((_=cdylib_ext) === null ) ? '' : `.${_}` }`;

const lib_path = get_cdylib_path( 'lib' );

const f32 = ref.types.float;
const str = ref.types.CString;
const pchar = ref.refType(ref.types.char);

const lib = 
  ffi.Library
  ( lib_path
  , { 'example_add': [ f32, [ f32, f32 ] ] 
    , 'example_concat': [ pchar, [ str, str ] ]
    , 'example_concat_free': [ ref.types.void, [ pchar ] ]
  }
  );

lib.example_concat_with_free = 
  ( a, b ) =>
  {
    const buffer = lib.example_concat( a, b );
    try
    { return ref.readCString( buffer ); }
    finally
    { lib.example_concat_free( buffer ); }
  }


// Export to an electron client ( App.js and etc. )
window.preload =
  { is_dev: is_dev
  , lib: lib

  // Note: Uncomment if you wanto use `electron.remote` in App.js or elsewhere
  // , remote: require( 'electron' ).remote;
};

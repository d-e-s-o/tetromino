
        mod __gl_imports {
            pub use std::mem;
            pub use std::os::raw;
        }
    

        pub mod types {
            #![allow(non_camel_case_types, non_snake_case, dead_code, missing_copy_implementations)]
    
// Common types from OpenGL 1.1
pub type GLenum = super::__gl_imports::raw::c_uint;
pub type GLboolean = super::__gl_imports::raw::c_uchar;
pub type GLbitfield = super::__gl_imports::raw::c_uint;
pub type GLvoid = super::__gl_imports::raw::c_void;
pub type GLbyte = super::__gl_imports::raw::c_char;
pub type GLshort = super::__gl_imports::raw::c_short;
pub type GLint = super::__gl_imports::raw::c_int;
pub type GLclampx = super::__gl_imports::raw::c_int;
pub type GLubyte = super::__gl_imports::raw::c_uchar;
pub type GLushort = super::__gl_imports::raw::c_ushort;
pub type GLuint = super::__gl_imports::raw::c_uint;
pub type GLsizei = super::__gl_imports::raw::c_int;
pub type GLfloat = super::__gl_imports::raw::c_float;
pub type GLclampf = super::__gl_imports::raw::c_float;
pub type GLdouble = super::__gl_imports::raw::c_double;
pub type GLclampd = super::__gl_imports::raw::c_double;
pub type GLeglImageOES = *const super::__gl_imports::raw::c_void;
pub type GLchar = super::__gl_imports::raw::c_char;
pub type GLcharARB = super::__gl_imports::raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const super::__gl_imports::raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = super::__gl_imports::raw::c_uint;

pub type GLhalfARB = super::__gl_imports::raw::c_ushort;
pub type GLhalf = super::__gl_imports::raw::c_ushort;

// Must be 32 bits
pub type GLfixed = GLint;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLint64 = i64;
pub type GLuint64 = u64;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub enum __GLsync {}
pub type GLsync = *const __GLsync;

// compatible with OpenCL cl_context
pub enum _cl_context {}
pub enum _cl_event {}

pub type GLDEBUGPROC = Option<extern "system" fn(source: GLenum,
                                                 gltype: GLenum,
                                                 id: GLuint,
                                                 severity: GLenum,
                                                 length: GLsizei,
                                                 message: *const GLchar,
                                                 userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCARB = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLDEBUGPROCKHR = Option<extern "system" fn(source: GLenum,
                                                    gltype: GLenum,
                                                    id: GLuint,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;

// GLES 1 types
// "pub type GLclampx = i32;",

// GLES 1/2 types (tagged for GLES 1)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)
// "pub type GLbyte = i8;",
// "pub type GLubyte = u8;",
// "pub type GLfloat = GLfloat;",
// "pub type GLclampf = GLfloat;",
// "pub type GLfixed = i32;",
// "pub type GLint64 = i64;",
// "pub type GLuint64 = u64;",
// "pub type GLint64EXT = i64;",
// "pub type GLuint64EXT = u64;",
// "pub type GLintptr = intptr_t;",
// "pub type GLsizeiptr = ssize_t;",

// GLES 2 types (none currently)

// Vendor extension types
pub type GLDEBUGPROCAMD = Option<extern "system" fn(id: GLuint,
                                                    category: GLenum,
                                                    severity: GLenum,
                                                    length: GLsizei,
                                                    message: *const GLchar,
                                                    userParam: *mut super::__gl_imports::raw::c_void)>;
pub type GLhalfNV = super::__gl_imports::raw::c_ushort;
pub type GLvdpauSurfaceNV = GLintptr;


        }
    
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM: types::GLenum = 0x0100;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_ALPHA_BITS: types::GLenum = 0x0D5B;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_BLUE_BITS: types::GLenum = 0x0D5A;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_BUFFER_BIT: types::GLenum = 0x00000200;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_CLEAR_VALUE: types::GLenum = 0x0B80;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_GREEN_BITS: types::GLenum = 0x0D59;
#[allow(dead_code, non_upper_case_globals)] pub const ACCUM_RED_BITS: types::GLenum = 0x0D58;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ADD: types::GLenum = 0x0104;
#[allow(dead_code, non_upper_case_globals)] pub const ADD_SIGNED: types::GLenum = 0x8574;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_POINT_SIZE_RANGE: types::GLenum = 0x846D;
#[allow(dead_code, non_upper_case_globals)] pub const ALL_ATTRIB_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA12: types::GLenum = 0x803D;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA16: types::GLenum = 0x803E;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA4: types::GLenum = 0x803B;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA8: types::GLenum = 0x803C;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_BIAS: types::GLenum = 0x0D1D;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_BITS: types::GLenum = 0x0D55;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_SCALE: types::GLenum = 0x0D1C;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST: types::GLenum = 0x0BC0;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST_FUNC: types::GLenum = 0x0BC1;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA_TEST_REF: types::GLenum = 0x0BC2;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const AMBIENT: types::GLenum = 0x1200;
#[allow(dead_code, non_upper_case_globals)] pub const AMBIENT_AND_DIFFUSE: types::GLenum = 0x1602;
#[allow(dead_code, non_upper_case_globals)] pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)] pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)] pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)] pub const ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB0;
#[allow(dead_code, non_upper_case_globals)] pub const AUTO_NORMAL: types::GLenum = 0x0D80;
#[allow(dead_code, non_upper_case_globals)] pub const AUX0: types::GLenum = 0x0409;
#[allow(dead_code, non_upper_case_globals)] pub const AUX1: types::GLenum = 0x040A;
#[allow(dead_code, non_upper_case_globals)] pub const AUX2: types::GLenum = 0x040B;
#[allow(dead_code, non_upper_case_globals)] pub const AUX3: types::GLenum = 0x040C;
#[allow(dead_code, non_upper_case_globals)] pub const AUX_BUFFERS: types::GLenum = 0x0C00;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_LEFT: types::GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_RIGHT: types::GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)] pub const BGR: types::GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA: types::GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)] pub const BITMAP: types::GLenum = 0x1A00;
#[allow(dead_code, non_upper_case_globals)] pub const BITMAP_TOKEN: types::GLenum = 0x0704;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_BIAS: types::GLenum = 0x0D1B;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_BITS: types::GLenum = 0x0D54;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_SCALE: types::GLenum = 0x0D1A;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const C3F_V3F: types::GLenum = 0x2A24;
#[allow(dead_code, non_upper_case_globals)] pub const C4F_N3F_V3F: types::GLenum = 0x2A26;
#[allow(dead_code, non_upper_case_globals)] pub const C4UB_V2F: types::GLenum = 0x2A22;
#[allow(dead_code, non_upper_case_globals)] pub const C4UB_V3F: types::GLenum = 0x2A23;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP: types::GLenum = 0x2900;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_BORDER: types::GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_ACTIVE_TEXTURE: types::GLenum = 0x84E1;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_ALL_ATTRIB_BITS: types::GLenum = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0BB1;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_PIXEL_STORE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIENT_VERTEX_ARRAY_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_PLANE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const COEFF: types::GLenum = 0x0A00;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY: types::GLenum = 0x8076;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_POINTER: types::GLenum = 0x8090;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_SIZE: types::GLenum = 0x8081;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_STRIDE: types::GLenum = 0x8083;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ARRAY_TYPE: types::GLenum = 0x8082;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_INDEX: types::GLenum = 0x1900;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_INDEXES: types::GLenum = 0x1603;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_MATERIAL: types::GLenum = 0x0B57;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_MATERIAL_FACE: types::GLenum = 0x0B55;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_MATERIAL_PARAMETER: types::GLenum = 0x0B56;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMBINE: types::GLenum = 0x8570;
#[allow(dead_code, non_upper_case_globals)] pub const COMBINE_ALPHA: types::GLenum = 0x8572;
#[allow(dead_code, non_upper_case_globals)] pub const COMBINE_RGB: types::GLenum = 0x8571;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE: types::GLenum = 0x1300;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_AND_EXECUTE: types::GLenum = 0x1301;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_ALPHA: types::GLenum = 0x84E9;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_INTENSITY: types::GLenum = 0x84EC;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_LUMINANCE: types::GLenum = 0x84EA;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_LUMINANCE_ALPHA: types::GLenum = 0x84EB;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB: types::GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA: types::GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT: types::GLenum = 0x8576;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ATTENUATION: types::GLenum = 0x1207;
#[allow(dead_code, non_upper_case_globals)] pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_PIXEL_TOKEN: types::GLenum = 0x0706;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_COLOR: types::GLenum = 0x0B00;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_INDEX: types::GLenum = 0x0B01;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_NORMAL: types::GLenum = 0x0B02;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_COLOR: types::GLenum = 0x0B04;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_DISTANCE: types::GLenum = 0x0B09;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_INDEX: types::GLenum = 0x0B05;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_POSITION: types::GLenum = 0x0B07;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_POSITION_VALID: types::GLenum = 0x0B08;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_RASTER_TEXTURE_COORDS: types::GLenum = 0x0B06;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_TEXTURE_COORDS: types::GLenum = 0x0B03;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DECAL: types::GLenum = 0x2101;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BIAS: types::GLenum = 0x0D1F;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BITS: types::GLenum = 0x0D56;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_SCALE: types::GLenum = 0x0D1E;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DIFFUSE: types::GLenum = 0x1201;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DOMAIN: types::GLenum = 0x0A02;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DOT3_RGB: types::GLenum = 0x86AE;
#[allow(dead_code, non_upper_case_globals)] pub const DOT3_RGBA: types::GLenum = 0x86AF;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE: types::GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLEBUFFER: types::GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER: types::GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_PIXEL_TOKEN: types::GLenum = 0x0705;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG: types::GLenum = 0x0B43;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG_ARRAY: types::GLenum = 0x8079;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG_ARRAY_POINTER: types::GLenum = 0x8093;
#[allow(dead_code, non_upper_case_globals)] pub const EDGE_FLAG_ARRAY_STRIDE: types::GLenum = 0x808C;
#[allow(dead_code, non_upper_case_globals)] pub const EMISSION: types::GLenum = 0x1600;
#[allow(dead_code, non_upper_case_globals)] pub const ENABLE_BIT: types::GLenum = 0x00002000;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)] pub const EVAL_BIT: types::GLenum = 0x00010000;
#[allow(dead_code, non_upper_case_globals)] pub const EXP: types::GLenum = 0x0800;
#[allow(dead_code, non_upper_case_globals)] pub const EXP2: types::GLenum = 0x0801;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const EYE_LINEAR: types::GLenum = 0x2400;
#[allow(dead_code, non_upper_case_globals)] pub const EYE_PLANE: types::GLenum = 0x2502;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK: types::GLenum = 0x1C01;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK_BUFFER_POINTER: types::GLenum = 0x0DF0;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK_BUFFER_SIZE: types::GLenum = 0x0DF1;
#[allow(dead_code, non_upper_case_globals)] pub const FEEDBACK_BUFFER_TYPE: types::GLenum = 0x0DF2;
#[allow(dead_code, non_upper_case_globals)] pub const FILL: types::GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)] pub const FLAT: types::GLenum = 0x1D00;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FOG: types::GLenum = 0x0B60;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_BIT: types::GLenum = 0x00000080;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_COLOR: types::GLenum = 0x0B66;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_DENSITY: types::GLenum = 0x0B62;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_END: types::GLenum = 0x0B64;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_HINT: types::GLenum = 0x0C54;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_INDEX: types::GLenum = 0x0B61;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_MODE: types::GLenum = 0x0B65;
#[allow(dead_code, non_upper_case_globals)] pub const FOG_START: types::GLenum = 0x0B63;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_LEFT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_RIGHT: types::GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_BIAS: types::GLenum = 0x0D19;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_BITS: types::GLenum = 0x0D53;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_SCALE: types::GLenum = 0x0D18;
#[allow(dead_code, non_upper_case_globals)] pub const HINT_BIT: types::GLenum = 0x00008000;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY: types::GLenum = 0x8077;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY_POINTER: types::GLenum = 0x8091;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY_STRIDE: types::GLenum = 0x8086;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_ARRAY_TYPE: types::GLenum = 0x8085;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_BITS: types::GLenum = 0x0D51;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_CLEAR_VALUE: types::GLenum = 0x0C20;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_LOGIC_OP: types::GLenum = 0x0BF1;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_MODE: types::GLenum = 0x0C30;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_OFFSET: types::GLenum = 0x0D13;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_SHIFT: types::GLenum = 0x0D12;
#[allow(dead_code, non_upper_case_globals)] pub const INDEX_WRITEMASK: types::GLenum = 0x0C21;
#[allow(dead_code, non_upper_case_globals)] pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY: types::GLenum = 0x8049;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY12: types::GLenum = 0x804C;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY16: types::GLenum = 0x804D;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY4: types::GLenum = 0x804A;
#[allow(dead_code, non_upper_case_globals)] pub const INTENSITY8: types::GLenum = 0x804B;
#[allow(dead_code, non_upper_case_globals)] pub const INTERPOLATE: types::GLenum = 0x8575;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LEFT: types::GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT0: types::GLenum = 0x4000;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT1: types::GLenum = 0x4001;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT2: types::GLenum = 0x4002;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT3: types::GLenum = 0x4003;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT4: types::GLenum = 0x4004;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT5: types::GLenum = 0x4005;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT6: types::GLenum = 0x4006;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT7: types::GLenum = 0x4007;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHTING: types::GLenum = 0x0B50;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHTING_BIT: types::GLenum = 0x00000040;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_AMBIENT: types::GLenum = 0x0B53;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_COLOR_CONTROL: types::GLenum = 0x81F8;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_LOCAL_VIEWER: types::GLenum = 0x0B51;
#[allow(dead_code, non_upper_case_globals)] pub const LIGHT_MODEL_TWO_SIDE: types::GLenum = 0x0B52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE: types::GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_ATTENUATION: types::GLenum = 0x1208;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_BIT: types::GLenum = 0x00000004;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_RESET_TOKEN: types::GLenum = 0x0707;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STIPPLE: types::GLenum = 0x0B24;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STIPPLE_PATTERN: types::GLenum = 0x0B25;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STIPPLE_REPEAT: types::GLenum = 0x0B26;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_TOKEN: types::GLenum = 0x0702;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_BASE: types::GLenum = 0x0B32;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_BIT: types::GLenum = 0x00020000;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_INDEX: types::GLenum = 0x0B33;
#[allow(dead_code, non_upper_case_globals)] pub const LIST_MODE: types::GLenum = 0x0B30;
#[allow(dead_code, non_upper_case_globals)] pub const LOAD: types::GLenum = 0x0101;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP: types::GLenum = 0x0BF1;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE: types::GLenum = 0x1909;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE12: types::GLenum = 0x8041;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE12_ALPHA12: types::GLenum = 0x8047;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE12_ALPHA4: types::GLenum = 0x8046;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE16: types::GLenum = 0x8042;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE16_ALPHA16: types::GLenum = 0x8048;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE4: types::GLenum = 0x803F;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE4_ALPHA4: types::GLenum = 0x8043;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE6_ALPHA2: types::GLenum = 0x8044;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE8: types::GLenum = 0x8040;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE8_ALPHA8: types::GLenum = 0x8045;
#[allow(dead_code, non_upper_case_globals)] pub const LUMINANCE_ALPHA: types::GLenum = 0x190A;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_COLOR_4: types::GLenum = 0x0D90;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_GRID_DOMAIN: types::GLenum = 0x0DD0;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_GRID_SEGMENTS: types::GLenum = 0x0DD1;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_INDEX: types::GLenum = 0x0D91;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_NORMAL: types::GLenum = 0x0D92;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_1: types::GLenum = 0x0D93;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_2: types::GLenum = 0x0D94;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_3: types::GLenum = 0x0D95;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_TEXTURE_COORD_4: types::GLenum = 0x0D96;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_VERTEX_3: types::GLenum = 0x0D97;
#[allow(dead_code, non_upper_case_globals)] pub const MAP1_VERTEX_4: types::GLenum = 0x0D98;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_COLOR_4: types::GLenum = 0x0DB0;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_GRID_DOMAIN: types::GLenum = 0x0DD2;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_GRID_SEGMENTS: types::GLenum = 0x0DD3;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_INDEX: types::GLenum = 0x0DB1;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_NORMAL: types::GLenum = 0x0DB2;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_1: types::GLenum = 0x0DB3;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_2: types::GLenum = 0x0DB4;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_3: types::GLenum = 0x0DB5;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_TEXTURE_COORD_4: types::GLenum = 0x0DB6;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_VERTEX_3: types::GLenum = 0x0DB7;
#[allow(dead_code, non_upper_case_globals)] pub const MAP2_VERTEX_4: types::GLenum = 0x0DB8;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_COLOR: types::GLenum = 0x0D10;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_STENCIL: types::GLenum = 0x0D11;
#[allow(dead_code, non_upper_case_globals)] pub const MATRIX_MODE: types::GLenum = 0x0BA0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_3D_TEXTURE_SIZE: types::GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D35;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIENT_ATTRIB_STACK_DEPTH: types::GLenum = 0x0D3B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIP_PLANES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_INDICES: types::GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_VERTICES: types::GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_EVAL_ORDER: types::GLenum = 0x0D30;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LIGHTS: types::GLenum = 0x0D31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_LIST_NESTING: types::GLenum = 0x0B31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_MODELVIEW_STACK_DEPTH: types::GLenum = 0x0D36;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_NAME_STACK_DEPTH: types::GLenum = 0x0D37;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PIXEL_MAP_TABLE: types::GLenum = 0x0D34;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROJECTION_STACK_DEPTH: types::GLenum = 0x0D38;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_STACK_DEPTH: types::GLenum = 0x0D39;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_UNITS: types::GLenum = 0x84E2;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW: types::GLenum = 0x1700;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW_MATRIX: types::GLenum = 0x0BA6;
#[allow(dead_code, non_upper_case_globals)] pub const MODELVIEW_STACK_DEPTH: types::GLenum = 0x0BA3;
#[allow(dead_code, non_upper_case_globals)] pub const MODULATE: types::GLenum = 0x2100;
#[allow(dead_code, non_upper_case_globals)] pub const MULT: types::GLenum = 0x0103;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE: types::GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE_BIT: types::GLenum = 0x20000000;
#[allow(dead_code, non_upper_case_globals)] pub const N3F_V3F: types::GLenum = 0x2A25;
#[allow(dead_code, non_upper_case_globals)] pub const NAME_STACK_DEPTH: types::GLenum = 0x0D70;
#[allow(dead_code, non_upper_case_globals)] pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)] pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)] pub const NORMALIZE: types::GLenum = 0x0BA1;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY: types::GLenum = 0x8075;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_POINTER: types::GLenum = 0x808F;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_STRIDE: types::GLenum = 0x807F;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_ARRAY_TYPE: types::GLenum = 0x807E;
#[allow(dead_code, non_upper_case_globals)] pub const NORMAL_MAP: types::GLenum = 0x8511;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_LINEAR: types::GLenum = 0x2401;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_PLANE: types::GLenum = 0x2501;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND0_ALPHA: types::GLenum = 0x8598;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND0_RGB: types::GLenum = 0x8590;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND1_ALPHA: types::GLenum = 0x8599;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND1_RGB: types::GLenum = 0x8591;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND2_ALPHA: types::GLenum = 0x859A;
#[allow(dead_code, non_upper_case_globals)] pub const OPERAND2_RGB: types::GLenum = 0x8592;
#[allow(dead_code, non_upper_case_globals)] pub const OR: types::GLenum = 0x1507;
#[allow(dead_code, non_upper_case_globals)] pub const ORDER: types::GLenum = 0x0A01;
#[allow(dead_code, non_upper_case_globals)] pub const OR_INVERTED: types::GLenum = 0x150D;
#[allow(dead_code, non_upper_case_globals)] pub const OR_REVERSE: types::GLenum = 0x150B;
#[allow(dead_code, non_upper_case_globals)] pub const OUT_OF_MEMORY: types::GLenum = 0x0505;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ALIGNMENT: types::GLenum = 0x0D05;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_IMAGE_HEIGHT: types::GLenum = 0x806C;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_LSB_FIRST: types::GLenum = 0x0D01;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_ROW_LENGTH: types::GLenum = 0x0D02;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_IMAGES: types::GLenum = 0x806B;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_PIXELS: types::GLenum = 0x0D04;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SKIP_ROWS: types::GLenum = 0x0D03;
#[allow(dead_code, non_upper_case_globals)] pub const PACK_SWAP_BYTES: types::GLenum = 0x0D00;
#[allow(dead_code, non_upper_case_globals)] pub const PASS_THROUGH_TOKEN: types::GLenum = 0x0700;
#[allow(dead_code, non_upper_case_globals)] pub const PERSPECTIVE_CORRECTION_HINT: types::GLenum = 0x0C50;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_A_TO_A: types::GLenum = 0x0C79;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_A_TO_A_SIZE: types::GLenum = 0x0CB9;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_B_TO_B: types::GLenum = 0x0C78;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_B_TO_B_SIZE: types::GLenum = 0x0CB8;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_G_TO_G: types::GLenum = 0x0C77;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_G_TO_G_SIZE: types::GLenum = 0x0CB7;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_A: types::GLenum = 0x0C75;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_A_SIZE: types::GLenum = 0x0CB5;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_B: types::GLenum = 0x0C74;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_B_SIZE: types::GLenum = 0x0CB4;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_G: types::GLenum = 0x0C73;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_G_SIZE: types::GLenum = 0x0CB3;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_I: types::GLenum = 0x0C70;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_I_SIZE: types::GLenum = 0x0CB0;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_R: types::GLenum = 0x0C72;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_I_TO_R_SIZE: types::GLenum = 0x0CB2;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_R_TO_R: types::GLenum = 0x0C76;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_R_TO_R_SIZE: types::GLenum = 0x0CB6;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_S_TO_S: types::GLenum = 0x0C71;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MAP_S_TO_S_SIZE: types::GLenum = 0x0CB1;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_MODE_BIT: types::GLenum = 0x00000020;
#[allow(dead_code, non_upper_case_globals)] pub const POINT: types::GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SMOOTH: types::GLenum = 0x0B10;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SMOOTH_HINT: types::GLenum = 0x0C51;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_TOKEN: types::GLenum = 0x0701;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON: types::GLenum = 0x0009;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_BIT: types::GLenum = 0x00000008;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_MODE: types::GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH: types::GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_STIPPLE: types::GLenum = 0x0B42;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_STIPPLE_BIT: types::GLenum = 0x00000010;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_TOKEN: types::GLenum = 0x0703;
#[allow(dead_code, non_upper_case_globals)] pub const POSITION: types::GLenum = 0x1203;
#[allow(dead_code, non_upper_case_globals)] pub const PREVIOUS: types::GLenum = 0x8578;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMARY_COLOR: types::GLenum = 0x8577;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION: types::GLenum = 0x1701;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION_MATRIX: types::GLenum = 0x0BA7;
#[allow(dead_code, non_upper_case_globals)] pub const PROJECTION_STACK_DEPTH: types::GLenum = 0x0BA4;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_3D: types::GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP: types::GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)] pub const Q: types::GLenum = 0x2003;
#[allow(dead_code, non_upper_case_globals)] pub const QUADRATIC_ATTENUATION: types::GLenum = 0x1209;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS: types::GLenum = 0x0007;
#[allow(dead_code, non_upper_case_globals)] pub const QUAD_STRIP: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const R: types::GLenum = 0x2002;
#[allow(dead_code, non_upper_case_globals)] pub const R3_G3_B2: types::GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)] pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)] pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)] pub const RED_BIAS: types::GLenum = 0x0D15;
#[allow(dead_code, non_upper_case_globals)] pub const RED_BITS: types::GLenum = 0x0D52;
#[allow(dead_code, non_upper_case_globals)] pub const RED_SCALE: types::GLenum = 0x0D14;
#[allow(dead_code, non_upper_case_globals)] pub const REFLECTION_MAP: types::GLenum = 0x8512;
#[allow(dead_code, non_upper_case_globals)] pub const RENDER: types::GLenum = 0x1C00;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const RENDER_MODE: types::GLenum = 0x0C40;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RESCALE_NORMAL: types::GLenum = 0x803A;
#[allow(dead_code, non_upper_case_globals)] pub const RETURN: types::GLenum = 0x0102;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10: types::GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)] pub const RGB12: types::GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16: types::GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)] pub const RGB4: types::GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5: types::GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA12: types::GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16: types::GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA2: types::GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA_MODE: types::GLenum = 0x0C31;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_SCALE: types::GLenum = 0x8573;
#[allow(dead_code, non_upper_case_globals)] pub const RIGHT: types::GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)] pub const S: types::GLenum = 0x2000;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BIT: types::GLenum = 0x00080000;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SELECT: types::GLenum = 0x1C02;
#[allow(dead_code, non_upper_case_globals)] pub const SELECTION_BUFFER_POINTER: types::GLenum = 0x0DF3;
#[allow(dead_code, non_upper_case_globals)] pub const SELECTION_BUFFER_SIZE: types::GLenum = 0x0DF4;
#[allow(dead_code, non_upper_case_globals)] pub const SEPARATE_SPECULAR_COLOR: types::GLenum = 0x81FA;
#[allow(dead_code, non_upper_case_globals)] pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADE_MODEL: types::GLenum = 0x0B54;
#[allow(dead_code, non_upper_case_globals)] pub const SHININESS: types::GLenum = 0x1601;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SINGLE_COLOR: types::GLenum = 0x81F9;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH: types::GLenum = 0x1D01;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const SOURCE0_ALPHA: types::GLenum = 0x8588;
#[allow(dead_code, non_upper_case_globals)] pub const SOURCE0_RGB: types::GLenum = 0x8580;
#[allow(dead_code, non_upper_case_globals)] pub const SOURCE1_ALPHA: types::GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)] pub const SOURCE1_RGB: types::GLenum = 0x8581;
#[allow(dead_code, non_upper_case_globals)] pub const SOURCE2_ALPHA: types::GLenum = 0x858A;
#[allow(dead_code, non_upper_case_globals)] pub const SOURCE2_RGB: types::GLenum = 0x8582;
#[allow(dead_code, non_upper_case_globals)] pub const SPECULAR: types::GLenum = 0x1202;
#[allow(dead_code, non_upper_case_globals)] pub const SPHERE_MAP: types::GLenum = 0x2402;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_CUTOFF: types::GLenum = 0x1206;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_DIRECTION: types::GLenum = 0x1204;
#[allow(dead_code, non_upper_case_globals)] pub const SPOT_EXPONENT: types::GLenum = 0x1205;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_OVERFLOW: types::GLenum = 0x0503;
#[allow(dead_code, non_upper_case_globals)] pub const STACK_UNDERFLOW: types::GLenum = 0x0504;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BITS: types::GLenum = 0x0D57;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STEREO: types::GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const SUBTRACT: types::GLenum = 0x84E7;
#[allow(dead_code, non_upper_case_globals)] pub const T: types::GLenum = 0x2001;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_C3F_V3F: types::GLenum = 0x2A2A;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_C4F_N3F_V3F: types::GLenum = 0x2A2C;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_C4UB_V3F: types::GLenum = 0x2A29;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_N3F_V3F: types::GLenum = 0x2A2B;
#[allow(dead_code, non_upper_case_globals)] pub const T2F_V3F: types::GLenum = 0x2A27;
#[allow(dead_code, non_upper_case_globals)] pub const T4F_C4F_N3F_V4F: types::GLenum = 0x2A2D;
#[allow(dead_code, non_upper_case_globals)] pub const T4F_V4F: types::GLenum = 0x2A28;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE: types::GLenum = 0x1702;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE0: types::GLenum = 0x84C0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE1: types::GLenum = 0x84C1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE10: types::GLenum = 0x84CA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE11: types::GLenum = 0x84CB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE12: types::GLenum = 0x84CC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE13: types::GLenum = 0x84CD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE14: types::GLenum = 0x84CE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE15: types::GLenum = 0x84CF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE16: types::GLenum = 0x84D0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE17: types::GLenum = 0x84D1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE18: types::GLenum = 0x84D2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE19: types::GLenum = 0x84D3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE2: types::GLenum = 0x84C2;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE20: types::GLenum = 0x84D4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE21: types::GLenum = 0x84D5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE22: types::GLenum = 0x84D6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE23: types::GLenum = 0x84D7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE24: types::GLenum = 0x84D8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE25: types::GLenum = 0x84D9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE26: types::GLenum = 0x84DA;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE27: types::GLenum = 0x84DB;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE28: types::GLenum = 0x84DC;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE29: types::GLenum = 0x84DD;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE3: types::GLenum = 0x84C3;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE30: types::GLenum = 0x84DE;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE31: types::GLenum = 0x84DF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE4: types::GLenum = 0x84C4;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE5: types::GLenum = 0x84C5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE6: types::GLenum = 0x84C6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE7: types::GLenum = 0x84C7;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE8: types::GLenum = 0x84C8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE9: types::GLenum = 0x84C9;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D: types::GLenum = 0x0DE0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_3D: types::GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BASE_LEVEL: types::GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_3D: types::GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BIT: types::GLenum = 0x00040000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER: types::GLenum = 0x1005;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPONENTS: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED: types::GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_IMAGE_SIZE: types::GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSION_HINT: types::GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY: types::GLenum = 0x8078;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_POINTER: types::GLenum = 0x8092;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_SIZE: types::GLenum = 0x8088;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_STRIDE: types::GLenum = 0x808A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COORD_ARRAY_TYPE: types::GLenum = 0x8089;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH: types::GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV: types::GLenum = 0x2300;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV_COLOR: types::GLenum = 0x2201;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ENV_MODE: types::GLenum = 0x2200;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_MODE: types::GLenum = 0x2500;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_Q: types::GLenum = 0x0C63;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_R: types::GLenum = 0x0C62;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_S: types::GLenum = 0x0C60;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GEN_T: types::GLenum = 0x0C61;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTENSITY_SIZE: types::GLenum = 0x8061;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_LUMINANCE_SIZE: types::GLenum = 0x8060;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MATRIX: types::GLenum = 0x0BA8;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LEVEL: types::GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LOD: types::GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_LOD: types::GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_PRIORITY: types::GLenum = 0x8066;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RESIDENT: types::GLenum = 0x8067;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STACK_DEPTH: types::GLenum = 0x0BA5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_R: types::GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_BIT: types::GLenum = 0x00001000;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPOSE_COLOR_MATRIX: types::GLenum = 0x84E6;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPOSE_MODELVIEW_MATRIX: types::GLenum = 0x84E3;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPOSE_PROJECTION_MATRIX: types::GLenum = 0x84E4;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSPOSE_TEXTURE_MATRIX: types::GLenum = 0x84E5;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_IMAGE_HEIGHT: types::GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_IMAGES: types::GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_2_3_3_REV: types::GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_3_3_2: types::GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10_10_10_2: types::GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_2_10_10_10_REV: types::GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8: types::GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8_REV: types::GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_1_5_5_5_REV: types::GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4_REV: types::GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5_REV: types::GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)] pub const V2F: types::GLenum = 0x2A20;
#[allow(dead_code, non_upper_case_globals)] pub const V3F: types::GLenum = 0x2A21;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY: types::GLenum = 0x8074;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_POINTER: types::GLenum = 0x808E;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_SIZE: types::GLenum = 0x807A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_STRIDE: types::GLenum = 0x807C;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_TYPE: types::GLenum = 0x807B;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT_BIT: types::GLenum = 0x00000800;
#[allow(dead_code, non_upper_case_globals)] pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const ZOOM_X: types::GLenum = 0x0D16;
#[allow(dead_code, non_upper_case_globals)] pub const ZOOM_Y: types::GLenum = 0x0D17;

        #[allow(non_snake_case, unused_variables, dead_code)]
        extern "system" {
#[link_name="glAccum"]
            pub fn Accum(op: types::GLenum, value: types::GLfloat) -> ();
#[link_name="glActiveTexture"]
            pub fn ActiveTexture(texture: types::GLenum) -> ();
#[link_name="glAlphaFunc"]
            pub fn AlphaFunc(func: types::GLenum, ref_: types::GLfloat) -> ();
#[link_name="glAreTexturesResident"]
            pub fn AreTexturesResident(n: types::GLsizei, textures: *const types::GLuint, residences: *mut types::GLboolean) -> types::GLboolean;
#[link_name="glArrayElement"]
            pub fn ArrayElement(i: types::GLint) -> ();
#[link_name="glBegin"]
            pub fn Begin(mode: types::GLenum) -> ();
#[link_name="glBindTexture"]
            pub fn BindTexture(target: types::GLenum, texture: types::GLuint) -> ();
#[link_name="glBitmap"]
            pub fn Bitmap(width: types::GLsizei, height: types::GLsizei, xorig: types::GLfloat, yorig: types::GLfloat, xmove: types::GLfloat, ymove: types::GLfloat, bitmap: *const types::GLubyte) -> ();
#[link_name="glBlendFunc"]
            pub fn BlendFunc(sfactor: types::GLenum, dfactor: types::GLenum) -> ();
#[link_name="glCallList"]
            pub fn CallList(list: types::GLuint) -> ();
#[link_name="glCallLists"]
            pub fn CallLists(n: types::GLsizei, type_: types::GLenum, lists: *const __gl_imports::raw::c_void) -> ();
#[link_name="glClear"]
            pub fn Clear(mask: types::GLbitfield) -> ();
#[link_name="glClearAccum"]
            pub fn ClearAccum(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> ();
#[link_name="glClearColor"]
            pub fn ClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> ();
#[link_name="glClearDepth"]
            pub fn ClearDepth(depth: types::GLdouble) -> ();
#[link_name="glClearIndex"]
            pub fn ClearIndex(c: types::GLfloat) -> ();
#[link_name="glClearStencil"]
            pub fn ClearStencil(s: types::GLint) -> ();
#[link_name="glClientActiveTexture"]
            pub fn ClientActiveTexture(texture: types::GLenum) -> ();
#[link_name="glClipPlane"]
            pub fn ClipPlane(plane: types::GLenum, equation: *const types::GLdouble) -> ();
#[link_name="glColor3b"]
            pub fn Color3b(red: types::GLbyte, green: types::GLbyte, blue: types::GLbyte) -> ();
#[link_name="glColor3bv"]
            pub fn Color3bv(v: *const types::GLbyte) -> ();
#[link_name="glColor3d"]
            pub fn Color3d(red: types::GLdouble, green: types::GLdouble, blue: types::GLdouble) -> ();
#[link_name="glColor3dv"]
            pub fn Color3dv(v: *const types::GLdouble) -> ();
#[link_name="glColor3f"]
            pub fn Color3f(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat) -> ();
#[link_name="glColor3fv"]
            pub fn Color3fv(v: *const types::GLfloat) -> ();
#[link_name="glColor3i"]
            pub fn Color3i(red: types::GLint, green: types::GLint, blue: types::GLint) -> ();
#[link_name="glColor3iv"]
            pub fn Color3iv(v: *const types::GLint) -> ();
#[link_name="glColor3s"]
            pub fn Color3s(red: types::GLshort, green: types::GLshort, blue: types::GLshort) -> ();
#[link_name="glColor3sv"]
            pub fn Color3sv(v: *const types::GLshort) -> ();
#[link_name="glColor3ub"]
            pub fn Color3ub(red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte) -> ();
#[link_name="glColor3ubv"]
            pub fn Color3ubv(v: *const types::GLubyte) -> ();
#[link_name="glColor3ui"]
            pub fn Color3ui(red: types::GLuint, green: types::GLuint, blue: types::GLuint) -> ();
#[link_name="glColor3uiv"]
            pub fn Color3uiv(v: *const types::GLuint) -> ();
#[link_name="glColor3us"]
            pub fn Color3us(red: types::GLushort, green: types::GLushort, blue: types::GLushort) -> ();
#[link_name="glColor3usv"]
            pub fn Color3usv(v: *const types::GLushort) -> ();
#[link_name="glColor4b"]
            pub fn Color4b(red: types::GLbyte, green: types::GLbyte, blue: types::GLbyte, alpha: types::GLbyte) -> ();
#[link_name="glColor4bv"]
            pub fn Color4bv(v: *const types::GLbyte) -> ();
#[link_name="glColor4d"]
            pub fn Color4d(red: types::GLdouble, green: types::GLdouble, blue: types::GLdouble, alpha: types::GLdouble) -> ();
#[link_name="glColor4dv"]
            pub fn Color4dv(v: *const types::GLdouble) -> ();
#[link_name="glColor4f"]
            pub fn Color4f(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> ();
#[link_name="glColor4fv"]
            pub fn Color4fv(v: *const types::GLfloat) -> ();
#[link_name="glColor4i"]
            pub fn Color4i(red: types::GLint, green: types::GLint, blue: types::GLint, alpha: types::GLint) -> ();
#[link_name="glColor4iv"]
            pub fn Color4iv(v: *const types::GLint) -> ();
#[link_name="glColor4s"]
            pub fn Color4s(red: types::GLshort, green: types::GLshort, blue: types::GLshort, alpha: types::GLshort) -> ();
#[link_name="glColor4sv"]
            pub fn Color4sv(v: *const types::GLshort) -> ();
#[link_name="glColor4ub"]
            pub fn Color4ub(red: types::GLubyte, green: types::GLubyte, blue: types::GLubyte, alpha: types::GLubyte) -> ();
#[link_name="glColor4ubv"]
            pub fn Color4ubv(v: *const types::GLubyte) -> ();
#[link_name="glColor4ui"]
            pub fn Color4ui(red: types::GLuint, green: types::GLuint, blue: types::GLuint, alpha: types::GLuint) -> ();
#[link_name="glColor4uiv"]
            pub fn Color4uiv(v: *const types::GLuint) -> ();
#[link_name="glColor4us"]
            pub fn Color4us(red: types::GLushort, green: types::GLushort, blue: types::GLushort, alpha: types::GLushort) -> ();
#[link_name="glColor4usv"]
            pub fn Color4usv(v: *const types::GLushort) -> ();
#[link_name="glColorMask"]
            pub fn ColorMask(red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> ();
#[link_name="glColorMaterial"]
            pub fn ColorMaterial(face: types::GLenum, mode: types::GLenum) -> ();
#[link_name="glColorPointer"]
            pub fn ColorPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCompressedTexImage1D"]
            pub fn CompressedTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCompressedTexImage2D"]
            pub fn CompressedTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCompressedTexImage3D"]
            pub fn CompressedTexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCompressedTexSubImage1D"]
            pub fn CompressedTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCompressedTexSubImage2D"]
            pub fn CompressedTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCompressedTexSubImage3D"]
            pub fn CompressedTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, imageSize: types::GLsizei, data: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCopyPixels"]
            pub fn CopyPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, type_: types::GLenum) -> ();
#[link_name="glCopyTexImage1D"]
            pub fn CopyTexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, border: types::GLint) -> ();
#[link_name="glCopyTexImage2D"]
            pub fn CopyTexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLenum, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint) -> ();
#[link_name="glCopyTexSubImage1D"]
            pub fn CopyTexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei) -> ();
#[link_name="glCopyTexSubImage2D"]
            pub fn CopyTexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> ();
#[link_name="glCopyTexSubImage3D"]
            pub fn CopyTexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> ();
#[link_name="glCullFace"]
            pub fn CullFace(mode: types::GLenum) -> ();
#[link_name="glDeleteLists"]
            pub fn DeleteLists(list: types::GLuint, range: types::GLsizei) -> ();
#[link_name="glDeleteTextures"]
            pub fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) -> ();
#[link_name="glDepthFunc"]
            pub fn DepthFunc(func: types::GLenum) -> ();
#[link_name="glDepthMask"]
            pub fn DepthMask(flag: types::GLboolean) -> ();
#[link_name="glDepthRange"]
            pub fn DepthRange(n: types::GLdouble, f: types::GLdouble) -> ();
#[link_name="glDisable"]
            pub fn Disable(cap: types::GLenum) -> ();
#[link_name="glDisableClientState"]
            pub fn DisableClientState(array: types::GLenum) -> ();
#[link_name="glDrawArrays"]
            pub fn DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> ();
#[link_name="glDrawBuffer"]
            pub fn DrawBuffer(buf: types::GLenum) -> ();
#[link_name="glDrawElements"]
            pub fn DrawElements(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> ();
#[link_name="glDrawPixels"]
            pub fn DrawPixels(width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glDrawRangeElements"]
            pub fn DrawRangeElements(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> ();
#[link_name="glEdgeFlag"]
            pub fn EdgeFlag(flag: types::GLboolean) -> ();
#[link_name="glEdgeFlagPointer"]
            pub fn EdgeFlagPointer(stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glEdgeFlagv"]
            pub fn EdgeFlagv(flag: *const types::GLboolean) -> ();
#[link_name="glEnable"]
            pub fn Enable(cap: types::GLenum) -> ();
#[link_name="glEnableClientState"]
            pub fn EnableClientState(array: types::GLenum) -> ();
#[link_name="glEnd"]
            pub fn End() -> ();
#[link_name="glEndList"]
            pub fn EndList() -> ();
#[link_name="glEvalCoord1d"]
            pub fn EvalCoord1d(u: types::GLdouble) -> ();
#[link_name="glEvalCoord1dv"]
            pub fn EvalCoord1dv(u: *const types::GLdouble) -> ();
#[link_name="glEvalCoord1f"]
            pub fn EvalCoord1f(u: types::GLfloat) -> ();
#[link_name="glEvalCoord1fv"]
            pub fn EvalCoord1fv(u: *const types::GLfloat) -> ();
#[link_name="glEvalCoord2d"]
            pub fn EvalCoord2d(u: types::GLdouble, v: types::GLdouble) -> ();
#[link_name="glEvalCoord2dv"]
            pub fn EvalCoord2dv(u: *const types::GLdouble) -> ();
#[link_name="glEvalCoord2f"]
            pub fn EvalCoord2f(u: types::GLfloat, v: types::GLfloat) -> ();
#[link_name="glEvalCoord2fv"]
            pub fn EvalCoord2fv(u: *const types::GLfloat) -> ();
#[link_name="glEvalMesh1"]
            pub fn EvalMesh1(mode: types::GLenum, i1: types::GLint, i2: types::GLint) -> ();
#[link_name="glEvalMesh2"]
            pub fn EvalMesh2(mode: types::GLenum, i1: types::GLint, i2: types::GLint, j1: types::GLint, j2: types::GLint) -> ();
#[link_name="glEvalPoint1"]
            pub fn EvalPoint1(i: types::GLint) -> ();
#[link_name="glEvalPoint2"]
            pub fn EvalPoint2(i: types::GLint, j: types::GLint) -> ();
#[link_name="glFeedbackBuffer"]
            pub fn FeedbackBuffer(size: types::GLsizei, type_: types::GLenum, buffer: *mut types::GLfloat) -> ();
#[link_name="glFinish"]
            pub fn Finish() -> ();
#[link_name="glFlush"]
            pub fn Flush() -> ();
#[link_name="glFogf"]
            pub fn Fogf(pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glFogfv"]
            pub fn Fogfv(pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glFogi"]
            pub fn Fogi(pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glFogiv"]
            pub fn Fogiv(pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glFrontFace"]
            pub fn FrontFace(mode: types::GLenum) -> ();
#[link_name="glFrustum"]
            pub fn Frustum(left: types::GLdouble, right: types::GLdouble, bottom: types::GLdouble, top: types::GLdouble, zNear: types::GLdouble, zFar: types::GLdouble) -> ();
#[link_name="glGenLists"]
            pub fn GenLists(range: types::GLsizei) -> types::GLuint;
#[link_name="glGenTextures"]
            pub fn GenTextures(n: types::GLsizei, textures: *mut types::GLuint) -> ();
#[link_name="glGetBooleanv"]
            pub fn GetBooleanv(pname: types::GLenum, data: *mut types::GLboolean) -> ();
#[link_name="glGetClipPlane"]
            pub fn GetClipPlane(plane: types::GLenum, equation: *mut types::GLdouble) -> ();
#[link_name="glGetCompressedTexImage"]
            pub fn GetCompressedTexImage(target: types::GLenum, level: types::GLint, img: *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetDoublev"]
            pub fn GetDoublev(pname: types::GLenum, data: *mut types::GLdouble) -> ();
#[link_name="glGetError"]
            pub fn GetError() -> types::GLenum;
#[link_name="glGetFloatv"]
            pub fn GetFloatv(pname: types::GLenum, data: *mut types::GLfloat) -> ();
#[link_name="glGetIntegerv"]
            pub fn GetIntegerv(pname: types::GLenum, data: *mut types::GLint) -> ();
#[link_name="glGetLightfv"]
            pub fn GetLightfv(light: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetLightiv"]
            pub fn GetLightiv(light: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetMapdv"]
            pub fn GetMapdv(target: types::GLenum, query: types::GLenum, v: *mut types::GLdouble) -> ();
#[link_name="glGetMapfv"]
            pub fn GetMapfv(target: types::GLenum, query: types::GLenum, v: *mut types::GLfloat) -> ();
#[link_name="glGetMapiv"]
            pub fn GetMapiv(target: types::GLenum, query: types::GLenum, v: *mut types::GLint) -> ();
#[link_name="glGetMaterialfv"]
            pub fn GetMaterialfv(face: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetMaterialiv"]
            pub fn GetMaterialiv(face: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetPixelMapfv"]
            pub fn GetPixelMapfv(map: types::GLenum, values: *mut types::GLfloat) -> ();
#[link_name="glGetPixelMapuiv"]
            pub fn GetPixelMapuiv(map: types::GLenum, values: *mut types::GLuint) -> ();
#[link_name="glGetPixelMapusv"]
            pub fn GetPixelMapusv(map: types::GLenum, values: *mut types::GLushort) -> ();
#[link_name="glGetPointerv"]
            pub fn GetPointerv(pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetPolygonStipple"]
            pub fn GetPolygonStipple(mask: *mut types::GLubyte) -> ();
#[link_name="glGetString"]
            pub fn GetString(name: types::GLenum) -> *const types::GLubyte;
#[link_name="glGetTexEnvfv"]
            pub fn GetTexEnvfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetTexEnviv"]
            pub fn GetTexEnviv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetTexGendv"]
            pub fn GetTexGendv(coord: types::GLenum, pname: types::GLenum, params: *mut types::GLdouble) -> ();
#[link_name="glGetTexGenfv"]
            pub fn GetTexGenfv(coord: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetTexGeniv"]
            pub fn GetTexGeniv(coord: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetTexImage"]
            pub fn GetTexImage(target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetTexLevelParameterfv"]
            pub fn GetTexLevelParameterfv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetTexLevelParameteriv"]
            pub fn GetTexLevelParameteriv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetTexParameterfv"]
            pub fn GetTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetTexParameteriv"]
            pub fn GetTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glHint"]
            pub fn Hint(target: types::GLenum, mode: types::GLenum) -> ();
#[link_name="glIndexMask"]
            pub fn IndexMask(mask: types::GLuint) -> ();
#[link_name="glIndexPointer"]
            pub fn IndexPointer(type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glIndexd"]
            pub fn Indexd(c: types::GLdouble) -> ();
#[link_name="glIndexdv"]
            pub fn Indexdv(c: *const types::GLdouble) -> ();
#[link_name="glIndexf"]
            pub fn Indexf(c: types::GLfloat) -> ();
#[link_name="glIndexfv"]
            pub fn Indexfv(c: *const types::GLfloat) -> ();
#[link_name="glIndexi"]
            pub fn Indexi(c: types::GLint) -> ();
#[link_name="glIndexiv"]
            pub fn Indexiv(c: *const types::GLint) -> ();
#[link_name="glIndexs"]
            pub fn Indexs(c: types::GLshort) -> ();
#[link_name="glIndexsv"]
            pub fn Indexsv(c: *const types::GLshort) -> ();
#[link_name="glIndexub"]
            pub fn Indexub(c: types::GLubyte) -> ();
#[link_name="glIndexubv"]
            pub fn Indexubv(c: *const types::GLubyte) -> ();
#[link_name="glInitNames"]
            pub fn InitNames() -> ();
#[link_name="glInterleavedArrays"]
            pub fn InterleavedArrays(format: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glIsEnabled"]
            pub fn IsEnabled(cap: types::GLenum) -> types::GLboolean;
#[link_name="glIsList"]
            pub fn IsList(list: types::GLuint) -> types::GLboolean;
#[link_name="glIsTexture"]
            pub fn IsTexture(texture: types::GLuint) -> types::GLboolean;
#[link_name="glLightModelf"]
            pub fn LightModelf(pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glLightModelfv"]
            pub fn LightModelfv(pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glLightModeli"]
            pub fn LightModeli(pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glLightModeliv"]
            pub fn LightModeliv(pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glLightf"]
            pub fn Lightf(light: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glLightfv"]
            pub fn Lightfv(light: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glLighti"]
            pub fn Lighti(light: types::GLenum, pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glLightiv"]
            pub fn Lightiv(light: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glLineStipple"]
            pub fn LineStipple(factor: types::GLint, pattern: types::GLushort) -> ();
#[link_name="glLineWidth"]
            pub fn LineWidth(width: types::GLfloat) -> ();
#[link_name="glListBase"]
            pub fn ListBase(base: types::GLuint) -> ();
#[link_name="glLoadIdentity"]
            pub fn LoadIdentity() -> ();
#[link_name="glLoadMatrixd"]
            pub fn LoadMatrixd(m: *const types::GLdouble) -> ();
#[link_name="glLoadMatrixf"]
            pub fn LoadMatrixf(m: *const types::GLfloat) -> ();
#[link_name="glLoadName"]
            pub fn LoadName(name: types::GLuint) -> ();
#[link_name="glLoadTransposeMatrixd"]
            pub fn LoadTransposeMatrixd(m: *const types::GLdouble) -> ();
#[link_name="glLoadTransposeMatrixf"]
            pub fn LoadTransposeMatrixf(m: *const types::GLfloat) -> ();
#[link_name="glLogicOp"]
            pub fn LogicOp(opcode: types::GLenum) -> ();
#[link_name="glMap1d"]
            pub fn Map1d(target: types::GLenum, u1: types::GLdouble, u2: types::GLdouble, stride: types::GLint, order: types::GLint, points: *const types::GLdouble) -> ();
#[link_name="glMap1f"]
            pub fn Map1f(target: types::GLenum, u1: types::GLfloat, u2: types::GLfloat, stride: types::GLint, order: types::GLint, points: *const types::GLfloat) -> ();
#[link_name="glMap2d"]
            pub fn Map2d(target: types::GLenum, u1: types::GLdouble, u2: types::GLdouble, ustride: types::GLint, uorder: types::GLint, v1: types::GLdouble, v2: types::GLdouble, vstride: types::GLint, vorder: types::GLint, points: *const types::GLdouble) -> ();
#[link_name="glMap2f"]
            pub fn Map2f(target: types::GLenum, u1: types::GLfloat, u2: types::GLfloat, ustride: types::GLint, uorder: types::GLint, v1: types::GLfloat, v2: types::GLfloat, vstride: types::GLint, vorder: types::GLint, points: *const types::GLfloat) -> ();
#[link_name="glMapGrid1d"]
            pub fn MapGrid1d(un: types::GLint, u1: types::GLdouble, u2: types::GLdouble) -> ();
#[link_name="glMapGrid1f"]
            pub fn MapGrid1f(un: types::GLint, u1: types::GLfloat, u2: types::GLfloat) -> ();
#[link_name="glMapGrid2d"]
            pub fn MapGrid2d(un: types::GLint, u1: types::GLdouble, u2: types::GLdouble, vn: types::GLint, v1: types::GLdouble, v2: types::GLdouble) -> ();
#[link_name="glMapGrid2f"]
            pub fn MapGrid2f(un: types::GLint, u1: types::GLfloat, u2: types::GLfloat, vn: types::GLint, v1: types::GLfloat, v2: types::GLfloat) -> ();
#[link_name="glMaterialf"]
            pub fn Materialf(face: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glMaterialfv"]
            pub fn Materialfv(face: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glMateriali"]
            pub fn Materiali(face: types::GLenum, pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glMaterialiv"]
            pub fn Materialiv(face: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glMatrixMode"]
            pub fn MatrixMode(mode: types::GLenum) -> ();
#[link_name="glMultMatrixd"]
            pub fn MultMatrixd(m: *const types::GLdouble) -> ();
#[link_name="glMultMatrixf"]
            pub fn MultMatrixf(m: *const types::GLfloat) -> ();
#[link_name="glMultTransposeMatrixd"]
            pub fn MultTransposeMatrixd(m: *const types::GLdouble) -> ();
#[link_name="glMultTransposeMatrixf"]
            pub fn MultTransposeMatrixf(m: *const types::GLfloat) -> ();
#[link_name="glMultiTexCoord1d"]
            pub fn MultiTexCoord1d(target: types::GLenum, s: types::GLdouble) -> ();
#[link_name="glMultiTexCoord1dv"]
            pub fn MultiTexCoord1dv(target: types::GLenum, v: *const types::GLdouble) -> ();
#[link_name="glMultiTexCoord1f"]
            pub fn MultiTexCoord1f(target: types::GLenum, s: types::GLfloat) -> ();
#[link_name="glMultiTexCoord1fv"]
            pub fn MultiTexCoord1fv(target: types::GLenum, v: *const types::GLfloat) -> ();
#[link_name="glMultiTexCoord1i"]
            pub fn MultiTexCoord1i(target: types::GLenum, s: types::GLint) -> ();
#[link_name="glMultiTexCoord1iv"]
            pub fn MultiTexCoord1iv(target: types::GLenum, v: *const types::GLint) -> ();
#[link_name="glMultiTexCoord1s"]
            pub fn MultiTexCoord1s(target: types::GLenum, s: types::GLshort) -> ();
#[link_name="glMultiTexCoord1sv"]
            pub fn MultiTexCoord1sv(target: types::GLenum, v: *const types::GLshort) -> ();
#[link_name="glMultiTexCoord2d"]
            pub fn MultiTexCoord2d(target: types::GLenum, s: types::GLdouble, t: types::GLdouble) -> ();
#[link_name="glMultiTexCoord2dv"]
            pub fn MultiTexCoord2dv(target: types::GLenum, v: *const types::GLdouble) -> ();
#[link_name="glMultiTexCoord2f"]
            pub fn MultiTexCoord2f(target: types::GLenum, s: types::GLfloat, t: types::GLfloat) -> ();
#[link_name="glMultiTexCoord2fv"]
            pub fn MultiTexCoord2fv(target: types::GLenum, v: *const types::GLfloat) -> ();
#[link_name="glMultiTexCoord2i"]
            pub fn MultiTexCoord2i(target: types::GLenum, s: types::GLint, t: types::GLint) -> ();
#[link_name="glMultiTexCoord2iv"]
            pub fn MultiTexCoord2iv(target: types::GLenum, v: *const types::GLint) -> ();
#[link_name="glMultiTexCoord2s"]
            pub fn MultiTexCoord2s(target: types::GLenum, s: types::GLshort, t: types::GLshort) -> ();
#[link_name="glMultiTexCoord2sv"]
            pub fn MultiTexCoord2sv(target: types::GLenum, v: *const types::GLshort) -> ();
#[link_name="glMultiTexCoord3d"]
            pub fn MultiTexCoord3d(target: types::GLenum, s: types::GLdouble, t: types::GLdouble, r: types::GLdouble) -> ();
#[link_name="glMultiTexCoord3dv"]
            pub fn MultiTexCoord3dv(target: types::GLenum, v: *const types::GLdouble) -> ();
#[link_name="glMultiTexCoord3f"]
            pub fn MultiTexCoord3f(target: types::GLenum, s: types::GLfloat, t: types::GLfloat, r: types::GLfloat) -> ();
#[link_name="glMultiTexCoord3fv"]
            pub fn MultiTexCoord3fv(target: types::GLenum, v: *const types::GLfloat) -> ();
#[link_name="glMultiTexCoord3i"]
            pub fn MultiTexCoord3i(target: types::GLenum, s: types::GLint, t: types::GLint, r: types::GLint) -> ();
#[link_name="glMultiTexCoord3iv"]
            pub fn MultiTexCoord3iv(target: types::GLenum, v: *const types::GLint) -> ();
#[link_name="glMultiTexCoord3s"]
            pub fn MultiTexCoord3s(target: types::GLenum, s: types::GLshort, t: types::GLshort, r: types::GLshort) -> ();
#[link_name="glMultiTexCoord3sv"]
            pub fn MultiTexCoord3sv(target: types::GLenum, v: *const types::GLshort) -> ();
#[link_name="glMultiTexCoord4d"]
            pub fn MultiTexCoord4d(target: types::GLenum, s: types::GLdouble, t: types::GLdouble, r: types::GLdouble, q: types::GLdouble) -> ();
#[link_name="glMultiTexCoord4dv"]
            pub fn MultiTexCoord4dv(target: types::GLenum, v: *const types::GLdouble) -> ();
#[link_name="glMultiTexCoord4f"]
            pub fn MultiTexCoord4f(target: types::GLenum, s: types::GLfloat, t: types::GLfloat, r: types::GLfloat, q: types::GLfloat) -> ();
#[link_name="glMultiTexCoord4fv"]
            pub fn MultiTexCoord4fv(target: types::GLenum, v: *const types::GLfloat) -> ();
#[link_name="glMultiTexCoord4i"]
            pub fn MultiTexCoord4i(target: types::GLenum, s: types::GLint, t: types::GLint, r: types::GLint, q: types::GLint) -> ();
#[link_name="glMultiTexCoord4iv"]
            pub fn MultiTexCoord4iv(target: types::GLenum, v: *const types::GLint) -> ();
#[link_name="glMultiTexCoord4s"]
            pub fn MultiTexCoord4s(target: types::GLenum, s: types::GLshort, t: types::GLshort, r: types::GLshort, q: types::GLshort) -> ();
#[link_name="glMultiTexCoord4sv"]
            pub fn MultiTexCoord4sv(target: types::GLenum, v: *const types::GLshort) -> ();
#[link_name="glNewList"]
            pub fn NewList(list: types::GLuint, mode: types::GLenum) -> ();
#[link_name="glNormal3b"]
            pub fn Normal3b(nx: types::GLbyte, ny: types::GLbyte, nz: types::GLbyte) -> ();
#[link_name="glNormal3bv"]
            pub fn Normal3bv(v: *const types::GLbyte) -> ();
#[link_name="glNormal3d"]
            pub fn Normal3d(nx: types::GLdouble, ny: types::GLdouble, nz: types::GLdouble) -> ();
#[link_name="glNormal3dv"]
            pub fn Normal3dv(v: *const types::GLdouble) -> ();
#[link_name="glNormal3f"]
            pub fn Normal3f(nx: types::GLfloat, ny: types::GLfloat, nz: types::GLfloat) -> ();
#[link_name="glNormal3fv"]
            pub fn Normal3fv(v: *const types::GLfloat) -> ();
#[link_name="glNormal3i"]
            pub fn Normal3i(nx: types::GLint, ny: types::GLint, nz: types::GLint) -> ();
#[link_name="glNormal3iv"]
            pub fn Normal3iv(v: *const types::GLint) -> ();
#[link_name="glNormal3s"]
            pub fn Normal3s(nx: types::GLshort, ny: types::GLshort, nz: types::GLshort) -> ();
#[link_name="glNormal3sv"]
            pub fn Normal3sv(v: *const types::GLshort) -> ();
#[link_name="glNormalPointer"]
            pub fn NormalPointer(type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glOrtho"]
            pub fn Ortho(left: types::GLdouble, right: types::GLdouble, bottom: types::GLdouble, top: types::GLdouble, zNear: types::GLdouble, zFar: types::GLdouble) -> ();
#[link_name="glPassThrough"]
            pub fn PassThrough(token: types::GLfloat) -> ();
#[link_name="glPixelMapfv"]
            pub fn PixelMapfv(map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLfloat) -> ();
#[link_name="glPixelMapuiv"]
            pub fn PixelMapuiv(map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLuint) -> ();
#[link_name="glPixelMapusv"]
            pub fn PixelMapusv(map: types::GLenum, mapsize: types::GLsizei, values: *const types::GLushort) -> ();
#[link_name="glPixelStoref"]
            pub fn PixelStoref(pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glPixelStorei"]
            pub fn PixelStorei(pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glPixelTransferf"]
            pub fn PixelTransferf(pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glPixelTransferi"]
            pub fn PixelTransferi(pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glPixelZoom"]
            pub fn PixelZoom(xfactor: types::GLfloat, yfactor: types::GLfloat) -> ();
#[link_name="glPointSize"]
            pub fn PointSize(size: types::GLfloat) -> ();
#[link_name="glPolygonMode"]
            pub fn PolygonMode(face: types::GLenum, mode: types::GLenum) -> ();
#[link_name="glPolygonOffset"]
            pub fn PolygonOffset(factor: types::GLfloat, units: types::GLfloat) -> ();
#[link_name="glPolygonStipple"]
            pub fn PolygonStipple(mask: *const types::GLubyte) -> ();
#[link_name="glPopAttrib"]
            pub fn PopAttrib() -> ();
#[link_name="glPopClientAttrib"]
            pub fn PopClientAttrib() -> ();
#[link_name="glPopMatrix"]
            pub fn PopMatrix() -> ();
#[link_name="glPopName"]
            pub fn PopName() -> ();
#[link_name="glPrioritizeTextures"]
            pub fn PrioritizeTextures(n: types::GLsizei, textures: *const types::GLuint, priorities: *const types::GLfloat) -> ();
#[link_name="glPushAttrib"]
            pub fn PushAttrib(mask: types::GLbitfield) -> ();
#[link_name="glPushClientAttrib"]
            pub fn PushClientAttrib(mask: types::GLbitfield) -> ();
#[link_name="glPushMatrix"]
            pub fn PushMatrix() -> ();
#[link_name="glPushName"]
            pub fn PushName(name: types::GLuint) -> ();
#[link_name="glRasterPos2d"]
            pub fn RasterPos2d(x: types::GLdouble, y: types::GLdouble) -> ();
#[link_name="glRasterPos2dv"]
            pub fn RasterPos2dv(v: *const types::GLdouble) -> ();
#[link_name="glRasterPos2f"]
            pub fn RasterPos2f(x: types::GLfloat, y: types::GLfloat) -> ();
#[link_name="glRasterPos2fv"]
            pub fn RasterPos2fv(v: *const types::GLfloat) -> ();
#[link_name="glRasterPos2i"]
            pub fn RasterPos2i(x: types::GLint, y: types::GLint) -> ();
#[link_name="glRasterPos2iv"]
            pub fn RasterPos2iv(v: *const types::GLint) -> ();
#[link_name="glRasterPos2s"]
            pub fn RasterPos2s(x: types::GLshort, y: types::GLshort) -> ();
#[link_name="glRasterPos2sv"]
            pub fn RasterPos2sv(v: *const types::GLshort) -> ();
#[link_name="glRasterPos3d"]
            pub fn RasterPos3d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> ();
#[link_name="glRasterPos3dv"]
            pub fn RasterPos3dv(v: *const types::GLdouble) -> ();
#[link_name="glRasterPos3f"]
            pub fn RasterPos3f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> ();
#[link_name="glRasterPos3fv"]
            pub fn RasterPos3fv(v: *const types::GLfloat) -> ();
#[link_name="glRasterPos3i"]
            pub fn RasterPos3i(x: types::GLint, y: types::GLint, z: types::GLint) -> ();
#[link_name="glRasterPos3iv"]
            pub fn RasterPos3iv(v: *const types::GLint) -> ();
#[link_name="glRasterPos3s"]
            pub fn RasterPos3s(x: types::GLshort, y: types::GLshort, z: types::GLshort) -> ();
#[link_name="glRasterPos3sv"]
            pub fn RasterPos3sv(v: *const types::GLshort) -> ();
#[link_name="glRasterPos4d"]
            pub fn RasterPos4d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> ();
#[link_name="glRasterPos4dv"]
            pub fn RasterPos4dv(v: *const types::GLdouble) -> ();
#[link_name="glRasterPos4f"]
            pub fn RasterPos4f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> ();
#[link_name="glRasterPos4fv"]
            pub fn RasterPos4fv(v: *const types::GLfloat) -> ();
#[link_name="glRasterPos4i"]
            pub fn RasterPos4i(x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> ();
#[link_name="glRasterPos4iv"]
            pub fn RasterPos4iv(v: *const types::GLint) -> ();
#[link_name="glRasterPos4s"]
            pub fn RasterPos4s(x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> ();
#[link_name="glRasterPos4sv"]
            pub fn RasterPos4sv(v: *const types::GLshort) -> ();
#[link_name="glReadBuffer"]
            pub fn ReadBuffer(src: types::GLenum) -> ();
#[link_name="glReadPixels"]
            pub fn ReadPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> ();
#[link_name="glRectd"]
            pub fn Rectd(x1: types::GLdouble, y1: types::GLdouble, x2: types::GLdouble, y2: types::GLdouble) -> ();
#[link_name="glRectdv"]
            pub fn Rectdv(v1: *const types::GLdouble, v2: *const types::GLdouble) -> ();
#[link_name="glRectf"]
            pub fn Rectf(x1: types::GLfloat, y1: types::GLfloat, x2: types::GLfloat, y2: types::GLfloat) -> ();
#[link_name="glRectfv"]
            pub fn Rectfv(v1: *const types::GLfloat, v2: *const types::GLfloat) -> ();
#[link_name="glRecti"]
            pub fn Recti(x1: types::GLint, y1: types::GLint, x2: types::GLint, y2: types::GLint) -> ();
#[link_name="glRectiv"]
            pub fn Rectiv(v1: *const types::GLint, v2: *const types::GLint) -> ();
#[link_name="glRects"]
            pub fn Rects(x1: types::GLshort, y1: types::GLshort, x2: types::GLshort, y2: types::GLshort) -> ();
#[link_name="glRectsv"]
            pub fn Rectsv(v1: *const types::GLshort, v2: *const types::GLshort) -> ();
#[link_name="glRenderMode"]
            pub fn RenderMode(mode: types::GLenum) -> types::GLint;
#[link_name="glRotated"]
            pub fn Rotated(angle: types::GLdouble, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> ();
#[link_name="glRotatef"]
            pub fn Rotatef(angle: types::GLfloat, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> ();
#[link_name="glSampleCoverage"]
            pub fn SampleCoverage(value: types::GLfloat, invert: types::GLboolean) -> ();
#[link_name="glScaled"]
            pub fn Scaled(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> ();
#[link_name="glScalef"]
            pub fn Scalef(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> ();
#[link_name="glScissor"]
            pub fn Scissor(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> ();
#[link_name="glSelectBuffer"]
            pub fn SelectBuffer(size: types::GLsizei, buffer: *mut types::GLuint) -> ();
#[link_name="glShadeModel"]
            pub fn ShadeModel(mode: types::GLenum) -> ();
#[link_name="glStencilFunc"]
            pub fn StencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> ();
#[link_name="glStencilMask"]
            pub fn StencilMask(mask: types::GLuint) -> ();
#[link_name="glStencilOp"]
            pub fn StencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> ();
#[link_name="glTexCoord1d"]
            pub fn TexCoord1d(s: types::GLdouble) -> ();
#[link_name="glTexCoord1dv"]
            pub fn TexCoord1dv(v: *const types::GLdouble) -> ();
#[link_name="glTexCoord1f"]
            pub fn TexCoord1f(s: types::GLfloat) -> ();
#[link_name="glTexCoord1fv"]
            pub fn TexCoord1fv(v: *const types::GLfloat) -> ();
#[link_name="glTexCoord1i"]
            pub fn TexCoord1i(s: types::GLint) -> ();
#[link_name="glTexCoord1iv"]
            pub fn TexCoord1iv(v: *const types::GLint) -> ();
#[link_name="glTexCoord1s"]
            pub fn TexCoord1s(s: types::GLshort) -> ();
#[link_name="glTexCoord1sv"]
            pub fn TexCoord1sv(v: *const types::GLshort) -> ();
#[link_name="glTexCoord2d"]
            pub fn TexCoord2d(s: types::GLdouble, t: types::GLdouble) -> ();
#[link_name="glTexCoord2dv"]
            pub fn TexCoord2dv(v: *const types::GLdouble) -> ();
#[link_name="glTexCoord2f"]
            pub fn TexCoord2f(s: types::GLfloat, t: types::GLfloat) -> ();
#[link_name="glTexCoord2fv"]
            pub fn TexCoord2fv(v: *const types::GLfloat) -> ();
#[link_name="glTexCoord2i"]
            pub fn TexCoord2i(s: types::GLint, t: types::GLint) -> ();
#[link_name="glTexCoord2iv"]
            pub fn TexCoord2iv(v: *const types::GLint) -> ();
#[link_name="glTexCoord2s"]
            pub fn TexCoord2s(s: types::GLshort, t: types::GLshort) -> ();
#[link_name="glTexCoord2sv"]
            pub fn TexCoord2sv(v: *const types::GLshort) -> ();
#[link_name="glTexCoord3d"]
            pub fn TexCoord3d(s: types::GLdouble, t: types::GLdouble, r: types::GLdouble) -> ();
#[link_name="glTexCoord3dv"]
            pub fn TexCoord3dv(v: *const types::GLdouble) -> ();
#[link_name="glTexCoord3f"]
            pub fn TexCoord3f(s: types::GLfloat, t: types::GLfloat, r: types::GLfloat) -> ();
#[link_name="glTexCoord3fv"]
            pub fn TexCoord3fv(v: *const types::GLfloat) -> ();
#[link_name="glTexCoord3i"]
            pub fn TexCoord3i(s: types::GLint, t: types::GLint, r: types::GLint) -> ();
#[link_name="glTexCoord3iv"]
            pub fn TexCoord3iv(v: *const types::GLint) -> ();
#[link_name="glTexCoord3s"]
            pub fn TexCoord3s(s: types::GLshort, t: types::GLshort, r: types::GLshort) -> ();
#[link_name="glTexCoord3sv"]
            pub fn TexCoord3sv(v: *const types::GLshort) -> ();
#[link_name="glTexCoord4d"]
            pub fn TexCoord4d(s: types::GLdouble, t: types::GLdouble, r: types::GLdouble, q: types::GLdouble) -> ();
#[link_name="glTexCoord4dv"]
            pub fn TexCoord4dv(v: *const types::GLdouble) -> ();
#[link_name="glTexCoord4f"]
            pub fn TexCoord4f(s: types::GLfloat, t: types::GLfloat, r: types::GLfloat, q: types::GLfloat) -> ();
#[link_name="glTexCoord4fv"]
            pub fn TexCoord4fv(v: *const types::GLfloat) -> ();
#[link_name="glTexCoord4i"]
            pub fn TexCoord4i(s: types::GLint, t: types::GLint, r: types::GLint, q: types::GLint) -> ();
#[link_name="glTexCoord4iv"]
            pub fn TexCoord4iv(v: *const types::GLint) -> ();
#[link_name="glTexCoord4s"]
            pub fn TexCoord4s(s: types::GLshort, t: types::GLshort, r: types::GLshort, q: types::GLshort) -> ();
#[link_name="glTexCoord4sv"]
            pub fn TexCoord4sv(v: *const types::GLshort) -> ();
#[link_name="glTexCoordPointer"]
            pub fn TexCoordPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexEnvf"]
            pub fn TexEnvf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glTexEnvfv"]
            pub fn TexEnvfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glTexEnvi"]
            pub fn TexEnvi(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glTexEnviv"]
            pub fn TexEnviv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glTexGend"]
            pub fn TexGend(coord: types::GLenum, pname: types::GLenum, param: types::GLdouble) -> ();
#[link_name="glTexGendv"]
            pub fn TexGendv(coord: types::GLenum, pname: types::GLenum, params: *const types::GLdouble) -> ();
#[link_name="glTexGenf"]
            pub fn TexGenf(coord: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glTexGenfv"]
            pub fn TexGenfv(coord: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glTexGeni"]
            pub fn TexGeni(coord: types::GLenum, pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glTexGeniv"]
            pub fn TexGeniv(coord: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glTexImage1D"]
            pub fn TexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexImage2D"]
            pub fn TexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexImage3D"]
            pub fn TexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexParameterf"]
            pub fn TexParameterf(target: types::GLenum, pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glTexParameterfv"]
            pub fn TexParameterfv(target: types::GLenum, pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glTexParameteri"]
            pub fn TexParameteri(target: types::GLenum, pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glTexParameteriv"]
            pub fn TexParameteriv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glTexSubImage1D"]
            pub fn TexSubImage1D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, width: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexSubImage2D"]
            pub fn TexSubImage2D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexSubImage3D"]
            pub fn TexSubImage3D(target: types::GLenum, level: types::GLint, xoffset: types::GLint, yoffset: types::GLint, zoffset: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTranslated"]
            pub fn Translated(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> ();
#[link_name="glTranslatef"]
            pub fn Translatef(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> ();
#[link_name="glVertex2d"]
            pub fn Vertex2d(x: types::GLdouble, y: types::GLdouble) -> ();
#[link_name="glVertex2dv"]
            pub fn Vertex2dv(v: *const types::GLdouble) -> ();
#[link_name="glVertex2f"]
            pub fn Vertex2f(x: types::GLfloat, y: types::GLfloat) -> ();
#[link_name="glVertex2fv"]
            pub fn Vertex2fv(v: *const types::GLfloat) -> ();
#[link_name="glVertex2i"]
            pub fn Vertex2i(x: types::GLint, y: types::GLint) -> ();
#[link_name="glVertex2iv"]
            pub fn Vertex2iv(v: *const types::GLint) -> ();
#[link_name="glVertex2s"]
            pub fn Vertex2s(x: types::GLshort, y: types::GLshort) -> ();
#[link_name="glVertex2sv"]
            pub fn Vertex2sv(v: *const types::GLshort) -> ();
#[link_name="glVertex3d"]
            pub fn Vertex3d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> ();
#[link_name="glVertex3dv"]
            pub fn Vertex3dv(v: *const types::GLdouble) -> ();
#[link_name="glVertex3f"]
            pub fn Vertex3f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> ();
#[link_name="glVertex3fv"]
            pub fn Vertex3fv(v: *const types::GLfloat) -> ();
#[link_name="glVertex3i"]
            pub fn Vertex3i(x: types::GLint, y: types::GLint, z: types::GLint) -> ();
#[link_name="glVertex3iv"]
            pub fn Vertex3iv(v: *const types::GLint) -> ();
#[link_name="glVertex3s"]
            pub fn Vertex3s(x: types::GLshort, y: types::GLshort, z: types::GLshort) -> ();
#[link_name="glVertex3sv"]
            pub fn Vertex3sv(v: *const types::GLshort) -> ();
#[link_name="glVertex4d"]
            pub fn Vertex4d(x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> ();
#[link_name="glVertex4dv"]
            pub fn Vertex4dv(v: *const types::GLdouble) -> ();
#[link_name="glVertex4f"]
            pub fn Vertex4f(x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> ();
#[link_name="glVertex4fv"]
            pub fn Vertex4fv(v: *const types::GLfloat) -> ();
#[link_name="glVertex4i"]
            pub fn Vertex4i(x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> ();
#[link_name="glVertex4iv"]
            pub fn Vertex4iv(v: *const types::GLint) -> ();
#[link_name="glVertex4s"]
            pub fn Vertex4s(x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> ();
#[link_name="glVertex4sv"]
            pub fn Vertex4sv(v: *const types::GLshort) -> ();
#[link_name="glVertexPointer"]
            pub fn VertexPointer(size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glViewport"]
            pub fn Viewport(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> ();
}

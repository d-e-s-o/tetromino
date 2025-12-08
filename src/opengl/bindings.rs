
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
    
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTES: types::GLenum = 0x8B89;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_ATTRIBUTE_MAX_LENGTH: types::GLenum = 0x8B8A;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_TEXTURE: types::GLenum = 0x84E0;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORMS: types::GLenum = 0x8B86;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCKS: types::GLenum = 0x8A36;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: types::GLenum = 0x8A35;
#[allow(dead_code, non_upper_case_globals)] pub const ACTIVE_UNIFORM_MAX_LENGTH: types::GLenum = 0x8B87;
#[allow(dead_code, non_upper_case_globals)] pub const ALIASED_LINE_WIDTH_RANGE: types::GLenum = 0x846E;
#[allow(dead_code, non_upper_case_globals)] pub const ALPHA: types::GLenum = 0x1906;
#[allow(dead_code, non_upper_case_globals)] pub const ALREADY_SIGNALED: types::GLenum = 0x911A;
#[allow(dead_code, non_upper_case_globals)] pub const ALWAYS: types::GLenum = 0x0207;
#[allow(dead_code, non_upper_case_globals)] pub const AND: types::GLenum = 0x1501;
#[allow(dead_code, non_upper_case_globals)] pub const AND_INVERTED: types::GLenum = 0x1504;
#[allow(dead_code, non_upper_case_globals)] pub const AND_REVERSE: types::GLenum = 0x1502;
#[allow(dead_code, non_upper_case_globals)] pub const ANY_SAMPLES_PASSED: types::GLenum = 0x8C2F;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER: types::GLenum = 0x8892;
#[allow(dead_code, non_upper_case_globals)] pub const ARRAY_BUFFER_BINDING: types::GLenum = 0x8894;
#[allow(dead_code, non_upper_case_globals)] pub const ATTACHED_SHADERS: types::GLenum = 0x8B85;
#[allow(dead_code, non_upper_case_globals)] pub const BACK: types::GLenum = 0x0405;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_LEFT: types::GLenum = 0x0402;
#[allow(dead_code, non_upper_case_globals)] pub const BACK_RIGHT: types::GLenum = 0x0403;
#[allow(dead_code, non_upper_case_globals)] pub const BGR: types::GLenum = 0x80E0;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA: types::GLenum = 0x80E1;
#[allow(dead_code, non_upper_case_globals)] pub const BGRA_INTEGER: types::GLenum = 0x8D9B;
#[allow(dead_code, non_upper_case_globals)] pub const BGR_INTEGER: types::GLenum = 0x8D9A;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND: types::GLenum = 0x0BE2;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_COLOR: types::GLenum = 0x8005;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST: types::GLenum = 0x0BE0;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_ALPHA: types::GLenum = 0x80CA;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_DST_RGB: types::GLenum = 0x80C8;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_ALPHA: types::GLenum = 0x883D;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_EQUATION_RGB: types::GLenum = 0x8009;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC: types::GLenum = 0x0BE1;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_ALPHA: types::GLenum = 0x80CB;
#[allow(dead_code, non_upper_case_globals)] pub const BLEND_SRC_RGB: types::GLenum = 0x80C9;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE: types::GLenum = 0x1905;
#[allow(dead_code, non_upper_case_globals)] pub const BLUE_INTEGER: types::GLenum = 0x8D96;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL: types::GLenum = 0x8B56;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC2: types::GLenum = 0x8B57;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC3: types::GLenum = 0x8B58;
#[allow(dead_code, non_upper_case_globals)] pub const BOOL_VEC4: types::GLenum = 0x8B59;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS: types::GLenum = 0x88BB;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_ACCESS_FLAGS: types::GLenum = 0x911F;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAPPED: types::GLenum = 0x88BC;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_LENGTH: types::GLenum = 0x9120;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_OFFSET: types::GLenum = 0x9121;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_MAP_POINTER: types::GLenum = 0x88BD;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_SIZE: types::GLenum = 0x8764;
#[allow(dead_code, non_upper_case_globals)] pub const BUFFER_USAGE: types::GLenum = 0x8765;
#[allow(dead_code, non_upper_case_globals)] pub const BYTE: types::GLenum = 0x1400;
#[allow(dead_code, non_upper_case_globals)] pub const CCW: types::GLenum = 0x0901;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_READ_COLOR: types::GLenum = 0x891C;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_BORDER: types::GLenum = 0x812D;
#[allow(dead_code, non_upper_case_globals)] pub const CLAMP_TO_EDGE: types::GLenum = 0x812F;
#[allow(dead_code, non_upper_case_globals)] pub const CLEAR: types::GLenum = 0x1500;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE0: types::GLenum = 0x3000;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE1: types::GLenum = 0x3001;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE2: types::GLenum = 0x3002;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE3: types::GLenum = 0x3003;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE4: types::GLenum = 0x3004;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE5: types::GLenum = 0x3005;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE6: types::GLenum = 0x3006;
#[allow(dead_code, non_upper_case_globals)] pub const CLIP_DISTANCE7: types::GLenum = 0x3007;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR: types::GLenum = 0x1800;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT0: types::GLenum = 0x8CE0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT1: types::GLenum = 0x8CE1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT10: types::GLenum = 0x8CEA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT11: types::GLenum = 0x8CEB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT12: types::GLenum = 0x8CEC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT13: types::GLenum = 0x8CED;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT14: types::GLenum = 0x8CEE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT15: types::GLenum = 0x8CEF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT16: types::GLenum = 0x8CF0;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT17: types::GLenum = 0x8CF1;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT18: types::GLenum = 0x8CF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT19: types::GLenum = 0x8CF3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT2: types::GLenum = 0x8CE2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT20: types::GLenum = 0x8CF4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT21: types::GLenum = 0x8CF5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT22: types::GLenum = 0x8CF6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT23: types::GLenum = 0x8CF7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT24: types::GLenum = 0x8CF8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT25: types::GLenum = 0x8CF9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT26: types::GLenum = 0x8CFA;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT27: types::GLenum = 0x8CFB;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT28: types::GLenum = 0x8CFC;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT29: types::GLenum = 0x8CFD;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT3: types::GLenum = 0x8CE3;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT30: types::GLenum = 0x8CFE;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT31: types::GLenum = 0x8CFF;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT4: types::GLenum = 0x8CE4;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT5: types::GLenum = 0x8CE5;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT6: types::GLenum = 0x8CE6;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT7: types::GLenum = 0x8CE7;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT8: types::GLenum = 0x8CE8;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_ATTACHMENT9: types::GLenum = 0x8CE9;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_BUFFER_BIT: types::GLenum = 0x00004000;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_CLEAR_VALUE: types::GLenum = 0x0C22;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_LOGIC_OP: types::GLenum = 0x0BF2;
#[allow(dead_code, non_upper_case_globals)] pub const COLOR_WRITEMASK: types::GLenum = 0x0C23;
#[allow(dead_code, non_upper_case_globals)] pub const COMPARE_REF_TO_TEXTURE: types::GLenum = 0x884E;
#[allow(dead_code, non_upper_case_globals)] pub const COMPILE_STATUS: types::GLenum = 0x8B81;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED: types::GLenum = 0x8225;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RED_RGTC1: types::GLenum = 0x8DBB;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG: types::GLenum = 0x8226;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGB: types::GLenum = 0x84ED;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RGBA: types::GLenum = 0x84EE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_RG_RGTC2: types::GLenum = 0x8DBD;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RED_RGTC1: types::GLenum = 0x8DBC;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SIGNED_RG_RGTC2: types::GLenum = 0x8DBE;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB: types::GLenum = 0x8C48;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_SRGB_ALPHA: types::GLenum = 0x8C49;
#[allow(dead_code, non_upper_case_globals)] pub const COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A3;
#[allow(dead_code, non_upper_case_globals)] pub const CONDITION_SATISFIED: types::GLenum = 0x911C;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_ALPHA: types::GLenum = 0x8003;
#[allow(dead_code, non_upper_case_globals)] pub const CONSTANT_COLOR: types::GLenum = 0x8001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_COMPATIBILITY_PROFILE_BIT: types::GLenum = 0x00000002;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_CORE_PROFILE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAGS: types::GLenum = 0x821E;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const CONTEXT_PROFILE_MASK: types::GLenum = 0x9126;
#[allow(dead_code, non_upper_case_globals)] pub const COPY: types::GLenum = 0x1503;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_INVERTED: types::GLenum = 0x150C;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_READ_BUFFER: types::GLenum = 0x8F36;
#[allow(dead_code, non_upper_case_globals)] pub const COPY_WRITE_BUFFER: types::GLenum = 0x8F37;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE: types::GLenum = 0x0B44;
#[allow(dead_code, non_upper_case_globals)] pub const CULL_FACE_MODE: types::GLenum = 0x0B45;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_PROGRAM: types::GLenum = 0x8B8D;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_QUERY: types::GLenum = 0x8865;
#[allow(dead_code, non_upper_case_globals)] pub const CURRENT_VERTEX_ATTRIB: types::GLenum = 0x8626;
#[allow(dead_code, non_upper_case_globals)] pub const CW: types::GLenum = 0x0900;
#[allow(dead_code, non_upper_case_globals)] pub const DECR: types::GLenum = 0x1E03;
#[allow(dead_code, non_upper_case_globals)] pub const DECR_WRAP: types::GLenum = 0x8508;
#[allow(dead_code, non_upper_case_globals)] pub const DELETE_STATUS: types::GLenum = 0x8B80;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH: types::GLenum = 0x1801;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH24_STENCIL8: types::GLenum = 0x88F0;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH32F_STENCIL8: types::GLenum = 0x8CAD;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_ATTACHMENT: types::GLenum = 0x8D00;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_BUFFER_BIT: types::GLenum = 0x00000100;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLAMP: types::GLenum = 0x864F;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_CLEAR_VALUE: types::GLenum = 0x0B73;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT: types::GLenum = 0x1902;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT16: types::GLenum = 0x81A5;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT24: types::GLenum = 0x81A6;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32: types::GLenum = 0x81A7;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_COMPONENT32F: types::GLenum = 0x8CAC;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_FUNC: types::GLenum = 0x0B74;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_RANGE: types::GLenum = 0x0B70;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL: types::GLenum = 0x84F9;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_STENCIL_ATTACHMENT: types::GLenum = 0x821A;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_TEST: types::GLenum = 0x0B71;
#[allow(dead_code, non_upper_case_globals)] pub const DEPTH_WRITEMASK: types::GLenum = 0x0B72;
#[allow(dead_code, non_upper_case_globals)] pub const DITHER: types::GLenum = 0x0BD0;
#[allow(dead_code, non_upper_case_globals)] pub const DONT_CARE: types::GLenum = 0x1100;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLE: types::GLenum = 0x140A;
#[allow(dead_code, non_upper_case_globals)] pub const DOUBLEBUFFER: types::GLenum = 0x0C32;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER: types::GLenum = 0x0C01;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER0: types::GLenum = 0x8825;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER1: types::GLenum = 0x8826;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER10: types::GLenum = 0x882F;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER11: types::GLenum = 0x8830;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER12: types::GLenum = 0x8831;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER13: types::GLenum = 0x8832;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER14: types::GLenum = 0x8833;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER15: types::GLenum = 0x8834;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER2: types::GLenum = 0x8827;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER3: types::GLenum = 0x8828;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER4: types::GLenum = 0x8829;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER5: types::GLenum = 0x882A;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER6: types::GLenum = 0x882B;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER7: types::GLenum = 0x882C;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER8: types::GLenum = 0x882D;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_BUFFER9: types::GLenum = 0x882E;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER: types::GLenum = 0x8CA9;
#[allow(dead_code, non_upper_case_globals)] pub const DRAW_FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const DST_ALPHA: types::GLenum = 0x0304;
#[allow(dead_code, non_upper_case_globals)] pub const DST_COLOR: types::GLenum = 0x0306;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_COPY: types::GLenum = 0x88EA;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_DRAW: types::GLenum = 0x88E8;
#[allow(dead_code, non_upper_case_globals)] pub const DYNAMIC_READ: types::GLenum = 0x88E9;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER: types::GLenum = 0x8893;
#[allow(dead_code, non_upper_case_globals)] pub const ELEMENT_ARRAY_BUFFER_BINDING: types::GLenum = 0x8895;
#[allow(dead_code, non_upper_case_globals)] pub const EQUAL: types::GLenum = 0x0202;
#[allow(dead_code, non_upper_case_globals)] pub const EQUIV: types::GLenum = 0x1509;
#[allow(dead_code, non_upper_case_globals)] pub const EXTENSIONS: types::GLenum = 0x1F03;
#[allow(dead_code, non_upper_case_globals)] pub const FALSE: types::GLboolean = 0;
#[allow(dead_code, non_upper_case_globals)] pub const FASTEST: types::GLenum = 0x1101;
#[allow(dead_code, non_upper_case_globals)] pub const FILL: types::GLenum = 0x1B02;
#[allow(dead_code, non_upper_case_globals)] pub const FIRST_VERTEX_CONVENTION: types::GLenum = 0x8E4D;
#[allow(dead_code, non_upper_case_globals)] pub const FIXED_ONLY: types::GLenum = 0x891D;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT: types::GLenum = 0x1406;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_32_UNSIGNED_INT_24_8_REV: types::GLenum = 0x8DAD;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2: types::GLenum = 0x8B5A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x3: types::GLenum = 0x8B65;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT2x4: types::GLenum = 0x8B66;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3: types::GLenum = 0x8B5B;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x2: types::GLenum = 0x8B67;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT3x4: types::GLenum = 0x8B68;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4: types::GLenum = 0x8B5C;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x2: types::GLenum = 0x8B69;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_MAT4x3: types::GLenum = 0x8B6A;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC2: types::GLenum = 0x8B50;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC3: types::GLenum = 0x8B51;
#[allow(dead_code, non_upper_case_globals)] pub const FLOAT_VEC4: types::GLenum = 0x8B52;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER: types::GLenum = 0x8B30;
#[allow(dead_code, non_upper_case_globals)] pub const FRAGMENT_SHADER_DERIVATIVE_HINT: types::GLenum = 0x8B8B;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER: types::GLenum = 0x8D40;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: types::GLenum = 0x8215;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: types::GLenum = 0x8214;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: types::GLenum = 0x8210;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: types::GLenum = 0x8211;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: types::GLenum = 0x8216;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: types::GLenum = 0x8213;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_LAYERED: types::GLenum = 0x8DA7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: types::GLenum = 0x8CD1;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: types::GLenum = 0x8CD0;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_RED_SIZE: types::GLenum = 0x8212;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: types::GLenum = 0x8217;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: types::GLenum = 0x8CD3;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: types::GLenum = 0x8CD4;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: types::GLenum = 0x8CD2;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_BINDING: types::GLenum = 0x8CA6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_COMPLETE: types::GLenum = 0x8CD5;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_DEFAULT: types::GLenum = 0x8218;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: types::GLenum = 0x8CD6;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER: types::GLenum = 0x8CDB;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS: types::GLenum = 0x8DA8;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: types::GLenum = 0x8CD7;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: types::GLenum = 0x8D56;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_INCOMPLETE_READ_BUFFER: types::GLenum = 0x8CDC;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_SRGB: types::GLenum = 0x8DB9;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNDEFINED: types::GLenum = 0x8219;
#[allow(dead_code, non_upper_case_globals)] pub const FRAMEBUFFER_UNSUPPORTED: types::GLenum = 0x8CDD;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT: types::GLenum = 0x0404;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_AND_BACK: types::GLenum = 0x0408;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_FACE: types::GLenum = 0x0B46;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_LEFT: types::GLenum = 0x0400;
#[allow(dead_code, non_upper_case_globals)] pub const FRONT_RIGHT: types::GLenum = 0x0401;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_ADD: types::GLenum = 0x8006;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_REVERSE_SUBTRACT: types::GLenum = 0x800B;
#[allow(dead_code, non_upper_case_globals)] pub const FUNC_SUBTRACT: types::GLenum = 0x800A;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_INPUT_TYPE: types::GLenum = 0x8917;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_OUTPUT_TYPE: types::GLenum = 0x8918;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_SHADER: types::GLenum = 0x8DD9;
#[allow(dead_code, non_upper_case_globals)] pub const GEOMETRY_VERTICES_OUT: types::GLenum = 0x8916;
#[allow(dead_code, non_upper_case_globals)] pub const GEQUAL: types::GLenum = 0x0206;
#[allow(dead_code, non_upper_case_globals)] pub const GREATER: types::GLenum = 0x0204;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN: types::GLenum = 0x1904;
#[allow(dead_code, non_upper_case_globals)] pub const GREEN_INTEGER: types::GLenum = 0x8D95;
#[allow(dead_code, non_upper_case_globals)] pub const HALF_FLOAT: types::GLenum = 0x140B;
#[allow(dead_code, non_upper_case_globals)] pub const INCR: types::GLenum = 0x1E02;
#[allow(dead_code, non_upper_case_globals)] pub const INCR_WRAP: types::GLenum = 0x8507;
#[allow(dead_code, non_upper_case_globals)] pub const INFO_LOG_LENGTH: types::GLenum = 0x8B84;
#[allow(dead_code, non_upper_case_globals)] pub const INT: types::GLenum = 0x1404;
#[allow(dead_code, non_upper_case_globals)] pub const INTERLEAVED_ATTRIBS: types::GLenum = 0x8C8C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_2_10_10_10_REV: types::GLenum = 0x8D9F;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D: types::GLenum = 0x8DC9;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DCE;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D: types::GLenum = 0x8DCA;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DCF;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9109;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910C;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_2D_RECT: types::GLenum = 0x8DCD;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_3D: types::GLenum = 0x8DCB;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_BUFFER: types::GLenum = 0x8DD0;
#[allow(dead_code, non_upper_case_globals)] pub const INT_SAMPLER_CUBE: types::GLenum = 0x8DCC;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC2: types::GLenum = 0x8B53;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC3: types::GLenum = 0x8B54;
#[allow(dead_code, non_upper_case_globals)] pub const INT_VEC4: types::GLenum = 0x8B55;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_ENUM: types::GLenum = 0x0500;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_FRAMEBUFFER_OPERATION: types::GLenum = 0x0506;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_INDEX: types::GLuint = 0xFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_OPERATION: types::GLenum = 0x0502;
#[allow(dead_code, non_upper_case_globals)] pub const INVALID_VALUE: types::GLenum = 0x0501;
#[allow(dead_code, non_upper_case_globals)] pub const INVERT: types::GLenum = 0x150A;
#[allow(dead_code, non_upper_case_globals)] pub const KEEP: types::GLenum = 0x1E00;
#[allow(dead_code, non_upper_case_globals)] pub const LAST_VERTEX_CONVENTION: types::GLenum = 0x8E4E;
#[allow(dead_code, non_upper_case_globals)] pub const LEFT: types::GLenum = 0x0406;
#[allow(dead_code, non_upper_case_globals)] pub const LEQUAL: types::GLenum = 0x0203;
#[allow(dead_code, non_upper_case_globals)] pub const LESS: types::GLenum = 0x0201;
#[allow(dead_code, non_upper_case_globals)] pub const LINE: types::GLenum = 0x1B01;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR: types::GLenum = 0x2601;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_LINEAR: types::GLenum = 0x2703;
#[allow(dead_code, non_upper_case_globals)] pub const LINEAR_MIPMAP_NEAREST: types::GLenum = 0x2701;
#[allow(dead_code, non_upper_case_globals)] pub const LINES: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const LINES_ADJACENCY: types::GLenum = 0x000A;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_LOOP: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH: types::GLenum = 0x0B20;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_SMOOTH_HINT: types::GLenum = 0x0C52;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP: types::GLenum = 0x0003;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_STRIP_ADJACENCY: types::GLenum = 0x000B;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH: types::GLenum = 0x0B21;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const LINK_STATUS: types::GLenum = 0x8B82;
#[allow(dead_code, non_upper_case_globals)] pub const LOGIC_OP_MODE: types::GLenum = 0x0BF0;
#[allow(dead_code, non_upper_case_globals)] pub const LOWER_LEFT: types::GLenum = 0x8CA1;
#[allow(dead_code, non_upper_case_globals)] pub const MAJOR_VERSION: types::GLenum = 0x821B;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_FLUSH_EXPLICIT_BIT: types::GLenum = 0x0010;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_BUFFER_BIT: types::GLenum = 0x0008;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_INVALIDATE_RANGE_BIT: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_READ_BIT: types::GLenum = 0x0001;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_UNSYNCHRONIZED_BIT: types::GLenum = 0x0020;
#[allow(dead_code, non_upper_case_globals)] pub const MAP_WRITE_BIT: types::GLenum = 0x0002;
#[allow(dead_code, non_upper_case_globals)] pub const MAX: types::GLenum = 0x8008;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_3D_TEXTURE_SIZE: types::GLenum = 0x8073;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ARRAY_TEXTURE_LAYERS: types::GLenum = 0x88FF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CLIP_DISTANCES: types::GLenum = 0x0D32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_ATTACHMENTS: types::GLenum = 0x8CDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COLOR_TEXTURE_SAMPLES: types::GLenum = 0x910E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8A33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8A32;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_UNIFORM_BLOCKS: types::GLenum = 0x8A2E;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8A31;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_CUBE_MAP_TEXTURE_SIZE: types::GLenum = 0x851C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DEPTH_TEXTURE_SAMPLES: types::GLenum = 0x910F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DRAW_BUFFERS: types::GLenum = 0x8824;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_DUAL_SOURCE_DRAW_BUFFERS: types::GLenum = 0x88FC;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_INDICES: types::GLenum = 0x80E9;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_ELEMENTS_VERTICES: types::GLenum = 0x80E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_INPUT_COMPONENTS: types::GLenum = 0x9125;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_BLOCKS: types::GLenum = 0x8A2D;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_FRAGMENT_UNIFORM_COMPONENTS: types::GLenum = 0x8B49;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_INPUT_COMPONENTS: types::GLenum = 0x9123;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_COMPONENTS: types::GLenum = 0x9124;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_OUTPUT_VERTICES: types::GLenum = 0x8DE0;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8C29;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS: types::GLenum = 0x8DE1;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_BLOCKS: types::GLenum = 0x8A2C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_GEOMETRY_UNIFORM_COMPONENTS: types::GLenum = 0x8DDF;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_INTEGER_SAMPLES: types::GLenum = 0x9110;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8905;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RECTANGLE_TEXTURE_SIZE: types::GLenum = 0x84F8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_RENDERBUFFER_SIZE: types::GLenum = 0x84E8;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLES: types::GLenum = 0x8D57;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SAMPLE_MASK_WORDS: types::GLenum = 0x8E59;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_SERVER_WAIT_TIMEOUT: types::GLenum = 0x9111;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_BUFFER_SIZE: types::GLenum = 0x8C2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8872;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_LOD_BIAS: types::GLenum = 0x84FD;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TEXTURE_SIZE: types::GLenum = 0x0D33;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: types::GLenum = 0x8C8A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: types::GLenum = 0x8C8B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: types::GLenum = 0x8C80;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BLOCK_SIZE: types::GLenum = 0x8A30;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_UNIFORM_BUFFER_BINDINGS: types::GLenum = 0x8A2F;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_COMPONENTS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VARYING_FLOATS: types::GLenum = 0x8B4B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_ATTRIBS: types::GLenum = 0x8869;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_OUTPUT_COMPONENTS: types::GLenum = 0x9122;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: types::GLenum = 0x8B4C;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_BLOCKS: types::GLenum = 0x8A2B;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VERTEX_UNIFORM_COMPONENTS: types::GLenum = 0x8B4A;
#[allow(dead_code, non_upper_case_globals)] pub const MAX_VIEWPORT_DIMS: types::GLenum = 0x0D3A;
#[allow(dead_code, non_upper_case_globals)] pub const MIN: types::GLenum = 0x8007;
#[allow(dead_code, non_upper_case_globals)] pub const MINOR_VERSION: types::GLenum = 0x821C;
#[allow(dead_code, non_upper_case_globals)] pub const MIN_PROGRAM_TEXEL_OFFSET: types::GLenum = 0x8904;
#[allow(dead_code, non_upper_case_globals)] pub const MIRRORED_REPEAT: types::GLenum = 0x8370;
#[allow(dead_code, non_upper_case_globals)] pub const MULTISAMPLE: types::GLenum = 0x809D;
#[allow(dead_code, non_upper_case_globals)] pub const NAND: types::GLenum = 0x150E;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST: types::GLenum = 0x2600;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_LINEAR: types::GLenum = 0x2702;
#[allow(dead_code, non_upper_case_globals)] pub const NEAREST_MIPMAP_NEAREST: types::GLenum = 0x2700;
#[allow(dead_code, non_upper_case_globals)] pub const NEVER: types::GLenum = 0x0200;
#[allow(dead_code, non_upper_case_globals)] pub const NICEST: types::GLenum = 0x1102;
#[allow(dead_code, non_upper_case_globals)] pub const NONE: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NOOP: types::GLenum = 0x1505;
#[allow(dead_code, non_upper_case_globals)] pub const NOR: types::GLenum = 0x1508;
#[allow(dead_code, non_upper_case_globals)] pub const NOTEQUAL: types::GLenum = 0x0205;
#[allow(dead_code, non_upper_case_globals)] pub const NO_ERROR: types::GLenum = 0;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_COMPRESSED_TEXTURE_FORMATS: types::GLenum = 0x86A2;
#[allow(dead_code, non_upper_case_globals)] pub const NUM_EXTENSIONS: types::GLenum = 0x821D;
#[allow(dead_code, non_upper_case_globals)] pub const OBJECT_TYPE: types::GLenum = 0x9112;
#[allow(dead_code, non_upper_case_globals)] pub const ONE: types::GLenum = 1;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_ALPHA: types::GLenum = 0x8004;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_CONSTANT_COLOR: types::GLenum = 0x8002;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_ALPHA: types::GLenum = 0x0305;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_DST_COLOR: types::GLenum = 0x0307;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_ALPHA: types::GLenum = 0x88FB;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC1_COLOR: types::GLenum = 0x88FA;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_ALPHA: types::GLenum = 0x0303;
#[allow(dead_code, non_upper_case_globals)] pub const ONE_MINUS_SRC_COLOR: types::GLenum = 0x0301;
#[allow(dead_code, non_upper_case_globals)] pub const OR: types::GLenum = 0x1507;
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
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER: types::GLenum = 0x88EB;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_PACK_BUFFER_BINDING: types::GLenum = 0x88ED;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER: types::GLenum = 0x88EC;
#[allow(dead_code, non_upper_case_globals)] pub const PIXEL_UNPACK_BUFFER_BINDING: types::GLenum = 0x88EF;
#[allow(dead_code, non_upper_case_globals)] pub const POINT: types::GLenum = 0x1B00;
#[allow(dead_code, non_upper_case_globals)] pub const POINTS: types::GLenum = 0x0000;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_FADE_THRESHOLD_SIZE: types::GLenum = 0x8128;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE: types::GLenum = 0x0B11;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const POINT_SPRITE_COORD_ORIGIN: types::GLenum = 0x8CA0;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_MODE: types::GLenum = 0x0B40;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FACTOR: types::GLenum = 0x8038;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_FILL: types::GLenum = 0x8037;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_LINE: types::GLenum = 0x2A02;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_POINT: types::GLenum = 0x2A01;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_OFFSET_UNITS: types::GLenum = 0x2A00;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH: types::GLenum = 0x0B41;
#[allow(dead_code, non_upper_case_globals)] pub const POLYGON_SMOOTH_HINT: types::GLenum = 0x0C53;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVES_GENERATED: types::GLenum = 0x8C87;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART: types::GLenum = 0x8F9D;
#[allow(dead_code, non_upper_case_globals)] pub const PRIMITIVE_RESTART_INDEX: types::GLenum = 0x8F9E;
#[allow(dead_code, non_upper_case_globals)] pub const PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const PROVOKING_VERTEX: types::GLenum = 0x8E4F;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D: types::GLenum = 0x8063;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_1D_ARRAY: types::GLenum = 0x8C19;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D: types::GLenum = 0x8064;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_ARRAY: types::GLenum = 0x8C1B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9101;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9103;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_3D: types::GLenum = 0x8070;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_CUBE_MAP: types::GLenum = 0x851B;
#[allow(dead_code, non_upper_case_globals)] pub const PROXY_TEXTURE_RECTANGLE: types::GLenum = 0x84F7;
#[allow(dead_code, non_upper_case_globals)] pub const QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION: types::GLenum = 0x8E4C;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_NO_WAIT: types::GLenum = 0x8E16;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_BY_REGION_WAIT: types::GLenum = 0x8E15;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_COUNTER_BITS: types::GLenum = 0x8864;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_NO_WAIT: types::GLenum = 0x8E14;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT: types::GLenum = 0x8866;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_RESULT_AVAILABLE: types::GLenum = 0x8867;
#[allow(dead_code, non_upper_case_globals)] pub const QUERY_WAIT: types::GLenum = 0x8E13;
#[allow(dead_code, non_upper_case_globals)] pub const R11F_G11F_B10F: types::GLenum = 0x8C3A;
#[allow(dead_code, non_upper_case_globals)] pub const R16: types::GLenum = 0x822A;
#[allow(dead_code, non_upper_case_globals)] pub const R16F: types::GLenum = 0x822D;
#[allow(dead_code, non_upper_case_globals)] pub const R16I: types::GLenum = 0x8233;
#[allow(dead_code, non_upper_case_globals)] pub const R16UI: types::GLenum = 0x8234;
#[allow(dead_code, non_upper_case_globals)] pub const R16_SNORM: types::GLenum = 0x8F98;
#[allow(dead_code, non_upper_case_globals)] pub const R32F: types::GLenum = 0x822E;
#[allow(dead_code, non_upper_case_globals)] pub const R32I: types::GLenum = 0x8235;
#[allow(dead_code, non_upper_case_globals)] pub const R32UI: types::GLenum = 0x8236;
#[allow(dead_code, non_upper_case_globals)] pub const R3_G3_B2: types::GLenum = 0x2A10;
#[allow(dead_code, non_upper_case_globals)] pub const R8: types::GLenum = 0x8229;
#[allow(dead_code, non_upper_case_globals)] pub const R8I: types::GLenum = 0x8231;
#[allow(dead_code, non_upper_case_globals)] pub const R8UI: types::GLenum = 0x8232;
#[allow(dead_code, non_upper_case_globals)] pub const R8_SNORM: types::GLenum = 0x8F94;
#[allow(dead_code, non_upper_case_globals)] pub const RASTERIZER_DISCARD: types::GLenum = 0x8C89;
#[allow(dead_code, non_upper_case_globals)] pub const READ_BUFFER: types::GLenum = 0x0C02;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER: types::GLenum = 0x8CA8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_FRAMEBUFFER_BINDING: types::GLenum = 0x8CAA;
#[allow(dead_code, non_upper_case_globals)] pub const READ_ONLY: types::GLenum = 0x88B8;
#[allow(dead_code, non_upper_case_globals)] pub const READ_WRITE: types::GLenum = 0x88BA;
#[allow(dead_code, non_upper_case_globals)] pub const RED: types::GLenum = 0x1903;
#[allow(dead_code, non_upper_case_globals)] pub const RED_INTEGER: types::GLenum = 0x8D94;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER: types::GLenum = 0x8D41;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_ALPHA_SIZE: types::GLenum = 0x8D53;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BINDING: types::GLenum = 0x8CA7;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_BLUE_SIZE: types::GLenum = 0x8D52;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_DEPTH_SIZE: types::GLenum = 0x8D54;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_GREEN_SIZE: types::GLenum = 0x8D51;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_HEIGHT: types::GLenum = 0x8D43;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_INTERNAL_FORMAT: types::GLenum = 0x8D44;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_RED_SIZE: types::GLenum = 0x8D50;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_SAMPLES: types::GLenum = 0x8CAB;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_STENCIL_SIZE: types::GLenum = 0x8D55;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERBUFFER_WIDTH: types::GLenum = 0x8D42;
#[allow(dead_code, non_upper_case_globals)] pub const RENDERER: types::GLenum = 0x1F01;
#[allow(dead_code, non_upper_case_globals)] pub const REPEAT: types::GLenum = 0x2901;
#[allow(dead_code, non_upper_case_globals)] pub const REPLACE: types::GLenum = 0x1E01;
#[allow(dead_code, non_upper_case_globals)] pub const RG: types::GLenum = 0x8227;
#[allow(dead_code, non_upper_case_globals)] pub const RG16: types::GLenum = 0x822C;
#[allow(dead_code, non_upper_case_globals)] pub const RG16F: types::GLenum = 0x822F;
#[allow(dead_code, non_upper_case_globals)] pub const RG16I: types::GLenum = 0x8239;
#[allow(dead_code, non_upper_case_globals)] pub const RG16UI: types::GLenum = 0x823A;
#[allow(dead_code, non_upper_case_globals)] pub const RG16_SNORM: types::GLenum = 0x8F99;
#[allow(dead_code, non_upper_case_globals)] pub const RG32F: types::GLenum = 0x8230;
#[allow(dead_code, non_upper_case_globals)] pub const RG32I: types::GLenum = 0x823B;
#[allow(dead_code, non_upper_case_globals)] pub const RG32UI: types::GLenum = 0x823C;
#[allow(dead_code, non_upper_case_globals)] pub const RG8: types::GLenum = 0x822B;
#[allow(dead_code, non_upper_case_globals)] pub const RG8I: types::GLenum = 0x8237;
#[allow(dead_code, non_upper_case_globals)] pub const RG8UI: types::GLenum = 0x8238;
#[allow(dead_code, non_upper_case_globals)] pub const RG8_SNORM: types::GLenum = 0x8F95;
#[allow(dead_code, non_upper_case_globals)] pub const RGB: types::GLenum = 0x1907;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10: types::GLenum = 0x8052;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2: types::GLenum = 0x8059;
#[allow(dead_code, non_upper_case_globals)] pub const RGB10_A2UI: types::GLenum = 0x906F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB12: types::GLenum = 0x8053;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16: types::GLenum = 0x8054;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16F: types::GLenum = 0x881B;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16I: types::GLenum = 0x8D89;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16UI: types::GLenum = 0x8D77;
#[allow(dead_code, non_upper_case_globals)] pub const RGB16_SNORM: types::GLenum = 0x8F9A;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32F: types::GLenum = 0x8815;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32I: types::GLenum = 0x8D83;
#[allow(dead_code, non_upper_case_globals)] pub const RGB32UI: types::GLenum = 0x8D71;
#[allow(dead_code, non_upper_case_globals)] pub const RGB4: types::GLenum = 0x804F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5: types::GLenum = 0x8050;
#[allow(dead_code, non_upper_case_globals)] pub const RGB5_A1: types::GLenum = 0x8057;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8: types::GLenum = 0x8051;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8I: types::GLenum = 0x8D8F;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8UI: types::GLenum = 0x8D7D;
#[allow(dead_code, non_upper_case_globals)] pub const RGB8_SNORM: types::GLenum = 0x8F96;
#[allow(dead_code, non_upper_case_globals)] pub const RGB9_E5: types::GLenum = 0x8C3D;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA: types::GLenum = 0x1908;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA12: types::GLenum = 0x805A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16: types::GLenum = 0x805B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16F: types::GLenum = 0x881A;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16I: types::GLenum = 0x8D88;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16UI: types::GLenum = 0x8D76;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA16_SNORM: types::GLenum = 0x8F9B;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA2: types::GLenum = 0x8055;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32F: types::GLenum = 0x8814;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32I: types::GLenum = 0x8D82;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA32UI: types::GLenum = 0x8D70;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA4: types::GLenum = 0x8056;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8: types::GLenum = 0x8058;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8I: types::GLenum = 0x8D8E;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8UI: types::GLenum = 0x8D7C;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA8_SNORM: types::GLenum = 0x8F97;
#[allow(dead_code, non_upper_case_globals)] pub const RGBA_INTEGER: types::GLenum = 0x8D99;
#[allow(dead_code, non_upper_case_globals)] pub const RGB_INTEGER: types::GLenum = 0x8D98;
#[allow(dead_code, non_upper_case_globals)] pub const RG_INTEGER: types::GLenum = 0x8228;
#[allow(dead_code, non_upper_case_globals)] pub const RIGHT: types::GLenum = 0x0407;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D: types::GLenum = 0x8B5D;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY: types::GLenum = 0x8DC0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_ARRAY_SHADOW: types::GLenum = 0x8DC3;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_1D_SHADOW: types::GLenum = 0x8B61;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D: types::GLenum = 0x8B5E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY: types::GLenum = 0x8DC1;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_ARRAY_SHADOW: types::GLenum = 0x8DC4;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x9108;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910B;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT: types::GLenum = 0x8B63;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_RECT_SHADOW: types::GLenum = 0x8B64;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_2D_SHADOW: types::GLenum = 0x8B62;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_3D: types::GLenum = 0x8B5F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BINDING: types::GLenum = 0x8919;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_BUFFER: types::GLenum = 0x8DC2;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE: types::GLenum = 0x8B60;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLER_CUBE_SHADOW: types::GLenum = 0x8DC5;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES: types::GLenum = 0x80A9;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLES_PASSED: types::GLenum = 0x8914;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_COVERAGE: types::GLenum = 0x809E;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_ALPHA_TO_ONE: types::GLenum = 0x809F;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_BUFFERS: types::GLenum = 0x80A8;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE: types::GLenum = 0x80A0;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_INVERT: types::GLenum = 0x80AB;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_COVERAGE_VALUE: types::GLenum = 0x80AA;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK: types::GLenum = 0x8E51;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_MASK_VALUE: types::GLenum = 0x8E52;
#[allow(dead_code, non_upper_case_globals)] pub const SAMPLE_POSITION: types::GLenum = 0x8E50;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_BOX: types::GLenum = 0x0C10;
#[allow(dead_code, non_upper_case_globals)] pub const SCISSOR_TEST: types::GLenum = 0x0C11;
#[allow(dead_code, non_upper_case_globals)] pub const SEPARATE_ATTRIBS: types::GLenum = 0x8C8D;
#[allow(dead_code, non_upper_case_globals)] pub const SET: types::GLenum = 0x150F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_SOURCE_LENGTH: types::GLenum = 0x8B88;
#[allow(dead_code, non_upper_case_globals)] pub const SHADER_TYPE: types::GLenum = 0x8B4F;
#[allow(dead_code, non_upper_case_globals)] pub const SHADING_LANGUAGE_VERSION: types::GLenum = 0x8B8C;
#[allow(dead_code, non_upper_case_globals)] pub const SHORT: types::GLenum = 0x1402;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNALED: types::GLenum = 0x9119;
#[allow(dead_code, non_upper_case_globals)] pub const SIGNED_NORMALIZED: types::GLenum = 0x8F9C;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_GRANULARITY: types::GLenum = 0x0B23;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_LINE_WIDTH_RANGE: types::GLenum = 0x0B22;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_GRANULARITY: types::GLenum = 0x0B13;
#[allow(dead_code, non_upper_case_globals)] pub const SMOOTH_POINT_SIZE_RANGE: types::GLenum = 0x0B12;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_ALPHA: types::GLenum = 0x8589;
#[allow(dead_code, non_upper_case_globals)] pub const SRC1_COLOR: types::GLenum = 0x88F9;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA: types::GLenum = 0x0302;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_ALPHA_SATURATE: types::GLenum = 0x0308;
#[allow(dead_code, non_upper_case_globals)] pub const SRC_COLOR: types::GLenum = 0x0300;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB: types::GLenum = 0x8C40;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8: types::GLenum = 0x8C41;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB8_ALPHA8: types::GLenum = 0x8C43;
#[allow(dead_code, non_upper_case_globals)] pub const SRGB_ALPHA: types::GLenum = 0x8C42;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_COPY: types::GLenum = 0x88E6;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_DRAW: types::GLenum = 0x88E4;
#[allow(dead_code, non_upper_case_globals)] pub const STATIC_READ: types::GLenum = 0x88E5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL: types::GLenum = 0x1802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_ATTACHMENT: types::GLenum = 0x8D20;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FAIL: types::GLenum = 0x8801;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_FUNC: types::GLenum = 0x8800;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_FAIL: types::GLenum = 0x8802;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_PASS_DEPTH_PASS: types::GLenum = 0x8803;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_REF: types::GLenum = 0x8CA3;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_VALUE_MASK: types::GLenum = 0x8CA4;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BACK_WRITEMASK: types::GLenum = 0x8CA5;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_BUFFER_BIT: types::GLenum = 0x00000400;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_CLEAR_VALUE: types::GLenum = 0x0B91;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FAIL: types::GLenum = 0x0B94;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_FUNC: types::GLenum = 0x0B92;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX: types::GLenum = 0x1901;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX1: types::GLenum = 0x8D46;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX16: types::GLenum = 0x8D49;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX4: types::GLenum = 0x8D47;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_INDEX8: types::GLenum = 0x8D48;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_FAIL: types::GLenum = 0x0B95;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_PASS_DEPTH_PASS: types::GLenum = 0x0B96;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_REF: types::GLenum = 0x0B97;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_TEST: types::GLenum = 0x0B90;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_VALUE_MASK: types::GLenum = 0x0B93;
#[allow(dead_code, non_upper_case_globals)] pub const STENCIL_WRITEMASK: types::GLenum = 0x0B98;
#[allow(dead_code, non_upper_case_globals)] pub const STEREO: types::GLenum = 0x0C33;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_COPY: types::GLenum = 0x88E2;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_DRAW: types::GLenum = 0x88E0;
#[allow(dead_code, non_upper_case_globals)] pub const STREAM_READ: types::GLenum = 0x88E1;
#[allow(dead_code, non_upper_case_globals)] pub const SUBPIXEL_BITS: types::GLenum = 0x0D50;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_CONDITION: types::GLenum = 0x9113;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FENCE: types::GLenum = 0x9116;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLAGS: types::GLenum = 0x9115;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_FLUSH_COMMANDS_BIT: types::GLenum = 0x00000001;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_GPU_COMMANDS_COMPLETE: types::GLenum = 0x9117;
#[allow(dead_code, non_upper_case_globals)] pub const SYNC_STATUS: types::GLenum = 0x9114;
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
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_1D_ARRAY: types::GLenum = 0x8C18;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D: types::GLenum = 0x0DE1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_ARRAY: types::GLenum = 0x8C1A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE: types::GLenum = 0x9100;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9102;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_3D: types::GLenum = 0x806F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_SIZE: types::GLenum = 0x805F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_ALPHA_TYPE: types::GLenum = 0x8C13;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BASE_LEVEL: types::GLenum = 0x813C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D: types::GLenum = 0x8068;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_1D_ARRAY: types::GLenum = 0x8C1C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D: types::GLenum = 0x8069;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_ARRAY: types::GLenum = 0x8C1D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE: types::GLenum = 0x9104;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x9105;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_3D: types::GLenum = 0x806A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_BUFFER: types::GLenum = 0x8C2C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_CUBE_MAP: types::GLenum = 0x8514;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BINDING_RECTANGLE: types::GLenum = 0x84F6;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_SIZE: types::GLenum = 0x805E;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BLUE_TYPE: types::GLenum = 0x8C12;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BORDER_COLOR: types::GLenum = 0x1004;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER: types::GLenum = 0x8C2A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_BUFFER_DATA_STORE_BINDING: types::GLenum = 0x8C2D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_FUNC: types::GLenum = 0x884D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPARE_MODE: types::GLenum = 0x884C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED: types::GLenum = 0x86A1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSED_IMAGE_SIZE: types::GLenum = 0x86A0;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_COMPRESSION_HINT: types::GLenum = 0x84EF;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP: types::GLenum = 0x8513;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_X: types::GLenum = 0x8516;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: types::GLenum = 0x8518;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: types::GLenum = 0x851A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_X: types::GLenum = 0x8515;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Y: types::GLenum = 0x8517;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_POSITIVE_Z: types::GLenum = 0x8519;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_CUBE_MAP_SEAMLESS: types::GLenum = 0x884F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH: types::GLenum = 0x8071;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_SIZE: types::GLenum = 0x884A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_DEPTH_TYPE: types::GLenum = 0x8C16;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_FIXED_SAMPLE_LOCATIONS: types::GLenum = 0x9107;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_SIZE: types::GLenum = 0x805D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_GREEN_TYPE: types::GLenum = 0x8C11;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_HEIGHT: types::GLenum = 0x1001;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_INTERNAL_FORMAT: types::GLenum = 0x1003;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_LOD_BIAS: types::GLenum = 0x8501;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAG_FILTER: types::GLenum = 0x2800;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LEVEL: types::GLenum = 0x813D;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MAX_LOD: types::GLenum = 0x813B;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_FILTER: types::GLenum = 0x2801;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_MIN_LOD: types::GLenum = 0x813A;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RECTANGLE: types::GLenum = 0x84F5;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_SIZE: types::GLenum = 0x805C;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_RED_TYPE: types::GLenum = 0x8C10;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SAMPLES: types::GLenum = 0x9106;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SHARED_SIZE: types::GLenum = 0x8C3F;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_STENCIL_SIZE: types::GLenum = 0x88F1;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_A: types::GLenum = 0x8E45;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_B: types::GLenum = 0x8E44;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_G: types::GLenum = 0x8E43;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_R: types::GLenum = 0x8E42;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_SWIZZLE_RGBA: types::GLenum = 0x8E46;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WIDTH: types::GLenum = 0x1000;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_R: types::GLenum = 0x8072;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_S: types::GLenum = 0x2802;
#[allow(dead_code, non_upper_case_globals)] pub const TEXTURE_WRAP_T: types::GLenum = 0x2803;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_EXPIRED: types::GLenum = 0x911B;
#[allow(dead_code, non_upper_case_globals)] pub const TIMEOUT_IGNORED: types::GLuint64 = 0xFFFFFFFFFFFFFFFF;
#[allow(dead_code, non_upper_case_globals)] pub const TIMESTAMP: types::GLenum = 0x8E28;
#[allow(dead_code, non_upper_case_globals)] pub const TIME_ELAPSED: types::GLenum = 0x88BF;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER: types::GLenum = 0x8C8E;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_BINDING: types::GLenum = 0x8C8F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_MODE: types::GLenum = 0x8C7F;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_SIZE: types::GLenum = 0x8C85;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_BUFFER_START: types::GLenum = 0x8C84;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: types::GLenum = 0x8C88;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYINGS: types::GLenum = 0x8C83;
#[allow(dead_code, non_upper_case_globals)] pub const TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: types::GLenum = 0x8C76;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES: types::GLenum = 0x0004;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLES_ADJACENCY: types::GLenum = 0x000C;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_FAN: types::GLenum = 0x0006;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP: types::GLenum = 0x0005;
#[allow(dead_code, non_upper_case_globals)] pub const TRIANGLE_STRIP_ADJACENCY: types::GLenum = 0x000D;
#[allow(dead_code, non_upper_case_globals)] pub const TRUE: types::GLboolean = 1;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_ARRAY_STRIDE: types::GLenum = 0x8A3C;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORMS: types::GLenum = 0x8A42;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: types::GLenum = 0x8A43;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_BINDING: types::GLenum = 0x8A3F;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_DATA_SIZE: types::GLenum = 0x8A40;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_INDEX: types::GLenum = 0x8A3A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_NAME_LENGTH: types::GLenum = 0x8A41;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: types::GLenum = 0x8A46;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER: types::GLenum = 0x8A45;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: types::GLenum = 0x8A44;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER: types::GLenum = 0x8A11;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_BINDING: types::GLenum = 0x8A28;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_OFFSET_ALIGNMENT: types::GLenum = 0x8A34;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_SIZE: types::GLenum = 0x8A2A;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_BUFFER_START: types::GLenum = 0x8A29;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_IS_ROW_MAJOR: types::GLenum = 0x8A3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_MATRIX_STRIDE: types::GLenum = 0x8A3D;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_NAME_LENGTH: types::GLenum = 0x8A39;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_OFFSET: types::GLenum = 0x8A3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_SIZE: types::GLenum = 0x8A38;
#[allow(dead_code, non_upper_case_globals)] pub const UNIFORM_TYPE: types::GLenum = 0x8A37;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ALIGNMENT: types::GLenum = 0x0CF5;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_IMAGE_HEIGHT: types::GLenum = 0x806E;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_LSB_FIRST: types::GLenum = 0x0CF1;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_ROW_LENGTH: types::GLenum = 0x0CF2;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_IMAGES: types::GLenum = 0x806D;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_PIXELS: types::GLenum = 0x0CF4;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SKIP_ROWS: types::GLenum = 0x0CF3;
#[allow(dead_code, non_upper_case_globals)] pub const UNPACK_SWAP_BYTES: types::GLenum = 0x0CF0;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNALED: types::GLenum = 0x9118;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE: types::GLenum = 0x1401;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_2_3_3_REV: types::GLenum = 0x8362;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_BYTE_3_3_2: types::GLenum = 0x8032;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT: types::GLenum = 0x1405;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10F_11F_11F_REV: types::GLenum = 0x8C3B;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_10_10_10_2: types::GLenum = 0x8036;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_24_8: types::GLenum = 0x84FA;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_2_10_10_10_REV: types::GLenum = 0x8368;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_5_9_9_9_REV: types::GLenum = 0x8C3E;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8: types::GLenum = 0x8035;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_8_8_8_8_REV: types::GLenum = 0x8367;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D: types::GLenum = 0x8DD1;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_1D_ARRAY: types::GLenum = 0x8DD6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D: types::GLenum = 0x8DD2;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_ARRAY: types::GLenum = 0x8DD7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE: types::GLenum = 0x910A;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY: types::GLenum = 0x910D;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_2D_RECT: types::GLenum = 0x8DD5;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_3D: types::GLenum = 0x8DD3;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_BUFFER: types::GLenum = 0x8DD8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_SAMPLER_CUBE: types::GLenum = 0x8DD4;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC2: types::GLenum = 0x8DC6;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC3: types::GLenum = 0x8DC7;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_INT_VEC4: types::GLenum = 0x8DC8;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_NORMALIZED: types::GLenum = 0x8C17;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT: types::GLenum = 0x1403;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_1_5_5_5_REV: types::GLenum = 0x8366;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4: types::GLenum = 0x8033;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_4_4_4_4_REV: types::GLenum = 0x8365;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_5_5_1: types::GLenum = 0x8034;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5: types::GLenum = 0x8363;
#[allow(dead_code, non_upper_case_globals)] pub const UNSIGNED_SHORT_5_6_5_REV: types::GLenum = 0x8364;
#[allow(dead_code, non_upper_case_globals)] pub const UPPER_LEFT: types::GLenum = 0x8CA2;
#[allow(dead_code, non_upper_case_globals)] pub const VALIDATE_STATUS: types::GLenum = 0x8B83;
#[allow(dead_code, non_upper_case_globals)] pub const VENDOR: types::GLenum = 0x1F00;
#[allow(dead_code, non_upper_case_globals)] pub const VERSION: types::GLenum = 0x1F02;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ARRAY_BINDING: types::GLenum = 0x85B5;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: types::GLenum = 0x889F;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_DIVISOR: types::GLenum = 0x88FE;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_ENABLED: types::GLenum = 0x8622;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_INTEGER: types::GLenum = 0x88FD;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: types::GLenum = 0x886A;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_POINTER: types::GLenum = 0x8645;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_SIZE: types::GLenum = 0x8623;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_STRIDE: types::GLenum = 0x8624;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_ATTRIB_ARRAY_TYPE: types::GLenum = 0x8625;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_PROGRAM_POINT_SIZE: types::GLenum = 0x8642;
#[allow(dead_code, non_upper_case_globals)] pub const VERTEX_SHADER: types::GLenum = 0x8B31;
#[allow(dead_code, non_upper_case_globals)] pub const VIEWPORT: types::GLenum = 0x0BA2;
#[allow(dead_code, non_upper_case_globals)] pub const WAIT_FAILED: types::GLenum = 0x911D;
#[allow(dead_code, non_upper_case_globals)] pub const WRITE_ONLY: types::GLenum = 0x88B9;
#[allow(dead_code, non_upper_case_globals)] pub const XOR: types::GLenum = 0x1506;
#[allow(dead_code, non_upper_case_globals)] pub const ZERO: types::GLenum = 0;

        #[allow(non_snake_case, unused_variables, dead_code)]
        extern "system" {
#[link_name="glActiveTexture"]
            pub fn ActiveTexture(texture: types::GLenum) -> ();
#[link_name="glAttachShader"]
            pub fn AttachShader(program: types::GLuint, shader: types::GLuint) -> ();
#[link_name="glBeginConditionalRender"]
            pub fn BeginConditionalRender(id: types::GLuint, mode: types::GLenum) -> ();
#[link_name="glBeginQuery"]
            pub fn BeginQuery(target: types::GLenum, id: types::GLuint) -> ();
#[link_name="glBeginTransformFeedback"]
            pub fn BeginTransformFeedback(primitiveMode: types::GLenum) -> ();
#[link_name="glBindAttribLocation"]
            pub fn BindAttribLocation(program: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> ();
#[link_name="glBindBuffer"]
            pub fn BindBuffer(target: types::GLenum, buffer: types::GLuint) -> ();
#[link_name="glBindBufferBase"]
            pub fn BindBufferBase(target: types::GLenum, index: types::GLuint, buffer: types::GLuint) -> ();
#[link_name="glBindBufferRange"]
            pub fn BindBufferRange(target: types::GLenum, index: types::GLuint, buffer: types::GLuint, offset: types::GLintptr, size: types::GLsizeiptr) -> ();
#[link_name="glBindFragDataLocation"]
            pub fn BindFragDataLocation(program: types::GLuint, color: types::GLuint, name: *const types::GLchar) -> ();
#[link_name="glBindFragDataLocationIndexed"]
            pub fn BindFragDataLocationIndexed(program: types::GLuint, colorNumber: types::GLuint, index: types::GLuint, name: *const types::GLchar) -> ();
#[link_name="glBindFramebuffer"]
            pub fn BindFramebuffer(target: types::GLenum, framebuffer: types::GLuint) -> ();
#[link_name="glBindRenderbuffer"]
            pub fn BindRenderbuffer(target: types::GLenum, renderbuffer: types::GLuint) -> ();
#[link_name="glBindSampler"]
            pub fn BindSampler(unit: types::GLuint, sampler: types::GLuint) -> ();
#[link_name="glBindTexture"]
            pub fn BindTexture(target: types::GLenum, texture: types::GLuint) -> ();
#[link_name="glBindVertexArray"]
            pub fn BindVertexArray(array: types::GLuint) -> ();
#[link_name="glBlendColor"]
            pub fn BlendColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> ();
#[link_name="glBlendEquation"]
            pub fn BlendEquation(mode: types::GLenum) -> ();
#[link_name="glBlendEquationSeparate"]
            pub fn BlendEquationSeparate(modeRGB: types::GLenum, modeAlpha: types::GLenum) -> ();
#[link_name="glBlendFunc"]
            pub fn BlendFunc(sfactor: types::GLenum, dfactor: types::GLenum) -> ();
#[link_name="glBlendFuncSeparate"]
            pub fn BlendFuncSeparate(sfactorRGB: types::GLenum, dfactorRGB: types::GLenum, sfactorAlpha: types::GLenum, dfactorAlpha: types::GLenum) -> ();
#[link_name="glBlitFramebuffer"]
            pub fn BlitFramebuffer(srcX0: types::GLint, srcY0: types::GLint, srcX1: types::GLint, srcY1: types::GLint, dstX0: types::GLint, dstY0: types::GLint, dstX1: types::GLint, dstY1: types::GLint, mask: types::GLbitfield, filter: types::GLenum) -> ();
#[link_name="glBufferData"]
            pub fn BufferData(target: types::GLenum, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void, usage: types::GLenum) -> ();
#[link_name="glBufferSubData"]
            pub fn BufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *const __gl_imports::raw::c_void) -> ();
#[link_name="glCheckFramebufferStatus"]
            pub fn CheckFramebufferStatus(target: types::GLenum) -> types::GLenum;
#[link_name="glClampColor"]
            pub fn ClampColor(target: types::GLenum, clamp: types::GLenum) -> ();
#[link_name="glClear"]
            pub fn Clear(mask: types::GLbitfield) -> ();
#[link_name="glClearBufferfi"]
            pub fn ClearBufferfi(buffer: types::GLenum, drawbuffer: types::GLint, depth: types::GLfloat, stencil: types::GLint) -> ();
#[link_name="glClearBufferfv"]
            pub fn ClearBufferfv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLfloat) -> ();
#[link_name="glClearBufferiv"]
            pub fn ClearBufferiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLint) -> ();
#[link_name="glClearBufferuiv"]
            pub fn ClearBufferuiv(buffer: types::GLenum, drawbuffer: types::GLint, value: *const types::GLuint) -> ();
#[link_name="glClearColor"]
            pub fn ClearColor(red: types::GLfloat, green: types::GLfloat, blue: types::GLfloat, alpha: types::GLfloat) -> ();
#[link_name="glClearDepth"]
            pub fn ClearDepth(depth: types::GLdouble) -> ();
#[link_name="glClearStencil"]
            pub fn ClearStencil(s: types::GLint) -> ();
#[link_name="glClientWaitSync"]
            pub fn ClientWaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> types::GLenum;
#[link_name="glColorMask"]
            pub fn ColorMask(red: types::GLboolean, green: types::GLboolean, blue: types::GLboolean, alpha: types::GLboolean) -> ();
#[link_name="glColorMaski"]
            pub fn ColorMaski(index: types::GLuint, r: types::GLboolean, g: types::GLboolean, b: types::GLboolean, a: types::GLboolean) -> ();
#[link_name="glColorP3ui"]
            pub fn ColorP3ui(type_: types::GLenum, color: types::GLuint) -> ();
#[link_name="glColorP3uiv"]
            pub fn ColorP3uiv(type_: types::GLenum, color: *const types::GLuint) -> ();
#[link_name="glColorP4ui"]
            pub fn ColorP4ui(type_: types::GLenum, color: types::GLuint) -> ();
#[link_name="glColorP4uiv"]
            pub fn ColorP4uiv(type_: types::GLenum, color: *const types::GLuint) -> ();
#[link_name="glCompileShader"]
            pub fn CompileShader(shader: types::GLuint) -> ();
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
#[link_name="glCopyBufferSubData"]
            pub fn CopyBufferSubData(readTarget: types::GLenum, writeTarget: types::GLenum, readOffset: types::GLintptr, writeOffset: types::GLintptr, size: types::GLsizeiptr) -> ();
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
#[link_name="glCreateProgram"]
            pub fn CreateProgram() -> types::GLuint;
#[link_name="glCreateShader"]
            pub fn CreateShader(type_: types::GLenum) -> types::GLuint;
#[link_name="glCullFace"]
            pub fn CullFace(mode: types::GLenum) -> ();
#[link_name="glDeleteBuffers"]
            pub fn DeleteBuffers(n: types::GLsizei, buffers: *const types::GLuint) -> ();
#[link_name="glDeleteFramebuffers"]
            pub fn DeleteFramebuffers(n: types::GLsizei, framebuffers: *const types::GLuint) -> ();
#[link_name="glDeleteProgram"]
            pub fn DeleteProgram(program: types::GLuint) -> ();
#[link_name="glDeleteQueries"]
            pub fn DeleteQueries(n: types::GLsizei, ids: *const types::GLuint) -> ();
#[link_name="glDeleteRenderbuffers"]
            pub fn DeleteRenderbuffers(n: types::GLsizei, renderbuffers: *const types::GLuint) -> ();
#[link_name="glDeleteSamplers"]
            pub fn DeleteSamplers(count: types::GLsizei, samplers: *const types::GLuint) -> ();
#[link_name="glDeleteShader"]
            pub fn DeleteShader(shader: types::GLuint) -> ();
#[link_name="glDeleteSync"]
            pub fn DeleteSync(sync: types::GLsync) -> ();
#[link_name="glDeleteTextures"]
            pub fn DeleteTextures(n: types::GLsizei, textures: *const types::GLuint) -> ();
#[link_name="glDeleteVertexArrays"]
            pub fn DeleteVertexArrays(n: types::GLsizei, arrays: *const types::GLuint) -> ();
#[link_name="glDepthFunc"]
            pub fn DepthFunc(func: types::GLenum) -> ();
#[link_name="glDepthMask"]
            pub fn DepthMask(flag: types::GLboolean) -> ();
#[link_name="glDepthRange"]
            pub fn DepthRange(n: types::GLdouble, f: types::GLdouble) -> ();
#[link_name="glDetachShader"]
            pub fn DetachShader(program: types::GLuint, shader: types::GLuint) -> ();
#[link_name="glDisable"]
            pub fn Disable(cap: types::GLenum) -> ();
#[link_name="glDisableVertexAttribArray"]
            pub fn DisableVertexAttribArray(index: types::GLuint) -> ();
#[link_name="glDisablei"]
            pub fn Disablei(target: types::GLenum, index: types::GLuint) -> ();
#[link_name="glDrawArrays"]
            pub fn DrawArrays(mode: types::GLenum, first: types::GLint, count: types::GLsizei) -> ();
#[link_name="glDrawArraysInstanced"]
            pub fn DrawArraysInstanced(mode: types::GLenum, first: types::GLint, count: types::GLsizei, instancecount: types::GLsizei) -> ();
#[link_name="glDrawBuffer"]
            pub fn DrawBuffer(buf: types::GLenum) -> ();
#[link_name="glDrawBuffers"]
            pub fn DrawBuffers(n: types::GLsizei, bufs: *const types::GLenum) -> ();
#[link_name="glDrawElements"]
            pub fn DrawElements(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> ();
#[link_name="glDrawElementsBaseVertex"]
            pub fn DrawElementsBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> ();
#[link_name="glDrawElementsInstanced"]
            pub fn DrawElementsInstanced(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei) -> ();
#[link_name="glDrawElementsInstancedBaseVertex"]
            pub fn DrawElementsInstancedBaseVertex(mode: types::GLenum, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, instancecount: types::GLsizei, basevertex: types::GLint) -> ();
#[link_name="glDrawRangeElements"]
            pub fn DrawRangeElements(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void) -> ();
#[link_name="glDrawRangeElementsBaseVertex"]
            pub fn DrawRangeElementsBaseVertex(mode: types::GLenum, start: types::GLuint, end: types::GLuint, count: types::GLsizei, type_: types::GLenum, indices: *const __gl_imports::raw::c_void, basevertex: types::GLint) -> ();
#[link_name="glEnable"]
            pub fn Enable(cap: types::GLenum) -> ();
#[link_name="glEnableVertexAttribArray"]
            pub fn EnableVertexAttribArray(index: types::GLuint) -> ();
#[link_name="glEnablei"]
            pub fn Enablei(target: types::GLenum, index: types::GLuint) -> ();
#[link_name="glEndConditionalRender"]
            pub fn EndConditionalRender() -> ();
#[link_name="glEndQuery"]
            pub fn EndQuery(target: types::GLenum) -> ();
#[link_name="glEndTransformFeedback"]
            pub fn EndTransformFeedback() -> ();
#[link_name="glFenceSync"]
            pub fn FenceSync(condition: types::GLenum, flags: types::GLbitfield) -> types::GLsync;
#[link_name="glFinish"]
            pub fn Finish() -> ();
#[link_name="glFlush"]
            pub fn Flush() -> ();
#[link_name="glFlushMappedBufferRange"]
            pub fn FlushMappedBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr) -> ();
#[link_name="glFramebufferRenderbuffer"]
            pub fn FramebufferRenderbuffer(target: types::GLenum, attachment: types::GLenum, renderbuffertarget: types::GLenum, renderbuffer: types::GLuint) -> ();
#[link_name="glFramebufferTexture"]
            pub fn FramebufferTexture(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint) -> ();
#[link_name="glFramebufferTexture1D"]
            pub fn FramebufferTexture1D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> ();
#[link_name="glFramebufferTexture2D"]
            pub fn FramebufferTexture2D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint) -> ();
#[link_name="glFramebufferTexture3D"]
            pub fn FramebufferTexture3D(target: types::GLenum, attachment: types::GLenum, textarget: types::GLenum, texture: types::GLuint, level: types::GLint, zoffset: types::GLint) -> ();
#[link_name="glFramebufferTextureLayer"]
            pub fn FramebufferTextureLayer(target: types::GLenum, attachment: types::GLenum, texture: types::GLuint, level: types::GLint, layer: types::GLint) -> ();
#[link_name="glFrontFace"]
            pub fn FrontFace(mode: types::GLenum) -> ();
#[link_name="glGenBuffers"]
            pub fn GenBuffers(n: types::GLsizei, buffers: *mut types::GLuint) -> ();
#[link_name="glGenFramebuffers"]
            pub fn GenFramebuffers(n: types::GLsizei, framebuffers: *mut types::GLuint) -> ();
#[link_name="glGenQueries"]
            pub fn GenQueries(n: types::GLsizei, ids: *mut types::GLuint) -> ();
#[link_name="glGenRenderbuffers"]
            pub fn GenRenderbuffers(n: types::GLsizei, renderbuffers: *mut types::GLuint) -> ();
#[link_name="glGenSamplers"]
            pub fn GenSamplers(count: types::GLsizei, samplers: *mut types::GLuint) -> ();
#[link_name="glGenTextures"]
            pub fn GenTextures(n: types::GLsizei, textures: *mut types::GLuint) -> ();
#[link_name="glGenVertexArrays"]
            pub fn GenVertexArrays(n: types::GLsizei, arrays: *mut types::GLuint) -> ();
#[link_name="glGenerateMipmap"]
            pub fn GenerateMipmap(target: types::GLenum) -> ();
#[link_name="glGetActiveAttrib"]
            pub fn GetActiveAttrib(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> ();
#[link_name="glGetActiveUniform"]
            pub fn GetActiveUniform(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLint, type_: *mut types::GLenum, name: *mut types::GLchar) -> ();
#[link_name="glGetActiveUniformBlockName"]
            pub fn GetActiveUniformBlockName(program: types::GLuint, uniformBlockIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformBlockName: *mut types::GLchar) -> ();
#[link_name="glGetActiveUniformBlockiv"]
            pub fn GetActiveUniformBlockiv(program: types::GLuint, uniformBlockIndex: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetActiveUniformName"]
            pub fn GetActiveUniformName(program: types::GLuint, uniformIndex: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, uniformName: *mut types::GLchar) -> ();
#[link_name="glGetActiveUniformsiv"]
            pub fn GetActiveUniformsiv(program: types::GLuint, uniformCount: types::GLsizei, uniformIndices: *const types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetAttachedShaders"]
            pub fn GetAttachedShaders(program: types::GLuint, maxCount: types::GLsizei, count: *mut types::GLsizei, shaders: *mut types::GLuint) -> ();
#[link_name="glGetAttribLocation"]
            pub fn GetAttribLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint;
#[link_name="glGetBooleani_v"]
            pub fn GetBooleani_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLboolean) -> ();
#[link_name="glGetBooleanv"]
            pub fn GetBooleanv(pname: types::GLenum, data: *mut types::GLboolean) -> ();
#[link_name="glGetBufferParameteri64v"]
            pub fn GetBufferParameteri64v(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint64) -> ();
#[link_name="glGetBufferParameteriv"]
            pub fn GetBufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetBufferPointerv"]
            pub fn GetBufferPointerv(target: types::GLenum, pname: types::GLenum, params: *const *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetBufferSubData"]
            pub fn GetBufferSubData(target: types::GLenum, offset: types::GLintptr, size: types::GLsizeiptr, data: *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetCompressedTexImage"]
            pub fn GetCompressedTexImage(target: types::GLenum, level: types::GLint, img: *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetDoublev"]
            pub fn GetDoublev(pname: types::GLenum, data: *mut types::GLdouble) -> ();
#[link_name="glGetError"]
            pub fn GetError() -> types::GLenum;
#[link_name="glGetFloatv"]
            pub fn GetFloatv(pname: types::GLenum, data: *mut types::GLfloat) -> ();
#[link_name="glGetFragDataIndex"]
            pub fn GetFragDataIndex(program: types::GLuint, name: *const types::GLchar) -> types::GLint;
#[link_name="glGetFragDataLocation"]
            pub fn GetFragDataLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint;
#[link_name="glGetFramebufferAttachmentParameteriv"]
            pub fn GetFramebufferAttachmentParameteriv(target: types::GLenum, attachment: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetInteger64i_v"]
            pub fn GetInteger64i_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint64) -> ();
#[link_name="glGetInteger64v"]
            pub fn GetInteger64v(pname: types::GLenum, data: *mut types::GLint64) -> ();
#[link_name="glGetIntegeri_v"]
            pub fn GetIntegeri_v(target: types::GLenum, index: types::GLuint, data: *mut types::GLint) -> ();
#[link_name="glGetIntegerv"]
            pub fn GetIntegerv(pname: types::GLenum, data: *mut types::GLint) -> ();
#[link_name="glGetMultisamplefv"]
            pub fn GetMultisamplefv(pname: types::GLenum, index: types::GLuint, val: *mut types::GLfloat) -> ();
#[link_name="glGetProgramInfoLog"]
            pub fn GetProgramInfoLog(program: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> ();
#[link_name="glGetProgramiv"]
            pub fn GetProgramiv(program: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetQueryObjecti64v"]
            pub fn GetQueryObjecti64v(id: types::GLuint, pname: types::GLenum, params: *mut types::GLint64) -> ();
#[link_name="glGetQueryObjectiv"]
            pub fn GetQueryObjectiv(id: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetQueryObjectui64v"]
            pub fn GetQueryObjectui64v(id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint64) -> ();
#[link_name="glGetQueryObjectuiv"]
            pub fn GetQueryObjectuiv(id: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> ();
#[link_name="glGetQueryiv"]
            pub fn GetQueryiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetRenderbufferParameteriv"]
            pub fn GetRenderbufferParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetSamplerParameterIiv"]
            pub fn GetSamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetSamplerParameterIuiv"]
            pub fn GetSamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> ();
#[link_name="glGetSamplerParameterfv"]
            pub fn GetSamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetSamplerParameteriv"]
            pub fn GetSamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetShaderInfoLog"]
            pub fn GetShaderInfoLog(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, infoLog: *mut types::GLchar) -> ();
#[link_name="glGetShaderSource"]
            pub fn GetShaderSource(shader: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, source: *mut types::GLchar) -> ();
#[link_name="glGetShaderiv"]
            pub fn GetShaderiv(shader: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetString"]
            pub fn GetString(name: types::GLenum) -> *const types::GLubyte;
#[link_name="glGetStringi"]
            pub fn GetStringi(name: types::GLenum, index: types::GLuint) -> *const types::GLubyte;
#[link_name="glGetSynciv"]
            pub fn GetSynciv(sync: types::GLsync, pname: types::GLenum, bufSize: types::GLsizei, length: *mut types::GLsizei, values: *mut types::GLint) -> ();
#[link_name="glGetTexImage"]
            pub fn GetTexImage(target: types::GLenum, level: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetTexLevelParameterfv"]
            pub fn GetTexLevelParameterfv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetTexLevelParameteriv"]
            pub fn GetTexLevelParameteriv(target: types::GLenum, level: types::GLint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetTexParameterIiv"]
            pub fn GetTexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetTexParameterIuiv"]
            pub fn GetTexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLuint) -> ();
#[link_name="glGetTexParameterfv"]
            pub fn GetTexParameterfv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetTexParameteriv"]
            pub fn GetTexParameteriv(target: types::GLenum, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetTransformFeedbackVarying"]
            pub fn GetTransformFeedbackVarying(program: types::GLuint, index: types::GLuint, bufSize: types::GLsizei, length: *mut types::GLsizei, size: *mut types::GLsizei, type_: *mut types::GLenum, name: *mut types::GLchar) -> ();
#[link_name="glGetUniformBlockIndex"]
            pub fn GetUniformBlockIndex(program: types::GLuint, uniformBlockName: *const types::GLchar) -> types::GLuint;
#[link_name="glGetUniformIndices"]
            pub fn GetUniformIndices(program: types::GLuint, uniformCount: types::GLsizei, uniformNames: *const *const types::GLchar, uniformIndices: *mut types::GLuint) -> ();
#[link_name="glGetUniformLocation"]
            pub fn GetUniformLocation(program: types::GLuint, name: *const types::GLchar) -> types::GLint;
#[link_name="glGetUniformfv"]
            pub fn GetUniformfv(program: types::GLuint, location: types::GLint, params: *mut types::GLfloat) -> ();
#[link_name="glGetUniformiv"]
            pub fn GetUniformiv(program: types::GLuint, location: types::GLint, params: *mut types::GLint) -> ();
#[link_name="glGetUniformuiv"]
            pub fn GetUniformuiv(program: types::GLuint, location: types::GLint, params: *mut types::GLuint) -> ();
#[link_name="glGetVertexAttribIiv"]
            pub fn GetVertexAttribIiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glGetVertexAttribIuiv"]
            pub fn GetVertexAttribIuiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLuint) -> ();
#[link_name="glGetVertexAttribPointerv"]
            pub fn GetVertexAttribPointerv(index: types::GLuint, pname: types::GLenum, pointer: *const *mut __gl_imports::raw::c_void) -> ();
#[link_name="glGetVertexAttribdv"]
            pub fn GetVertexAttribdv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLdouble) -> ();
#[link_name="glGetVertexAttribfv"]
            pub fn GetVertexAttribfv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLfloat) -> ();
#[link_name="glGetVertexAttribiv"]
            pub fn GetVertexAttribiv(index: types::GLuint, pname: types::GLenum, params: *mut types::GLint) -> ();
#[link_name="glHint"]
            pub fn Hint(target: types::GLenum, mode: types::GLenum) -> ();
#[link_name="glIsBuffer"]
            pub fn IsBuffer(buffer: types::GLuint) -> types::GLboolean;
#[link_name="glIsEnabled"]
            pub fn IsEnabled(cap: types::GLenum) -> types::GLboolean;
#[link_name="glIsEnabledi"]
            pub fn IsEnabledi(target: types::GLenum, index: types::GLuint) -> types::GLboolean;
#[link_name="glIsFramebuffer"]
            pub fn IsFramebuffer(framebuffer: types::GLuint) -> types::GLboolean;
#[link_name="glIsProgram"]
            pub fn IsProgram(program: types::GLuint) -> types::GLboolean;
#[link_name="glIsQuery"]
            pub fn IsQuery(id: types::GLuint) -> types::GLboolean;
#[link_name="glIsRenderbuffer"]
            pub fn IsRenderbuffer(renderbuffer: types::GLuint) -> types::GLboolean;
#[link_name="glIsSampler"]
            pub fn IsSampler(sampler: types::GLuint) -> types::GLboolean;
#[link_name="glIsShader"]
            pub fn IsShader(shader: types::GLuint) -> types::GLboolean;
#[link_name="glIsSync"]
            pub fn IsSync(sync: types::GLsync) -> types::GLboolean;
#[link_name="glIsTexture"]
            pub fn IsTexture(texture: types::GLuint) -> types::GLboolean;
#[link_name="glIsVertexArray"]
            pub fn IsVertexArray(array: types::GLuint) -> types::GLboolean;
#[link_name="glLineWidth"]
            pub fn LineWidth(width: types::GLfloat) -> ();
#[link_name="glLinkProgram"]
            pub fn LinkProgram(program: types::GLuint) -> ();
#[link_name="glLogicOp"]
            pub fn LogicOp(opcode: types::GLenum) -> ();
#[link_name="glMapBuffer"]
            pub fn MapBuffer(target: types::GLenum, access: types::GLenum) -> *mut __gl_imports::raw::c_void;
#[link_name="glMapBufferRange"]
            pub fn MapBufferRange(target: types::GLenum, offset: types::GLintptr, length: types::GLsizeiptr, access: types::GLbitfield) -> *mut __gl_imports::raw::c_void;
#[link_name="glMultiDrawArrays"]
            pub fn MultiDrawArrays(mode: types::GLenum, first: *const types::GLint, count: *const types::GLsizei, drawcount: types::GLsizei) -> ();
#[link_name="glMultiDrawElements"]
            pub fn MultiDrawElements(mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei) -> ();
#[link_name="glMultiDrawElementsBaseVertex"]
            pub fn MultiDrawElementsBaseVertex(mode: types::GLenum, count: *const types::GLsizei, type_: types::GLenum, indices: *const *const __gl_imports::raw::c_void, drawcount: types::GLsizei, basevertex: *const types::GLint) -> ();
#[link_name="glMultiTexCoordP1ui"]
            pub fn MultiTexCoordP1ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glMultiTexCoordP1uiv"]
            pub fn MultiTexCoordP1uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glMultiTexCoordP2ui"]
            pub fn MultiTexCoordP2ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glMultiTexCoordP2uiv"]
            pub fn MultiTexCoordP2uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glMultiTexCoordP3ui"]
            pub fn MultiTexCoordP3ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glMultiTexCoordP3uiv"]
            pub fn MultiTexCoordP3uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glMultiTexCoordP4ui"]
            pub fn MultiTexCoordP4ui(texture: types::GLenum, type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glMultiTexCoordP4uiv"]
            pub fn MultiTexCoordP4uiv(texture: types::GLenum, type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glNormalP3ui"]
            pub fn NormalP3ui(type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glNormalP3uiv"]
            pub fn NormalP3uiv(type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glPixelStoref"]
            pub fn PixelStoref(pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glPixelStorei"]
            pub fn PixelStorei(pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glPointParameterf"]
            pub fn PointParameterf(pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glPointParameterfv"]
            pub fn PointParameterfv(pname: types::GLenum, params: *const types::GLfloat) -> ();
#[link_name="glPointParameteri"]
            pub fn PointParameteri(pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glPointParameteriv"]
            pub fn PointParameteriv(pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glPointSize"]
            pub fn PointSize(size: types::GLfloat) -> ();
#[link_name="glPolygonMode"]
            pub fn PolygonMode(face: types::GLenum, mode: types::GLenum) -> ();
#[link_name="glPolygonOffset"]
            pub fn PolygonOffset(factor: types::GLfloat, units: types::GLfloat) -> ();
#[link_name="glPrimitiveRestartIndex"]
            pub fn PrimitiveRestartIndex(index: types::GLuint) -> ();
#[link_name="glProvokingVertex"]
            pub fn ProvokingVertex(mode: types::GLenum) -> ();
#[link_name="glQueryCounter"]
            pub fn QueryCounter(id: types::GLuint, target: types::GLenum) -> ();
#[link_name="glReadBuffer"]
            pub fn ReadBuffer(src: types::GLenum) -> ();
#[link_name="glReadPixels"]
            pub fn ReadPixels(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei, format: types::GLenum, type_: types::GLenum, pixels: *mut __gl_imports::raw::c_void) -> ();
#[link_name="glRenderbufferStorage"]
            pub fn RenderbufferStorage(target: types::GLenum, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> ();
#[link_name="glRenderbufferStorageMultisample"]
            pub fn RenderbufferStorageMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei) -> ();
#[link_name="glSampleCoverage"]
            pub fn SampleCoverage(value: types::GLfloat, invert: types::GLboolean) -> ();
#[link_name="glSampleMaski"]
            pub fn SampleMaski(maskNumber: types::GLuint, mask: types::GLbitfield) -> ();
#[link_name="glSamplerParameterIiv"]
            pub fn SamplerParameterIiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> ();
#[link_name="glSamplerParameterIuiv"]
            pub fn SamplerParameterIuiv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLuint) -> ();
#[link_name="glSamplerParameterf"]
            pub fn SamplerParameterf(sampler: types::GLuint, pname: types::GLenum, param: types::GLfloat) -> ();
#[link_name="glSamplerParameterfv"]
            pub fn SamplerParameterfv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLfloat) -> ();
#[link_name="glSamplerParameteri"]
            pub fn SamplerParameteri(sampler: types::GLuint, pname: types::GLenum, param: types::GLint) -> ();
#[link_name="glSamplerParameteriv"]
            pub fn SamplerParameteriv(sampler: types::GLuint, pname: types::GLenum, param: *const types::GLint) -> ();
#[link_name="glScissor"]
            pub fn Scissor(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> ();
#[link_name="glSecondaryColorP3ui"]
            pub fn SecondaryColorP3ui(type_: types::GLenum, color: types::GLuint) -> ();
#[link_name="glSecondaryColorP3uiv"]
            pub fn SecondaryColorP3uiv(type_: types::GLenum, color: *const types::GLuint) -> ();
#[link_name="glShaderSource"]
            pub fn ShaderSource(shader: types::GLuint, count: types::GLsizei, string: *const *const types::GLchar, length: *const types::GLint) -> ();
#[link_name="glStencilFunc"]
            pub fn StencilFunc(func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> ();
#[link_name="glStencilFuncSeparate"]
            pub fn StencilFuncSeparate(face: types::GLenum, func: types::GLenum, ref_: types::GLint, mask: types::GLuint) -> ();
#[link_name="glStencilMask"]
            pub fn StencilMask(mask: types::GLuint) -> ();
#[link_name="glStencilMaskSeparate"]
            pub fn StencilMaskSeparate(face: types::GLenum, mask: types::GLuint) -> ();
#[link_name="glStencilOp"]
            pub fn StencilOp(fail: types::GLenum, zfail: types::GLenum, zpass: types::GLenum) -> ();
#[link_name="glStencilOpSeparate"]
            pub fn StencilOpSeparate(face: types::GLenum, sfail: types::GLenum, dpfail: types::GLenum, dppass: types::GLenum) -> ();
#[link_name="glTexBuffer"]
            pub fn TexBuffer(target: types::GLenum, internalformat: types::GLenum, buffer: types::GLuint) -> ();
#[link_name="glTexCoordP1ui"]
            pub fn TexCoordP1ui(type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glTexCoordP1uiv"]
            pub fn TexCoordP1uiv(type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glTexCoordP2ui"]
            pub fn TexCoordP2ui(type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glTexCoordP2uiv"]
            pub fn TexCoordP2uiv(type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glTexCoordP3ui"]
            pub fn TexCoordP3ui(type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glTexCoordP3uiv"]
            pub fn TexCoordP3uiv(type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glTexCoordP4ui"]
            pub fn TexCoordP4ui(type_: types::GLenum, coords: types::GLuint) -> ();
#[link_name="glTexCoordP4uiv"]
            pub fn TexCoordP4uiv(type_: types::GLenum, coords: *const types::GLuint) -> ();
#[link_name="glTexImage1D"]
            pub fn TexImage1D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexImage2D"]
            pub fn TexImage2D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexImage2DMultisample"]
            pub fn TexImage2DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, fixedsamplelocations: types::GLboolean) -> ();
#[link_name="glTexImage3D"]
            pub fn TexImage3D(target: types::GLenum, level: types::GLint, internalformat: types::GLint, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, border: types::GLint, format: types::GLenum, type_: types::GLenum, pixels: *const __gl_imports::raw::c_void) -> ();
#[link_name="glTexImage3DMultisample"]
            pub fn TexImage3DMultisample(target: types::GLenum, samples: types::GLsizei, internalformat: types::GLenum, width: types::GLsizei, height: types::GLsizei, depth: types::GLsizei, fixedsamplelocations: types::GLboolean) -> ();
#[link_name="glTexParameterIiv"]
            pub fn TexParameterIiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLint) -> ();
#[link_name="glTexParameterIuiv"]
            pub fn TexParameterIuiv(target: types::GLenum, pname: types::GLenum, params: *const types::GLuint) -> ();
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
#[link_name="glTransformFeedbackVaryings"]
            pub fn TransformFeedbackVaryings(program: types::GLuint, count: types::GLsizei, varyings: *const *const types::GLchar, bufferMode: types::GLenum) -> ();
#[link_name="glUniform1f"]
            pub fn Uniform1f(location: types::GLint, v0: types::GLfloat) -> ();
#[link_name="glUniform1fv"]
            pub fn Uniform1fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> ();
#[link_name="glUniform1i"]
            pub fn Uniform1i(location: types::GLint, v0: types::GLint) -> ();
#[link_name="glUniform1iv"]
            pub fn Uniform1iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> ();
#[link_name="glUniform1ui"]
            pub fn Uniform1ui(location: types::GLint, v0: types::GLuint) -> ();
#[link_name="glUniform1uiv"]
            pub fn Uniform1uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> ();
#[link_name="glUniform2f"]
            pub fn Uniform2f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat) -> ();
#[link_name="glUniform2fv"]
            pub fn Uniform2fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> ();
#[link_name="glUniform2i"]
            pub fn Uniform2i(location: types::GLint, v0: types::GLint, v1: types::GLint) -> ();
#[link_name="glUniform2iv"]
            pub fn Uniform2iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> ();
#[link_name="glUniform2ui"]
            pub fn Uniform2ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint) -> ();
#[link_name="glUniform2uiv"]
            pub fn Uniform2uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> ();
#[link_name="glUniform3f"]
            pub fn Uniform3f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat) -> ();
#[link_name="glUniform3fv"]
            pub fn Uniform3fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> ();
#[link_name="glUniform3i"]
            pub fn Uniform3i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint) -> ();
#[link_name="glUniform3iv"]
            pub fn Uniform3iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> ();
#[link_name="glUniform3ui"]
            pub fn Uniform3ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint) -> ();
#[link_name="glUniform3uiv"]
            pub fn Uniform3uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> ();
#[link_name="glUniform4f"]
            pub fn Uniform4f(location: types::GLint, v0: types::GLfloat, v1: types::GLfloat, v2: types::GLfloat, v3: types::GLfloat) -> ();
#[link_name="glUniform4fv"]
            pub fn Uniform4fv(location: types::GLint, count: types::GLsizei, value: *const types::GLfloat) -> ();
#[link_name="glUniform4i"]
            pub fn Uniform4i(location: types::GLint, v0: types::GLint, v1: types::GLint, v2: types::GLint, v3: types::GLint) -> ();
#[link_name="glUniform4iv"]
            pub fn Uniform4iv(location: types::GLint, count: types::GLsizei, value: *const types::GLint) -> ();
#[link_name="glUniform4ui"]
            pub fn Uniform4ui(location: types::GLint, v0: types::GLuint, v1: types::GLuint, v2: types::GLuint, v3: types::GLuint) -> ();
#[link_name="glUniform4uiv"]
            pub fn Uniform4uiv(location: types::GLint, count: types::GLsizei, value: *const types::GLuint) -> ();
#[link_name="glUniformBlockBinding"]
            pub fn UniformBlockBinding(program: types::GLuint, uniformBlockIndex: types::GLuint, uniformBlockBinding: types::GLuint) -> ();
#[link_name="glUniformMatrix2fv"]
            pub fn UniformMatrix2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix2x3fv"]
            pub fn UniformMatrix2x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix2x4fv"]
            pub fn UniformMatrix2x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix3fv"]
            pub fn UniformMatrix3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix3x2fv"]
            pub fn UniformMatrix3x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix3x4fv"]
            pub fn UniformMatrix3x4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix4fv"]
            pub fn UniformMatrix4fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix4x2fv"]
            pub fn UniformMatrix4x2fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUniformMatrix4x3fv"]
            pub fn UniformMatrix4x3fv(location: types::GLint, count: types::GLsizei, transpose: types::GLboolean, value: *const types::GLfloat) -> ();
#[link_name="glUnmapBuffer"]
            pub fn UnmapBuffer(target: types::GLenum) -> types::GLboolean;
#[link_name="glUseProgram"]
            pub fn UseProgram(program: types::GLuint) -> ();
#[link_name="glValidateProgram"]
            pub fn ValidateProgram(program: types::GLuint) -> ();
#[link_name="glVertexAttrib1d"]
            pub fn VertexAttrib1d(index: types::GLuint, x: types::GLdouble) -> ();
#[link_name="glVertexAttrib1dv"]
            pub fn VertexAttrib1dv(index: types::GLuint, v: *const types::GLdouble) -> ();
#[link_name="glVertexAttrib1f"]
            pub fn VertexAttrib1f(index: types::GLuint, x: types::GLfloat) -> ();
#[link_name="glVertexAttrib1fv"]
            pub fn VertexAttrib1fv(index: types::GLuint, v: *const types::GLfloat) -> ();
#[link_name="glVertexAttrib1s"]
            pub fn VertexAttrib1s(index: types::GLuint, x: types::GLshort) -> ();
#[link_name="glVertexAttrib1sv"]
            pub fn VertexAttrib1sv(index: types::GLuint, v: *const types::GLshort) -> ();
#[link_name="glVertexAttrib2d"]
            pub fn VertexAttrib2d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble) -> ();
#[link_name="glVertexAttrib2dv"]
            pub fn VertexAttrib2dv(index: types::GLuint, v: *const types::GLdouble) -> ();
#[link_name="glVertexAttrib2f"]
            pub fn VertexAttrib2f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat) -> ();
#[link_name="glVertexAttrib2fv"]
            pub fn VertexAttrib2fv(index: types::GLuint, v: *const types::GLfloat) -> ();
#[link_name="glVertexAttrib2s"]
            pub fn VertexAttrib2s(index: types::GLuint, x: types::GLshort, y: types::GLshort) -> ();
#[link_name="glVertexAttrib2sv"]
            pub fn VertexAttrib2sv(index: types::GLuint, v: *const types::GLshort) -> ();
#[link_name="glVertexAttrib3d"]
            pub fn VertexAttrib3d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble) -> ();
#[link_name="glVertexAttrib3dv"]
            pub fn VertexAttrib3dv(index: types::GLuint, v: *const types::GLdouble) -> ();
#[link_name="glVertexAttrib3f"]
            pub fn VertexAttrib3f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat) -> ();
#[link_name="glVertexAttrib3fv"]
            pub fn VertexAttrib3fv(index: types::GLuint, v: *const types::GLfloat) -> ();
#[link_name="glVertexAttrib3s"]
            pub fn VertexAttrib3s(index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort) -> ();
#[link_name="glVertexAttrib3sv"]
            pub fn VertexAttrib3sv(index: types::GLuint, v: *const types::GLshort) -> ();
#[link_name="glVertexAttrib4Nbv"]
            pub fn VertexAttrib4Nbv(index: types::GLuint, v: *const types::GLbyte) -> ();
#[link_name="glVertexAttrib4Niv"]
            pub fn VertexAttrib4Niv(index: types::GLuint, v: *const types::GLint) -> ();
#[link_name="glVertexAttrib4Nsv"]
            pub fn VertexAttrib4Nsv(index: types::GLuint, v: *const types::GLshort) -> ();
#[link_name="glVertexAttrib4Nub"]
            pub fn VertexAttrib4Nub(index: types::GLuint, x: types::GLubyte, y: types::GLubyte, z: types::GLubyte, w: types::GLubyte) -> ();
#[link_name="glVertexAttrib4Nubv"]
            pub fn VertexAttrib4Nubv(index: types::GLuint, v: *const types::GLubyte) -> ();
#[link_name="glVertexAttrib4Nuiv"]
            pub fn VertexAttrib4Nuiv(index: types::GLuint, v: *const types::GLuint) -> ();
#[link_name="glVertexAttrib4Nusv"]
            pub fn VertexAttrib4Nusv(index: types::GLuint, v: *const types::GLushort) -> ();
#[link_name="glVertexAttrib4bv"]
            pub fn VertexAttrib4bv(index: types::GLuint, v: *const types::GLbyte) -> ();
#[link_name="glVertexAttrib4d"]
            pub fn VertexAttrib4d(index: types::GLuint, x: types::GLdouble, y: types::GLdouble, z: types::GLdouble, w: types::GLdouble) -> ();
#[link_name="glVertexAttrib4dv"]
            pub fn VertexAttrib4dv(index: types::GLuint, v: *const types::GLdouble) -> ();
#[link_name="glVertexAttrib4f"]
            pub fn VertexAttrib4f(index: types::GLuint, x: types::GLfloat, y: types::GLfloat, z: types::GLfloat, w: types::GLfloat) -> ();
#[link_name="glVertexAttrib4fv"]
            pub fn VertexAttrib4fv(index: types::GLuint, v: *const types::GLfloat) -> ();
#[link_name="glVertexAttrib4iv"]
            pub fn VertexAttrib4iv(index: types::GLuint, v: *const types::GLint) -> ();
#[link_name="glVertexAttrib4s"]
            pub fn VertexAttrib4s(index: types::GLuint, x: types::GLshort, y: types::GLshort, z: types::GLshort, w: types::GLshort) -> ();
#[link_name="glVertexAttrib4sv"]
            pub fn VertexAttrib4sv(index: types::GLuint, v: *const types::GLshort) -> ();
#[link_name="glVertexAttrib4ubv"]
            pub fn VertexAttrib4ubv(index: types::GLuint, v: *const types::GLubyte) -> ();
#[link_name="glVertexAttrib4uiv"]
            pub fn VertexAttrib4uiv(index: types::GLuint, v: *const types::GLuint) -> ();
#[link_name="glVertexAttrib4usv"]
            pub fn VertexAttrib4usv(index: types::GLuint, v: *const types::GLushort) -> ();
#[link_name="glVertexAttribDivisor"]
            pub fn VertexAttribDivisor(index: types::GLuint, divisor: types::GLuint) -> ();
#[link_name="glVertexAttribI1i"]
            pub fn VertexAttribI1i(index: types::GLuint, x: types::GLint) -> ();
#[link_name="glVertexAttribI1iv"]
            pub fn VertexAttribI1iv(index: types::GLuint, v: *const types::GLint) -> ();
#[link_name="glVertexAttribI1ui"]
            pub fn VertexAttribI1ui(index: types::GLuint, x: types::GLuint) -> ();
#[link_name="glVertexAttribI1uiv"]
            pub fn VertexAttribI1uiv(index: types::GLuint, v: *const types::GLuint) -> ();
#[link_name="glVertexAttribI2i"]
            pub fn VertexAttribI2i(index: types::GLuint, x: types::GLint, y: types::GLint) -> ();
#[link_name="glVertexAttribI2iv"]
            pub fn VertexAttribI2iv(index: types::GLuint, v: *const types::GLint) -> ();
#[link_name="glVertexAttribI2ui"]
            pub fn VertexAttribI2ui(index: types::GLuint, x: types::GLuint, y: types::GLuint) -> ();
#[link_name="glVertexAttribI2uiv"]
            pub fn VertexAttribI2uiv(index: types::GLuint, v: *const types::GLuint) -> ();
#[link_name="glVertexAttribI3i"]
            pub fn VertexAttribI3i(index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint) -> ();
#[link_name="glVertexAttribI3iv"]
            pub fn VertexAttribI3iv(index: types::GLuint, v: *const types::GLint) -> ();
#[link_name="glVertexAttribI3ui"]
            pub fn VertexAttribI3ui(index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint) -> ();
#[link_name="glVertexAttribI3uiv"]
            pub fn VertexAttribI3uiv(index: types::GLuint, v: *const types::GLuint) -> ();
#[link_name="glVertexAttribI4bv"]
            pub fn VertexAttribI4bv(index: types::GLuint, v: *const types::GLbyte) -> ();
#[link_name="glVertexAttribI4i"]
            pub fn VertexAttribI4i(index: types::GLuint, x: types::GLint, y: types::GLint, z: types::GLint, w: types::GLint) -> ();
#[link_name="glVertexAttribI4iv"]
            pub fn VertexAttribI4iv(index: types::GLuint, v: *const types::GLint) -> ();
#[link_name="glVertexAttribI4sv"]
            pub fn VertexAttribI4sv(index: types::GLuint, v: *const types::GLshort) -> ();
#[link_name="glVertexAttribI4ubv"]
            pub fn VertexAttribI4ubv(index: types::GLuint, v: *const types::GLubyte) -> ();
#[link_name="glVertexAttribI4ui"]
            pub fn VertexAttribI4ui(index: types::GLuint, x: types::GLuint, y: types::GLuint, z: types::GLuint, w: types::GLuint) -> ();
#[link_name="glVertexAttribI4uiv"]
            pub fn VertexAttribI4uiv(index: types::GLuint, v: *const types::GLuint) -> ();
#[link_name="glVertexAttribI4usv"]
            pub fn VertexAttribI4usv(index: types::GLuint, v: *const types::GLushort) -> ();
#[link_name="glVertexAttribIPointer"]
            pub fn VertexAttribIPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glVertexAttribP1ui"]
            pub fn VertexAttribP1ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> ();
#[link_name="glVertexAttribP1uiv"]
            pub fn VertexAttribP1uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> ();
#[link_name="glVertexAttribP2ui"]
            pub fn VertexAttribP2ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> ();
#[link_name="glVertexAttribP2uiv"]
            pub fn VertexAttribP2uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> ();
#[link_name="glVertexAttribP3ui"]
            pub fn VertexAttribP3ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> ();
#[link_name="glVertexAttribP3uiv"]
            pub fn VertexAttribP3uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> ();
#[link_name="glVertexAttribP4ui"]
            pub fn VertexAttribP4ui(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: types::GLuint) -> ();
#[link_name="glVertexAttribP4uiv"]
            pub fn VertexAttribP4uiv(index: types::GLuint, type_: types::GLenum, normalized: types::GLboolean, value: *const types::GLuint) -> ();
#[link_name="glVertexAttribPointer"]
            pub fn VertexAttribPointer(index: types::GLuint, size: types::GLint, type_: types::GLenum, normalized: types::GLboolean, stride: types::GLsizei, pointer: *const __gl_imports::raw::c_void) -> ();
#[link_name="glVertexP2ui"]
            pub fn VertexP2ui(type_: types::GLenum, value: types::GLuint) -> ();
#[link_name="glVertexP2uiv"]
            pub fn VertexP2uiv(type_: types::GLenum, value: *const types::GLuint) -> ();
#[link_name="glVertexP3ui"]
            pub fn VertexP3ui(type_: types::GLenum, value: types::GLuint) -> ();
#[link_name="glVertexP3uiv"]
            pub fn VertexP3uiv(type_: types::GLenum, value: *const types::GLuint) -> ();
#[link_name="glVertexP4ui"]
            pub fn VertexP4ui(type_: types::GLenum, value: types::GLuint) -> ();
#[link_name="glVertexP4uiv"]
            pub fn VertexP4uiv(type_: types::GLenum, value: *const types::GLuint) -> ();
#[link_name="glViewport"]
            pub fn Viewport(x: types::GLint, y: types::GLint, width: types::GLsizei, height: types::GLsizei) -> ();
#[link_name="glWaitSync"]
            pub fn WaitSync(sync: types::GLsync, flags: types::GLbitfield, timeout: types::GLuint64) -> ();
}

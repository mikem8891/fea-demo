// @ts-check

/**
 * @param {string} url 
 */
export async function text(url) {
  let responce = await fetch(url);
  let status = responce.status;
  if (200 <= status && status < 300){
    return responce.text();
  } else {
    throw new Error(`Failed to load text from: "${url}"`);
  }
}

/**
 * @param {WebGLRenderingContext | WebGL2RenderingContext} gl
 * @param {string} vertex vertex shader source code
 * @param {string} fragment fragment shader source code
 */
export function shaderProgram(gl, vertex, fragment) {
  // Create blank shaders objects
  const vertexShader = gl.createShader(gl.VERTEX_SHADER);
  if (!vertexShader)
    throw new Error("failed to create vertex shader");
  const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
  if (!fragmentShader)
    throw new Error("failed to create fragment shader");
  compileShader(gl, vertexShader, vertex);
  compileShader(gl, fragmentShader, fragment);
  // Create and link the program to run the shaders
  const program = gl.createProgram();
  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);
  gl.linkProgram(program);
  // Check for linking and validation errors
  if (!gl.getProgramParameter(program, gl.LINK_STATUS)){
    throw Error(`ERROR linking program\n\n${gl.getProgramInfoLog(program)}`);
  }
  gl.validateProgram(program);
  if (!gl.getProgramParameter(program, gl.VALIDATE_STATUS)){
    throw Error(`ERROR validating program\n\n${gl.getProgramInfoLog(program)}`);
  }
  return program;
}

/**
 * @param {WebGLRenderingContext | WebGL2RenderingContext} gl
 * @param {WebGLShader} shader
 * @param {string} source
 */
function compileShader(gl, shader, source) {
  // Set shader source code
  gl.shaderSource(shader, source);
  // Compiler shader source code
  gl.compileShader(shader);
  // Check for compilation errors
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)){
    const message = gl.getShaderInfoLog(shader);
    throw new Error(`ERROR compiling shader!\n\n${message}`);
  }
}
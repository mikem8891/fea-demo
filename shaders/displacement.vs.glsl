#version 300 es

precision mediump float;
in vec2 vertPosition;
in vec2 vertDisp;
in vec3 vertStress;

out vec4 vertColor;

void main(){
    gl_Position = vec4(vertPosition + 100.0 * vertDisp, 0.0, 1.0);
    vertColor = vec4(0.5 + vertStress.xzy / 80.0, 1.0);
}

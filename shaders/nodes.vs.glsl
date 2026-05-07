#version 300 es

precision mediump float;

struct View {
    vec2 center;
    vec2 range;
};

uniform View view;
uniform float scaleDisp;

in vec2 vertPosition;
in vec2 vertDisp;

void main() {

    vec2 vertScaledPosition = vertPosition + scaleDisp * vertDisp;
    vec2 vertScreen = 2.0 * (vertScaledPosition - view.center) / view.range;
    gl_Position = vec4(vertScreen, 0.0, 1.0);
    gl_PointSize = 5.0;

}

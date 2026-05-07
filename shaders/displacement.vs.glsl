#version 300 es

precision mediump float;

struct View {
    vec2 center;
    vec2 range;
};

uniform View view;
uniform float scaleDisp;
uniform float maxStress;

in vec2 vertPosition;
in vec2 vertDisp;
in vec3 vertStress;

out vec4 vertColor;

void main() {

    vec2 vertScaledPosition = vertPosition + scaleDisp * vertDisp;
    vec2 vertScreen = 2.0 * (vertScaledPosition - view.center) / view.range;
    gl_Position = vec4(vertScreen, 0.0, 1.0);

    float sig = vertStress.x - vertStress.y;
    float tau = vertStress.z;
    float stressInt = sqrt(sig * sig + 4.0 * tau * tau);
    float red = stressInt / maxStress;
    float blue = 1.0 - red;
    red = red * red;
    blue = blue * blue;
    float green = 2.0 * (1.0 - red - blue);
    vertColor = vec4(red, green, blue, 1.0);
}

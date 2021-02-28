precision highp float;
uniform vec2 canvasSize;
uniform sampler2D state;
varying vec2 v_uv;

const float DEAD_COLOR = 0.0;
const float ALIVE_COLOR = 0.1;
const float STILL_ALIVE_COLOR = 0.2;

vec4 rgbToDec(float r, float g, float b) {
  return vec4(vec3(r, g, b) / 256.0, 1.0);
}

void main() {
  vec4 stillAlive = rgbToDec(21.0, 151.0, 183.0);
  vec4 alive = rgbToDec(143.0, 152.0, 171.0);
  vec4 dUp = rgbToDec(98.0, 252.0, 198.0);
  vec4 dMid = rgbToDec(51.0, 51.0, 153.0);
  vec4 dDown = rgbToDec(0.0, 0.0, 0.0);
  vec4 value = texture2D(state, v_uv);

  if (value.x == DEAD_COLOR) {
    float counter = value.y;
    if (counter <= 0.5) {
      gl_FragColor = mix(dUp, dMid, counter * 2.0);
    } else {
      gl_FragColor = mix(dMid, dDown, (counter - 0.5) * 2.0);
    }
  } else if (value.x == ALIVE_COLOR) {
    gl_FragColor = alive;
  } else if (value.x == STILL_ALIVE_COLOR) {
    gl_FragColor = stillAlive;
  }
}

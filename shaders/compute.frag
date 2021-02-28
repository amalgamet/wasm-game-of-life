precision highp float;

uniform vec2 canvasSize;
uniform sampler2D state;

varying vec2 v_uv;

const vec4 DEAD_COLOR = vec4(0.0, 0.0, 0.0, 1.0);
const vec4 ALIVE_COLOR = vec4(1.0, 1.0, 1.0, 1.0);

vec4 textureOffset(vec2 uv, vec2 offset) {
  return texture2D(state, (uv * canvasSize + offset) / canvasSize);
}

float num(vec4 v) {
  if (v.x > 0.0) {
    return 1.0;
  } else {
    return 0.0;
  }
}

float numNeighbors(vec2 v_uv) {
  float left = num(textureOffset(v_uv, vec2(1, 0)));
  float right = num(textureOffset(v_uv, vec2(-1, 0)));
  float up = num(textureOffset(v_uv, vec2(0, -1)));
  float down = num(textureOffset(v_uv, vec2(0, 1)));

  float leftUp = num(textureOffset(v_uv, vec2(1, -1)));
  float rightUp = num(textureOffset(v_uv, vec2(-1, -1)));
  float leftDown = num(textureOffset(v_uv, vec2(1, 1)));
  float rightDown = num(textureOffset(v_uv, vec2(-1, 1)));

  return left + right + up + down + leftUp + leftDown + rightUp + rightDown;
}

void main() {
  float neighbors = numNeighbors(v_uv);
  float myValue = num(texture2D(state, v_uv));

  if (myValue == 1.0 && neighbors < 2.0) {
    gl_FragColor = DEAD_COLOR;
  } else if (myValue == 1.0 && neighbors == 2.0 || myValue == 1.0 && neighbors == 3.0) {
    gl_FragColor = ALIVE_COLOR;
  } else if (myValue == 1.0 && neighbors > 3.0) {
    gl_FragColor = DEAD_COLOR;
  } else if (myValue == 0.0 && neighbors == 3.0) {
    gl_FragColor = ALIVE_COLOR;
  } else {
    gl_FragColor = vec4(ALIVE_COLOR.xyz * myValue, 1.0);
  }
}

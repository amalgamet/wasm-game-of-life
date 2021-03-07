precision highp float;

uniform sampler2D state;

varying vec2 v_uv;

void main() {
    gl_FragColor = texture2D(state, v_uv);
}

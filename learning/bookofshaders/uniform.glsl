#ifdef GL_ES
precision mediump float;
#endif

uniform float u_time;

void main() {
	gl_FragColor = vec4(abs( cos(10.0 * u_time)),abs(sin(3.0 * u_time)),abs(fract(u_time)),1.0);
}

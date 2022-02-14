#ifdef GL_ES
precision mediump float;
#endif

uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform float u_time;

void main() {
	vec2 st = gl_FragCoord.xy/u_resolution;
    vec2 m = u_mouse / u_resolution;
	gl_FragColor = vec4(st.x * abs(fract(m.y)),st.y * abs(cos(m.x * 2.0)),abs(sin(u_time)),1.0);
}
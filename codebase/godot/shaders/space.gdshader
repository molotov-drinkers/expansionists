shader_type sky;

uniform sampler2D sky_tex: source_color, filter_linear, hint_default_black;

void sky() {
	COLOR = texture(sky_tex, SKY_COORDS).rgb;
}

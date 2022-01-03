#version 330 core
precision lowp float;

in vec4 position;
in vec2 world_position;

out vec2 TexCoords;

uniform mat4 projection;
uniform mat4 model;

void main()
{
    TexCoords = position.zw;
    vec2 fragment_position = vec2(model * vec4(position.xy, 0.0, 1.0));
    gl_Position = projection * vec4(world_position + fragment_position, 0.0, 1.0);
}
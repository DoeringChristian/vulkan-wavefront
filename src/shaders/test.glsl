#version 450

layout(set = 0, binding = 0)buffer In{
    float i[];
};
layout(set = 0, binding = 1)buffer Out{
    float o[];
};

void main(){
    o[int(gl_GlobalInvocationID.x)] = o[int(gl_GlobalInvocationID.x)];
}
    

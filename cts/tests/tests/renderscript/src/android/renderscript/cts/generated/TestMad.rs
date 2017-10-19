/*
 * Copyright (C) 2016 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Don't edit this file!  It is auto-generated by frameworks/rs/api/generate.sh.

#pragma version(1)
#pragma rs java_package_name(android.renderscript.cts)

rs_allocation gAllocInMultiplicand2;
rs_allocation gAllocInOffset;

float __attribute__((kernel)) testMadFloatFloatFloatFloat(float inMultiplicand1, unsigned int x) {
    float inMultiplicand2 = rsGetElementAt_float(gAllocInMultiplicand2, x);
    float inOffset = rsGetElementAt_float(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}

float2 __attribute__((kernel)) testMadFloat2Float2Float2Float2(float2 inMultiplicand1, unsigned int x) {
    float2 inMultiplicand2 = rsGetElementAt_float2(gAllocInMultiplicand2, x);
    float2 inOffset = rsGetElementAt_float2(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}

float3 __attribute__((kernel)) testMadFloat3Float3Float3Float3(float3 inMultiplicand1, unsigned int x) {
    float3 inMultiplicand2 = rsGetElementAt_float3(gAllocInMultiplicand2, x);
    float3 inOffset = rsGetElementAt_float3(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}

float4 __attribute__((kernel)) testMadFloat4Float4Float4Float4(float4 inMultiplicand1, unsigned int x) {
    float4 inMultiplicand2 = rsGetElementAt_float4(gAllocInMultiplicand2, x);
    float4 inOffset = rsGetElementAt_float4(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}

half __attribute__((kernel)) testMadHalfHalfHalfHalf(half inMultiplicand1, unsigned int x) {
    half inMultiplicand2 = rsGetElementAt_half(gAllocInMultiplicand2, x);
    half inOffset = rsGetElementAt_half(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}

half2 __attribute__((kernel)) testMadHalf2Half2Half2Half2(half2 inMultiplicand1, unsigned int x) {
    half2 inMultiplicand2 = rsGetElementAt_half2(gAllocInMultiplicand2, x);
    half2 inOffset = rsGetElementAt_half2(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}

half3 __attribute__((kernel)) testMadHalf3Half3Half3Half3(half3 inMultiplicand1, unsigned int x) {
    half3 inMultiplicand2 = rsGetElementAt_half3(gAllocInMultiplicand2, x);
    half3 inOffset = rsGetElementAt_half3(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}

half4 __attribute__((kernel)) testMadHalf4Half4Half4Half4(half4 inMultiplicand1, unsigned int x) {
    half4 inMultiplicand2 = rsGetElementAt_half4(gAllocInMultiplicand2, x);
    half4 inOffset = rsGetElementAt_half4(gAllocInOffset, x);
    return mad(inMultiplicand1, inMultiplicand2, inOffset);
}
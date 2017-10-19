# Copyright (C) 2013 The Android Open Source Project
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

LOCAL_PATH := $(call my-dir)

include $(CLEAR_VARS)

# Only compile source java files in this apk.
LOCAL_SRC_FILES := $(call all-java-files-under, src)

# Adb test cases, but name 'aadb' ensures adb tests run before all other modules depending on adb
LOCAL_MODULE := CtsAadbHostTestCases

LOCAL_JAVA_LIBRARIES := cts-tradefed tradefed

LOCAL_CTS_TEST_PACKAGE := android.aadb

# tag this module as a cts test artifact
LOCAL_COMPATIBILITY_SUITE := cts

include $(BUILD_CTS_HOST_JAVA_LIBRARY)

# Build the test APKs using their own makefiles
include $(call all-makefiles-under,$(LOCAL_PATH))

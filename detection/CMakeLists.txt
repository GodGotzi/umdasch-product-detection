# CMakeList.txt: CMake-Projekt für "product-detection". Schließen Sie die Quelle ein, und definieren Sie
# projektspezifische Logik hier.
#
cmake_minimum_required (VERSION 3.8)

# Enable Hot Reload for MSVC compilers if supported.
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} "${CMAKE_SOURCE_DIR}/cmake/")

project ("detection")

find_package(OpenCV REQUIRED)

file(GLOB_RECURSE SOURCE_FILES 
	${CMAKE_SOURCE_DIR}/src/source/*.c
	${CMAKE_SOURCE_DIR}/src/source/*.cpp
	${CMAKE_SOURCE_DIR}/src/source/*/*.c
	${CMAKE_SOURCE_DIR}/src/source/*/*.cpp)

file(GLOB_RECURSE HEADER_FILES 
	${CMAKE_SOURCE_DIR}/src/include/*.h
	${CMAKE_SOURCE_DIR}/src/include/*.hpp
	${CMAKE_SOURCE_DIR}/src/include/*/*.h
	${CMAKE_SOURCE_DIR}/src/include/*/*.hpp)

include_directories(${CMAKE_SOURCE_DIR}/src/include opencv_world)
link_directories( ${CMAKE_BINARY_DIR}/bin)
add_executable(${PROJECT_NAME} ${HEADER_FILES} ${SOURCE_FILES} "src/source/detector.cpp" "src/source/preprocess.cpp" "src/include/preprocess.hpp" )

target_link_libraries(${PROJECT_NAME} opencv_ml opencv_dnn opencv_core opencv_flann)

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET detection PROPERTY CXX_STANDARD 20)
endif()

cmake_minimum_required(VERSION 3.13.0)

project(hedera)

include_directories(include)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY build/examples)

add_executable(generate_keys examples/generate_keys.c)

# fixme: platform dependent (this is osx specific)
find_library(SECURITY_FRAMEWORK Security)
target_link_libraries(generate_keys ${SECURITY_FRAMEWORK})

# fixme: profile dependent (debug vs. release)
target_link_directories(generate_keys PRIVATE target/release)
target_link_libraries(generate_keys hedera)

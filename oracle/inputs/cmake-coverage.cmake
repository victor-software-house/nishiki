message(STATUS "Home is" $ENV{HOME})
#[[ A bracket comment, with an escape \] inside ]]
set(raw [=[ A bracket argument, with an escape \n inside ]=])
set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES extra.txt)
message(\"an escaped string, with \t inside\")
message(BUILD_NAME)

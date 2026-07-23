cmake_minimum_required(VERSION 3.21)

# Standalone Qt-for-WebAssembly build of OMShell. Unlike the native build this
# does NOT link OpenModelicaCompiler: in the browser omc runs as a separate wasm
# module in a Web Worker, which omcinteractiveenvironment.cpp talks to over
# postMessage (Asyncify). Configure with the Qt wasm toolchain, e.g.
#   <qt-wasm>/bin/qt-cmake -G "Unix Makefiles" -S . -B build
project(OMShellQtWasm LANGUAGES CXX)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets PrintSupport Xml)
qt_standard_project_setup()

set(SRC ${CMAKE_CURRENT_SOURCE_DIR}/..)

qt_add_executable(OMShell-qt
  ${SRC}/main.cpp
  ${SRC}/commandcompletion.cpp
  ${SRC}/omcinteractiveenvironment.cpp
  ${SRC}/oms.cpp
  ${SRC}/oms.qrc)

target_include_directories(OMShell-qt PRIVATE ${SRC})
target_link_libraries(OMShell-qt PRIVATE Qt6::Widgets Qt6::PrintSupport Qt6::Xml)

# Asyncify lets the synchronous evalExpression() suspend while it awaits the omc
# Web Worker. EXPORTED_RUNTIME_METHODS makes the string helpers used by EM_JS
# available; ALLOW_MEMORY_GROWTH because omc + the MSL need a large heap.
target_link_options(OMShell-qt PRIVATE
  -sASYNCIFY
  -sASYNCIFY_STACK_SIZE=131072
  -sALLOW_MEMORY_GROWTH=1
  -sEXPORTED_RUNTIME_METHODS=UTF8ToString,stringToNewUTF8)

# Multithread Qt build (OMSHELL_QT_WASM_PREFIX -> wasm_multithread): omc calls run
# on a secondary "OMC thread" that blocks on an Atomics futex instead of Asyncify
# (omcinteractiveenvironment.cpp, guarded by __EMSCRIPTEN_PTHREADS__). Qt's
# toolchain already adds -pthread; pre-spawn a small pool so creating the OMC
# thread never stalls the main thread.
if(QT_FEATURE_thread)
  target_link_options(OMShell-qt PRIVATE -sPTHREAD_POOL_SIZE=4)
  # A plain macro (not the compiler builtin __EMSCRIPTEN_PTHREADS__) so AUTOMOC,
  # which does not see compiler builtins, also takes the channel branch and emits
  # moc for the OMC-thread QObject.
  target_compile_definitions(OMShell-qt PRIVATE OMC_WASM_THREADS)
endif()

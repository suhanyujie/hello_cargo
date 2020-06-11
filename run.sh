#!/usr/bin/env bash
#
# 编译一个 rust 源码文件并运行
scriptFile=$0
codeFile=$1
RUST_BACKTRACE=full rustc $codeFile -o a.out && ./a.out

# 参考 https://wangdoc.com/bash
# `$@` 表示脚本全部参数列表

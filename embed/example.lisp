#!/usr/bin/env sbcl --script

(load "~/quicklisp/setup.lisp")

(eval-when (:compile-toplevel :load-toplevel :execute)
  (ql:quickload "cffi"))

(defpackage :example (:use :cl :cffi))
(in-package :example)

(define-foreign-library embed (:darwin "target/release/libembed.dylib"))
(use-foreign-library embed)

(defcfun "process" :void)

(process)

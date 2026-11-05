;;;; Written for nishiki: the units of the common-lisp grammar that the
;;;; third-party fixtures leave out.

#| A block comment,
   #| with one nested inside |#
   that ends here. |#

(defun area (r)
  ;; A comment after blanks.
  (* pi r r))

(defun (setf point-x) (value point)
  (setf (car point) value))

(defmacro define-reader (name)
  `(defun (setf ,name) (value object)
     (setf (slot-value object ',name) value)))

(defmacro define-getter (name)
  `(defun ,name (object)
     (slot-value object ',name)))

(defconstant +limit+ 10)
(defconstant cl-user::+other-limit+ 20)

(defvar  *counter* 0)
(defparameter  *table* (make-hash-table))

(defstruct point x y)
(defstruct (circle (:conc-name c-)) radius)

(progn flet ((twice (x) (* 2 x)))
  (twice 3))

(defmethod update ((object point))
  (reinitialize-instance object)
  (typep object '(satisfies point-p)))

(defun shapes ()
  (list #2(1 2) #*1010 #3*101 #* #0* #2A((1 2) (3 4))
        #1=(a . #1#) 1. 1.5d0 1d0 -2f-3 .5s0))

(defun banners (stream n)
  (format stream "~2,'0[zero~;one~:;many~]" n)
  (format stream "~/cl-user::pprint-point/" n)
  (format stream "~1,2:@/cl-user::pprint-point/" n)
  (format stream "first line ~
                  second line~%"))

(defun quoted ()
  (list 'cl:*print-base* 'cl-user::+limit+ 'a:b:*c*))

(defun repl-values ()
  (list ** *** ++ +++ // ///))

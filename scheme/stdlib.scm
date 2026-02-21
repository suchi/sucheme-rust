;;; sucheme-rust Standard Library
;;; R5RS-compliant standard procedures implemented in Scheme.
;;; This file is loaded automatically at startup.

;;; ============================================================
;;; R5RS Appendix: Standard derived expressions (macro-based)
;;; ============================================================

;; (define-record-type name (constructor field ...) pred (field accessor [modifier]) ...)
;; Simplified implementation using vectors
(define-syntax define-record-type
  (syntax-rules ()
    ((define-record-type name
       (constructor field ...)
       pred
       (field-name accessor modifier) ...)
     (begin
       (define (constructor field ...)
         (vector 'name field ...))
       (define (pred obj)
         (and (vector? obj)
              (> (vector-length obj) 0)
              (eq? (vector-ref obj 0) 'name)))
       (define-record-fields 1 (field ...) (field-name accessor modifier) ...)))
    ((define-record-type name
       (constructor field ...)
       pred
       (field-name accessor) ...)
     (begin
       (define (constructor field ...)
         (vector 'name field ...))
       (define (pred obj)
         (and (vector? obj)
              (> (vector-length obj) 0)
              (eq? (vector-ref obj 0) 'name)))
       (define-record-fields 1 (field ...) (field-name accessor) ...)))))

(define-syntax define-record-fields
  (syntax-rules ()
    ((define-record-fields n () ) #t)
    ((define-record-fields n (f . rest) (field-name accessor modifier) . more-fields)
     (begin
       (define (accessor rec) (vector-ref rec n))
       (define (modifier rec val) (vector-set! rec n val))
       (define-record-fields (+ n 1) rest . more-fields)))
    ((define-record-fields n (f . rest) (field-name accessor) . more-fields)
     (begin
       (define (accessor rec) (vector-ref rec n))
       (define-record-fields (+ n 1) rest . more-fields)))))

;; R5RS Section 5.3: Sequence
(define-syntax begin
  (syntax-rules ()
    ((begin) (if #f #f))
    ((begin e) e)
    ((begin e1 e2 ...)
     (let ((x e1)) (begin e2 ...)))))

;; Not redefinition, but our eval handles begin natively.
;; Keep as documentation.

;;; ============================================================
;;; Boolean utilities
;;; ============================================================

(define (boolean=? . args)
  (or (null? args)
      (null? (cdr args))
      (and (eq? (car args) (cadr args))
           (apply boolean=? (cdr args)))))

;;; ============================================================
;;; List utilities
;;; ============================================================

(define (list? x)
  (let loop ((fast x) (slow x))
    (cond
      ((null? fast) #t)
      ((pair? fast)
       (let ((f2 (cdr fast)))
         (cond
           ((null? f2) #t)
           ((pair? f2)
            (if (eq? f2 slow)
                #f
                (loop (cdr f2) (cdr slow))))
           (else #f))))
      (else #f))))

(define (iota count . rest)
  (let ((start (if (null? rest) 0 (car rest)))
        (step  (if (or (null? rest) (null? (cdr rest))) 1 (cadr rest))))
    (let loop ((i 0) (acc '()))
      (if (>= i count)
          (reverse acc)
          (loop (+ i 1) (cons (+ start (* i step)) acc))))))

(define (last-pair lst)
  (if (null? (cdr lst))
      lst
      (last-pair (cdr lst))))

(define (flatten lst)
  (cond
    ((null? lst) '())
    ((pair? lst) (append (flatten (car lst)) (flatten (cdr lst))))
    (else (list lst))))

(define (any pred lst)
  (cond
    ((null? lst) #f)
    ((pred (car lst)) #t)
    (else (any pred (cdr lst)))))

(define (every pred lst)
  (cond
    ((null? lst) #t)
    ((pred (car lst)) (every pred (cdr lst)))
    (else #f)))

(define (count pred lst)
  (let loop ((l lst) (n 0))
    (cond
      ((null? l) n)
      ((pred (car l)) (loop (cdr l) (+ n 1)))
      (else (loop (cdr l) n)))))

(define (remove pred lst)
  (filter (lambda (x) (not (pred x))) lst))

(define (partition pred lst)
  (let loop ((l lst) (yes '()) (no '()))
    (cond
      ((null? l) (values (reverse yes) (reverse no)))
      ((pred (car l)) (loop (cdr l) (cons (car l) yes) no))
      (else (loop (cdr l) yes (cons (car l) no))))))

(define (take lst n)
  (if (or (= n 0) (null? lst))
      '()
      (cons (car lst) (take (cdr lst) (- n 1)))))

(define (drop lst n)
  (if (or (= n 0) (null? lst))
      lst
      (drop (cdr lst) (- n 1))))

(define (zip . lists)
  (apply map list lists))

(define (unzip1 lst) (map car lst))
(define (unzip2 lst) (values (map car lst) (map cadr lst)))

(define (list-index pred lst)
  (let loop ((l lst) (i 0))
    (cond
      ((null? l) #f)
      ((pred (car l)) i)
      (else (loop (cdr l) (+ i 1))))))

(define (delete x lst)
  (filter (lambda (e) (not (equal? e x))) lst))

(define (delete-duplicates lst)
  (let loop ((l lst) (seen '()))
    (cond
      ((null? l) (reverse seen))
      ((member (car l) seen) (loop (cdr l) seen))
      (else (loop (cdr l) (cons (car l) seen))))))

(define (sort lst less?)
  ;; Simple merge sort
  (define (merge a b)
    (cond
      ((null? a) b)
      ((null? b) a)
      ((less? (car a) (car b))
       (cons (car a) (merge (cdr a) b)))
      (else
       (cons (car b) (merge a (cdr b))))))
  (define (split lst)
    (let loop ((l lst) (a '()) (b '()))
      (if (null? l)
          (values a b)
          (loop (cdr l) b (cons (car l) a)))))
  (define (msort lst n)
    (if (<= n 1)
        lst
        (let-values (((a b) (split lst)))
          (merge (msort a (quotient n 2))
                 (msort b (- n (quotient n 2)))))))
  (msort lst (length lst)))

;;; ============================================================
;;; String utilities
;;; ============================================================

(define (string-contains str substr)
  (let* ((slen (string-length str))
         (plen (string-length substr)))
    (let loop ((i 0))
      (cond
        ((> (+ i plen) slen) #f)
        ((string=? (substring str i (+ i plen)) substr) i)
        (else (loop (+ i 1)))))))

(define (string-split str delim)
  (let ((dlen (string-length delim))
        (slen (string-length str)))
    (let loop ((start 0) (result '()))
      (let ((pos (string-contains str delim)))
        (if (not pos)
            (reverse (if (= start slen)
                         result
                         (cons (substring str start slen) result)))
            ;; Simple linear scan
            (let find ((i start))
              (cond
                ((> (+ i dlen) slen)
                 (reverse (cons (substring str start slen) result)))
                ((string=? (substring str i (+ i dlen)) delim)
                 (loop (+ i dlen) (cons (substring str start i) result)))
                (else (find (+ i 1))))))))))

(define (string-join strs . args)
  (let ((delim (if (null? args) "" (car args))))
    (if (null? strs)
        ""
        (let loop ((rest (cdr strs))
                   (acc (car strs)))
          (if (null? rest)
              acc
              (loop (cdr rest) (string-append acc delim (car rest))))))))

(define (string-trim str)
  (let* ((chars (string->list str))
         (front (let loop ((c chars))
                  (if (and (pair? c) (char-whitespace? (car c)))
                      (loop (cdr c))
                      c)))
         (back (reverse
                (let loop ((c (reverse front)))
                  (if (and (pair? c) (char-whitespace? (car c)))
                      (loop (cdr c))
                      c)))))
    (list->string back)))

(define (string-repeat str n)
  (let loop ((i n) (acc ""))
    (if (<= i 0)
        acc
        (loop (- i 1) (string-append acc str)))))

(define (string-prefix? prefix str)
  (let ((plen (string-length prefix))
        (slen (string-length str)))
    (and (<= plen slen)
         (string=? prefix (substring str 0 plen)))))

(define (string-suffix? suffix str)
  (let ((sufflen (string-length suffix))
        (slen (string-length str)))
    (and (<= sufflen slen)
         (string=? suffix (substring str (- slen sufflen) slen)))))

;;; ============================================================
;;; Math utilities
;;; ============================================================

(define (square x) (* x x))
(define (cube x) (* x x x))

(define (negate x) (- x))

(define (average . args)
  (/ (apply + args) (length args)))

(define (exact-integer? n)
  (and (integer? n) (exact? n)))

(define (zero? x) (= x 0))
(define (positive? x) (> x 0))
(define (negative? x) (< x 0))

;;; ============================================================
;;; Higher-order functions
;;; ============================================================

(define (compose . procs)
  (if (null? procs)
      (lambda (x) x)
      (let ((f (car procs))
            (g (apply compose (cdr procs))))
        (lambda args (f (apply g args))))))

(define (negate-pred pred)
  (lambda (x) (not (pred x))))

(define (constantly x)
  (lambda args x))

(define (identity x) x)

(define (flip f)
  (lambda (a b) (f b a)))

(define (curry f . initial-args)
  (lambda args (apply f (append initial-args args))))

;;; ============================================================
;;; I/O utilities
;;; ============================================================

(define (print . args)
  (for-each (lambda (x) (display x) (display " ")) args)
  (newline))

(define (println . args)
  (for-each display args)
  (newline))

(define (writeln . args)
  (for-each write args)
  (newline))

;;; ============================================================
;;; Association list utilities
;;; ============================================================

(define (alist->hash-table alist)
  ;; Represented as an alist, just return as-is
  alist)

(define (alist-ref key alist . default)
  (let ((pair (assoc key alist)))
    (if pair
        (cdr pair)
        (if (null? default)
            (error "alist-ref: key not found" key)
            (car default)))))

(define (alist-set key val alist)
  (cons (cons key val)
        (filter (lambda (pair) (not (equal? (car pair) key))) alist)))

(define (alist-delete key alist)
  (filter (lambda (pair) (not (equal? (car pair) key))) alist))

;;; ============================================================
;;; Numeric string conversions
;;; ============================================================

(define (number->padded-string n width char)
  (let* ((s (number->string n))
         (pad (- width (string-length s))))
    (if (<= pad 0)
        s
        (string-append (make-string pad char) s))))

;;; ============================================================
;;; Tail-call safe list processing
;;; ============================================================

(define (reduce f init lst)
  (if (null? lst)
      init
      (reduce f (f init (car lst)) (cdr lst))))

(define (reduce-right f init lst)
  (if (null? lst)
      init
      (f (car lst) (reduce-right f init (cdr lst)))))

;;; ============================================================
;;; R5RS Section 6.4: Control features
;;; ============================================================

;; string-for-each
(define (string-for-each f str . rest)
  (let ((chars (string->list str)))
    (if (null? rest)
        (for-each f chars)
        (apply for-each f chars (map string->list rest)))))

;; string-map
(define (string-map f str . rest)
  (let ((chars (string->list str)))
    (list->string
     (if (null? rest)
         (map f chars)
         (apply map f chars (map string->list rest))))))

;; vector-for-each
(define (vector-for-each f vec . rest)
  (let ((n (vector-length vec)))
    (let loop ((i 0))
      (when (< i n)
        (if (null? rest)
            (f (vector-ref vec i))
            (apply f (vector-ref vec i)
                   (map (lambda (v) (vector-ref v i)) rest)))
        (loop (+ i 1))))))

;; vector-map
(define (vector-map f vec . rest)
  (let* ((n (vector-length vec))
         (result (make-vector n)))
    (let loop ((i 0))
      (when (< i n)
        (vector-set! result i
                     (if (null? rest)
                         (f (vector-ref vec i))
                         (apply f (vector-ref vec i)
                                (map (lambda (v) (vector-ref v i)) rest))))
        (loop (+ i 1))))
    result))

;; vector-append
(define (vector-append . vecs)
  (list->vector (apply append (map vector->list vecs))))

;; vector-copy!
(define (vector-copy! to at from . rest)
  (let* ((start (if (null? rest) 0 (car rest)))
         (end (if (or (null? rest) (null? (cdr rest)))
                  (vector-length from)
                  (cadr rest)))
         (count (- end start)))
    (let loop ((i 0))
      (when (< i count)
        (vector-set! to (+ at i) (vector-ref from (+ start i)))
        (loop (+ i 1))))))

;;; ============================================================
;;; R5RS Appendix: Standard derived expression types
;;; ============================================================

;; do is handled natively in eval.rs

;; let-values
(define-syntax let-values
  (syntax-rules ()
    ((let-values () body ...)
     (begin body ...))
    ((let-values (((var ...) expr) rest ...) body ...)
     (call-with-values
       (lambda () expr)
       (lambda (var ...)
         (let-values (rest ...) body ...))))))

(define-syntax let*-values
  (syntax-rules ()
    ((let*-values () body ...)
     (begin body ...))
    ((let*-values (binding rest ...) body ...)
     (let-values (binding)
       (let*-values (rest ...) body ...)))))

;; receive (SRFI-8)
(define-syntax receive
  (syntax-rules ()
    ((receive formals expression body ...)
     (call-with-values (lambda () expression)
       (lambda formals body ...)))))

;; fluid-let
(define-syntax fluid-let
  (syntax-rules ()
    ((fluid-let ((var val) ...) body ...)
     (let ((old-var var) ...)
       (set! var val) ...
       (let ((result (begin body ...)))
         (set! var old-var) ...
         result)))))

;; when and unless (may override native if needed)
;; (handled natively in eval.rs)

;; dotimes
(define-syntax dotimes
  (syntax-rules ()
    ((dotimes (var n) body ...)
     (let loop ((var 0))
       (when (< var n)
         body ...
         (loop (+ var 1)))))))

;; while
(define-syntax while
  (syntax-rules ()
    ((while test body ...)
     (let loop ()
       (when test
         body ...
         (loop))))))

;; until
(define-syntax until
  (syntax-rules ()
    ((until test body ...)
     (let loop ()
       (unless test
         body ...
         (loop))))))

;; swap!
(define-syntax swap!
  (syntax-rules ()
    ((swap! a b)
     (let ((tmp a))
       (set! a b)
       (set! b tmp)))))

;; push! and pop! for mutable variables
(define-syntax push!
  (syntax-rules ()
    ((push! x lst)
     (set! lst (cons x lst)))))

(define-syntax pop!
  (syntax-rules ()
    ((pop! lst)
     (let ((head (car lst)))
       (set! lst (cdr lst))
       head))))

;; inc! and dec!
(define-syntax inc!
  (syntax-rules ()
    ((inc! x) (set! x (+ x 1)))
    ((inc! x n) (set! x (+ x n)))))

(define-syntax dec!
  (syntax-rules ()
    ((dec! x) (set! x (- x 1)))
    ((dec! x n) (set! x (- x n)))))

;;; ============================================================
;;; Numeric utilities
;;; ============================================================

(define (floor->exact x) (inexact->exact (floor x)))
(define (ceiling->exact x) (inexact->exact (ceiling x)))
(define (round->exact x) (inexact->exact (round x)))
(define (truncate->exact x) (inexact->exact (truncate x)))

;; clamp
(define (clamp x lo hi)
  (max lo (min hi x)))

;; Fibonacci - example of tail recursion
(define (fibonacci n)
  (let loop ((i 0) (a 0) (b 1))
    (if (= i n)
        a
        (loop (+ i 1) b (+ a b)))))

;;; ============================================================
;;; Association / table utilities
;;; ============================================================

(define (make-hash-table) '())

(define (hash-table-set! ht key val)
  (alist-set key val ht))

(define (hash-table-ref ht key . default)
  (apply alist-ref key ht default))

;;; ============================================================
;;; Port / I/O utilities
;;; ============================================================

(define (with-output-to-string thunk)
  ;; Simplified stub: can't truly capture stdout in this impl
  (thunk)
  "")

(define (open-input-string str)
  ;; Returns a simple string port object (stub)
  (list 'string-port str 0))

(define (read-char-from-string port)
  (let ((str (cadr port))
        (pos (caddr port)))
    (if (>= pos (string-length str))
        (eof-object)
        (begin
          (set-car! (cddr port) (+ pos 1))
          (string-ref str pos)))))

;;; ============================================================
;;; Tail-recursive map/for-each variants (for deep lists)
;;; ============================================================

(define (list-map f lst)
  (let loop ((l lst) (acc '()))
    (if (null? l)
        (reverse acc)
        (loop (cdr l) (cons (f (car l)) acc)))))

;;; ============================================================
;;; Char utilities
;;; ============================================================

(define (char->digit char . rest)
  (let ((radix (if (null? rest) 10 (car rest))))
    (let ((n (- (char->integer char) (char->integer #\0))))
      (if (and (>= n 0) (< n radix))
          n
          #f))))

(define (digit->char n . rest)
  (let ((radix (if (null? rest) 10 (car rest))))
    (if (and (>= n 0) (< n radix))
        (integer->char (+ n (char->integer #\0)))
        #f)))

;;; End of standard library

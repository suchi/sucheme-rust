;;; examples/10-macros.scm
;;; sucheme-rust チュートリアル - 第10章: マクロ (syntax-rules)
;;; 実行方法: sucheme-rust examples/10-macros.scm

(display "=== 第10章: マクロ ===") (newline)

;;; --- 1. define-syntax の基本 ---
(display "--- define-syntax の基本 ---") (newline)

;; 単純なマクロ: my-if
(define-syntax my-if
  (syntax-rules ()
    ((my-if test then else)
     (cond (test then)
           (else else)))))

(display (my-if (> 5 3) "5>3" "5<=3"))  ; => 5>3
(newline)

;; swap! マクロ (stdlib にも定義済み)
(define-syntax my-swap!
  (syntax-rules ()
    ((my-swap! a b)
     (let ((tmp a))
       (set! a b)
       (set! b tmp)))))

(define p 10)
(define q 20)
(my-swap! p q)
(display p) (display " ") (display q)  ; => 20 10
(newline)

;;; --- 2. パターンマッチング ---
(display "--- パターンマッチング ---") (newline)

;; 複数パターン
(define-syntax my-and
  (syntax-rules ()
    ((my-and) #t)
    ((my-and x) x)
    ((my-and x y ...) (if x (my-and y ...) #f))))

(display (my-and))              ; => #t
(newline)
(display (my-and 1 2 3))        ; => 3
(newline)
(display (my-and 1 #f 3))       ; => #f
(newline)

(define-syntax my-or
  (syntax-rules ()
    ((my-or) #f)
    ((my-or x) x)
    ((my-or x y ...)
     (let ((t x))
       (if t t (my-or y ...))))))

(display (my-or #f #f 42))      ; => 42
(newline)
(display (my-or #f #f #f))      ; => #f
(newline)

;;; --- 3. リスト操作マクロ ---
(display "--- リスト操作マクロ ---") (newline)

;; push!/pop! (stdlib にも定義済み)
(define-syntax my-push!
  (syntax-rules ()
    ((my-push! x lst)
     (set! lst (cons x lst)))))

(define-syntax my-pop!
  (syntax-rules ()
    ((my-pop! lst)
     (let ((head (car lst)))
       (set! lst (cdr lst))
       head))))

(define stack '())
(my-push! 1 stack)
(my-push! 2 stack)
(my-push! 3 stack)
(display stack)          ; => (3 2 1)
(newline)
(display (my-pop! stack)) ; => 3
(display stack)          ; => (2 1)
(newline)

;;; --- 4. let 変種を作る ---
(display "--- let 変種マクロ ---") (newline)

;; alist-let: 連想リストから値を取り出す
(define-syntax alist-let
  (syntax-rules ()
    ((alist-let alist ((key var) ...) body ...)
     (let ((var (cdr (assq 'key alist))) ...)
       body ...))))

(define person '((name . "Alice") (age . 30) (city . "Tokyo")))
(alist-let person
  ((name n) (age a) (city c))
  (display n) (display " is ")
  (display a) (display " years old from ")
  (display c) (newline))
; => Alice is 30 years old from Tokyo

;;; --- 5. define-record-type (R5RS 拡張) ---
(display "--- define-record-type ---") (newline)

(define-record-type <point>
  (make-point x y)
  point?
  (x point-x set-point-x!)
  (y point-y set-point-y!))

(define p1 (make-point 3 4))
(display (point? p1))         ; => #t
(newline)
(display (point-x p1))        ; => 3
(newline)
(display (point-y p1))        ; => 4
(newline)

(define (distance p)
  (sqrt (+ (square (point-x p))
           (square (point-y p)))))

(display (distance p1))       ; => 5.0
(newline)

;;; --- 6. 繰り返しマクロ ---
(display "--- 繰り返しマクロ ---") (newline)

;; repeat: n 回繰り返し
(define-syntax repeat
  (syntax-rules ()
    ((repeat n body ...)
     (let loop ((i n))
       (when (> i 0)
         body ...
         (loop (- i 1)))))))

(define count 0)
(repeat 5 (set! count (+ count 1)))
(display count)   ; => 5
(newline)

;; dotimes (stdlib にも定義済み)
(dotimes (i 5)
  (display i)
  (display " "))
(newline)  ; => 0 1 2 3 4

(display "=== 第10章 完了 ===") (newline)

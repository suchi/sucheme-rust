;;; examples/02-variables.scm
;;; sucheme-rust チュートリアル - 第2章: 変数と定義
;;; 実行方法: sucheme-rust examples/02-variables.scm

(display "=== 第2章: 変数と定義 ===") (newline)

;;; --- 1. define による変数定義 ---
(display "--- 変数定義 ---") (newline)

(define x 42)
(display x)           ; => 42
(newline)

(define pi 3.14159265358979)
(display pi)          ; => 3.14159265358979
(newline)

(define greeting "Hello, Scheme!")
(display greeting)    ; => Hello, Scheme!
(newline)

;;; --- 2. set! による変数更新 ---
(display "--- 変数更新 (set!) ---") (newline)

(define counter 0)
(display counter)     ; => 0
(newline)
(set! counter (+ counter 1))
(display counter)     ; => 1
(newline)
(set! counter (+ counter 1))
(display counter)     ; => 2
(newline)

;;; --- 3. let による局所変数 ---
(display "--- let (局所変数) ---") (newline)

(let ((a 10)
      (b 20))
  (display (+ a b)))  ; => 30
(newline)

;; let のスコープ外ではアクセスできない
;; (display a)  ; エラー: aは未束縛

;;; --- 4. let* (順次束縛) ---
(display "--- let* (順次束縛) ---") (newline)

(let* ((x 5)
       (y (* x 2))    ; xを参照できる
       (z (+ x y)))
  (display z))        ; => 15 (5 + 10)
(newline)

;;; --- 5. letrec (相互再帰) ---
(display "--- letrec (相互再帰) ---") (newline)

(letrec ((even? (lambda (n)
                  (if (= n 0) #t (odd? (- n 1)))))
         (odd? (lambda (n)
                 (if (= n 0) #f (even? (- n 1))))))
  (display (even? 10)) ; => #t
  (newline)
  (display (odd? 7)))  ; => #t
(newline)

;;; --- 6. begin (順次実行) ---
(display "--- begin ---") (newline)

(define result
  (begin
    (define tmp 100)
    (* tmp 2)))
(display result)       ; => 200
(newline)

(display "=== 第2章 完了 ===") (newline)

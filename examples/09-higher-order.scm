;;; examples/09-higher-order.scm
;;; sucheme-rust チュートリアル - 第9章: 高階関数とプログラミングスタイル
;;; 実行方法: sucheme-rust examples/09-higher-order.scm

(display "=== 第9章: 高階関数 ===") (newline)

;;; --- 1. 関数の組み合わせ ---
(display "--- compose ---") (newline)

(define (compose f g) (lambda (x) (f (g x))))

(define add1 (lambda (x) (+ x 1)))
(define double (lambda (x) (* x 2)))
(define add1-then-double (compose double add1))
(define double-then-add1 (compose add1 double))

(display (add1-then-double 5))  ; => 12 (5+1=6, 6*2=12)
(newline)
(display (double-then-add1 5))  ; => 11 (5*2=10, 10+1=11)
(newline)

;; stdlib の compose (複数引数)
(define transform (compose double add1 (lambda (x) (* x 3))))
(display (transform 2))  ; => 14 (2*3=6, 6+1=7, 7*2=14)
(newline)

;;; --- 2. カリー化 ---
(display "--- カリー化 ---") (newline)

(define (curry f . args)
  (lambda more-args
    (apply f (append args more-args))))

(define add-to-10 (curry + 10))
(display (add-to-10 5))    ; => 15
(newline)
(display (add-to-10 20))   ; => 30
(newline)

(define multiply-by-3 (curry * 3))
(display (multiply-by-3 7))  ; => 21
(newline)

;;; --- 3. 部分適用 ---
(display "--- 部分適用 ---") (newline)

(define (partial f . initial-args)
  (lambda args
    (apply f (append initial-args args))))

(define greater-than-5? (partial < 5))
(display (greater-than-5? 10))   ; => #t
(newline)
(display (greater-than-5? 3))    ; => #f
(newline)

;; filter との組み合わせ
(display (filter (partial < 5) '(1 3 7 2 9 4 8)))
; => (7 9 8)
(newline)

;;; --- 4. apply の活用 ---
(display "--- apply ---") (newline)

(define args '(1 2 3 4 5))
(display (apply + args))         ; => 15
(newline)
(display (apply max args))       ; => 5
(newline)
(display (apply string args))    ; エラー: 数値は文字でない

; 安全に:
(display (apply string (map integer->char '(72 105))))  ; => Hi
(newline)
(display (apply list 'a 'b '(c d e)))  ; => (a b c d e)
(newline)

;;; --- 5. 関数を生成する関数 ---
(display "--- 関数ファクトリ ---") (newline)

;; n乗を計算する関数を生成
(define (make-power n)
  (lambda (x) (expt x n)))

(define square (make-power 2))
(define cube   (make-power 3))
(define fourth (make-power 4))

(for-each
  (lambda (f)
    (display (f 3))
    (display " "))
  (list square cube fourth))
; => 9 27 81
(newline)

;;; --- 6. メモ化 ---
(display "--- メモ化 ---") (newline)

(define (memoize f)
  (let ((cache '()))
    (lambda args
      (let ((cached (assoc args cache)))
        (if cached
            (cdr cached)
            (let ((result (apply f args)))
              (set! cache (cons (cons args result) cache))
              result))))))

;; メモ化したフィボナッチ
(define fib
  (memoize
    (lambda (n)
      (if (< n 2)
          n
          (+ (fib (- n 1)) (fib (- n 2)))))))

(display (fib 30))   ; => 832040
(newline)
(display (fib 35))   ; => 9227465
(newline)

;;; --- 7. Y コンビネーター ---
(display "--- Yコンビネーター ---") (newline)

;; Yコンビネーター: 再帰を自己参照なしで実現
(define Y
  (lambda (f)
    ((lambda (x) (f (lambda (v) ((x x) v))))
     (lambda (x) (f (lambda (v) ((x x) v)))))))

(define fact
  (Y (lambda (self)
       (lambda (n)
         (if (<= n 1) 1 (* n (self (- n 1))))))))

(display (fact 5))   ; => 120
(newline)
(display (fact 10))  ; => 3628800
(newline)

(display "=== 第9章 完了 ===") (newline)

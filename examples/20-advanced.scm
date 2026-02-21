;;; examples/20-advanced.scm
;;; sucheme-rust チュートリアル - 第20章: 高度なトピック
;;; 実行方法: sucheme-rust examples/20-advanced.scm

(display "=== 第20章: 高度なトピック ===") (newline)

;;; --- 1. 関数合成の高度な使い方 ---
(display "--- 関数パイプライン ---") (newline)

;; パイプライン演算子 (~> のような)
(define (pipeline value . fns)
  (fold-left (lambda (acc f) (f acc)) value fns))

(define result
  (pipeline 5
    (lambda (x) (* x 2))      ; 10
    (lambda (x) (+ x 3))      ; 13
    (lambda (x) (* x x))))    ; 169

(display result)  ; => 169
(newline)

;; データ変換パイプライン
(define data '(1 2 3 4 5 6 7 8 9 10))
(define processed
  (pipeline data
    (lambda (lst) (filter odd? lst))       ; (1 3 5 7 9)
    (lambda (lst) (map (lambda (x) (* x x)) lst))  ; (1 9 25 49 81)
    (lambda (lst) (apply + lst))))         ; 165

(display processed)  ; => 165
(newline)

;;; --- 2. モナドパターン ---
(display "--- Maybe モナド ---") (newline)

;; Maybe型: Just x または Nothing
(define (just x) (cons 'just x))
(define (nothing) '(nothing))
(define (just? m) (and (pair? m) (eq? (car m) 'just)))
(define (from-just m) (cdr m))

(define (maybe-bind m f)
  (if (just? m) (f (from-just m)) (nothing)))

(define (maybe-return x) (just x))

;; 安全な数値演算
(define (safe-div a b)
  (if (= b 0) (nothing) (just (/ a b))))

(define (safe-sqrt x)
  (if (< x 0) (nothing) (just (sqrt x))))

(define (safe-log x)
  (if (<= x 0) (nothing) (just (log x))))

;; チェーン: sqrt(log(x/y))
(define (complex-calc x y)
  (maybe-bind (safe-div x y)
    (lambda (d)
      (maybe-bind (safe-log d)
        (lambda (l)
          (safe-sqrt l))))))

(display (complex-calc 100 1))    ; => (just . 2.302...)
(newline)
(display (complex-calc 100 0))    ; => (nothing)
(newline)
(display (complex-calc 1 100))    ; => (nothing) - log(0.01) < 0, sqrt fails
(newline)

;;; --- 3. CPS変換 ---
(display "--- CPS (継続渡しスタイル) ---") (newline)

;; CPS版の基本関数
(define (cps-add a b k) (k (+ a b)))
(define (cps-mul a b k) (k (* a b)))
(define (cps-sub a b k) (k (- a b)))

;; (3 + 4) * (5 - 2) を CPS で計算
(cps-add 3 4
  (lambda (sum)
    (cps-sub 5 2
      (lambda (diff)
        (cps-mul sum diff
          (lambda (result)
            (display "CPS結果: ")
            (display result)
            (newline)))))))
; => 21

;; CPS版フィボナッチ
(define (cps-fib n k)
  (if (< n 2)
      (k n)
      (cps-fib (- n 1)
               (lambda (a)
                 (cps-fib (- n 2)
                          (lambda (b)
                            (k (+ a b))))))))

(cps-fib 10 (lambda (r) (display "fib(10) = ") (display r) (newline)))
; => fib(10) = 55

;;; --- 4. 不動点コンビネーター ---
(display "--- 不動点 ---") (newline)

;; Zコンビネーター (正格評価版)
(define Z
  (lambda (f)
    ((lambda (x) (f (lambda (v) ((x x) v))))
     (lambda (x) (f (lambda (v) ((x x) v)))))))

(define fib
  (Z (lambda (self)
       (lambda (n)
         (if (< n 2) n (+ (self (- n 1)) (self (- n 2))))))))

(display (map fib (iota 10)))  ; => (0 1 1 2 3 5 8 13 21 34)
(newline)

;;; --- 5. 構造的再帰 (catamorphism) ---
(display "--- カタモーフィズム ---") (newline)

;; リストのカタモーフィズム (fold)
(define (list-cata nil cons-fn lst)
  (if (null? lst)
      nil
      (cons-fn (car lst) (list-cata nil cons-fn (cdr lst)))))

;; fold-right として使える
(display (list-cata 0 + '(1 2 3 4 5)))         ; => 15
(newline)
(display (list-cata '() cons '(1 2 3)))          ; => (1 2 3)
(newline)
(display (list-cata 0 (lambda (x acc) (+ 1 acc)) '(a b c d)))  ; => 4 (length)
(newline)

;;; --- 6. トランスデューサー ---
(display "--- トランスデューサー ---") (newline)

;; コンポーザブルなリスト変換
(define (mapping f)
  (lambda (reducer)
    (lambda (acc x)
      (reducer acc (f x)))))

(define (filtering pred)
  (lambda (reducer)
    (lambda (acc x)
      (if (pred x) (reducer acc x) acc))))

(define (transduce xf f init coll)
  (fold-left (xf f) init coll))

;; 合成: 奇数をフィルタして二乗する
(define xform
  (lambda (reducer)
    ((mapping (lambda (x) (* x x)))
     ((filtering odd?)
      reducer))))

(display (transduce xform cons '() '(1 2 3 4 5 6 7)))
; => (1 9 25 49) (order reversed by fold)
(newline)

(display "=== 第20章 完了 ===") (newline)

;;; examples/11-practical-math.scm
;;; sucheme-rust チュートリアル - 第11章: 実用的な数値計算
;;; 実行方法: sucheme-rust examples/11-practical-math.scm

(display "=== 第11章: 実用的な数値計算 ===") (newline)

;;; --- 1. 基本的な数学関数 ---
(display "--- 基本的な数学関数 ---") (newline)

(display (abs -42))           ; => 42
(newline)
(display (max 3 1 4 1 5 9))   ; => 9
(newline)
(display (min 3 1 4 1 5))     ; => 1
(newline)
(display (gcd 12 8))          ; => 4
(newline)
(display (lcm 4 6))           ; => 12
(newline)
(display (expt 2 10))         ; => 1024
(newline)
(display (sqrt 144))          ; => 12
(newline)
(display (floor 3.7))         ; => 3.0
(newline)
(display (ceiling 3.2))       ; => 4.0
(newline)
(display (round 3.5))         ; => 4.0
(newline)
(display (truncate -3.7))     ; => -3.0
(newline)

;;; --- 2. 三角関数 ---
(display "--- 三角関数 ---") (newline)

(define pi (* 4 (atan 1)))
(display "pi = ")
(display pi)
(newline)

(display (sin (/ pi 2)))     ; => 1.0
(newline)
(display (cos 0))            ; => 1.0
(newline)
(display (tan (/ pi 4)))     ; => 1.0 (近似)
(newline)
(display (atan 1 1))         ; => 0.785... (pi/4)
(newline)

;;; --- 3. 対数・指数 ---
(display "--- 対数・指数 ---") (newline)

(display (exp 1))            ; => 2.718... (e)
(newline)
(display (log (exp 1)))      ; => 1.0
(newline)
(display (log 100 10))       ; => 2.0
(newline)
(display (log 8 2))          ; => 3.0
(newline)

;;; --- 4. 整数演算 ---
(display "--- 整数演算 ---") (newline)

(display (quotient 17 5))    ; => 3
(newline)
(display (remainder 17 5))   ; => 2
(newline)
(display (modulo -13 4))     ; => 3 (常に非負)
(newline)
(display (remainder -13 4))  ; => -1 (符号を保持)
(newline)

;;; --- 5. 数値変換 ---
(display "--- 数値変換 ---") (newline)

(display (exact->inexact 1/3))   ; エラーの場合は:
; (display (exact->inexact (/ 1 3)))
(display (exact->inexact 1))     ; => 1.0
(newline)
(display (inexact->exact 3.14))  ; => 3 (近似)
(newline)
(display (exact? 42))            ; => #t
(newline)
(display (inexact? 3.14))        ; => #t
(newline)

;;; --- 6. 実用的な計算 ---
(display "--- 実用的な計算 ---") (newline)

;; 複利計算: 元金 * (1 + 利率)^年数
(define (compound-interest principal rate years)
  (* principal (expt (+ 1 rate) years)))

(display "100万円を年利5%で10年複利運用: ")
(display (round (compound-interest 1000000 0.05 10)))
(newline)  ; => 1628895.0

;; BMI計算
(define (bmi weight-kg height-m)
  (/ weight-kg (* height-m height-m)))

(display "BMI(70kg, 1.70m): ")
(display (exact->inexact (bmi 70 1.70)))
(newline)  ; => 24.22...

;; 温度変換
(define (celsius->fahrenheit c) (+ (* c 9/5) 32))
(define (fahrenheit->celsius f) (* (- f 32) 5/9))

; 整数版
(define (c->f c) (+ (* c 9 1/5) 32))

(for-each
  (lambda (c)
    (display c) (display "°C = ")
    (display (exact->inexact (+ (* c 9/5) 32)))
    (display "°F") (newline))
  '(0 20 37 100))

;;; --- 7. 統計計算 ---
(display "--- 統計計算 ---") (newline)

(define data '(72 85 91 68 79 88 94 76 83 90))

(define (mean lst)
  (/ (apply + lst) (length lst)))

(define (variance lst)
  (let* ((m (mean lst))
         (diffs (map (lambda (x) (square (- x m))) lst)))
    (mean diffs)))

(define (std-dev lst)
  (sqrt (exact->inexact (variance lst))))

(display "データ: ") (display data) (newline)
(display "平均: ") (display (exact->inexact (mean data))) (newline)
(display "分散: ") (display (exact->inexact (variance data))) (newline)
(display "標準偏差: ") (display (std-dev data)) (newline)

(display "=== 第11章 完了 ===") (newline)

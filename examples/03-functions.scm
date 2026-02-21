;;; examples/03-functions.scm
;;; sucheme-rust チュートリアル - 第3章: 関数と手続き
;;; 実行方法: sucheme-rust examples/03-functions.scm

(display "=== 第3章: 関数と手続き ===") (newline)

;;; --- 1. lambda による無名関数 ---
(display "--- lambda ---") (newline)

(define square (lambda (x) (* x x)))
(display (square 5))   ; => 25
(newline)

;; すぐ呼び出す
(display ((lambda (x y) (+ x y)) 3 4))  ; => 7
(newline)

;;; --- 2. define の短縮形 ---
(display "--- define の短縮形 ---") (newline)

;; (define (name params) body) は
;; (define name (lambda (params) body)) と同じ
(define (cube x) (* x x x))
(display (cube 3))     ; => 27
(newline)

(define (add a b) (+ a b))
(display (add 10 20))  ; => 30
(newline)

;;; --- 3. 可変長引数 ---
(display "--- 可変長引数 ---") (newline)

(define (sum . args)
  (apply + args))
(display (sum 1 2 3 4 5))  ; => 15
(newline)

(define (first-and-rest first . rest)
  (list first rest))
(display (first-and-rest 1 2 3 4))  ; => (1 (2 3 4))
(newline)

;;; --- 4. 高階関数 ---
(display "--- 高階関数 ---") (newline)

;; 関数を引数に取る
(define (apply-twice f x) (f (f x)))
(display (apply-twice square 3))  ; => 81 (3^2=9, 9^2=81)
(newline)

;; 関数を返す
(define (make-adder n)
  (lambda (x) (+ x n)))

(define add5 (make-adder 5))
(define add10 (make-adder 10))
(display (add5 3))     ; => 8
(newline)
(display (add10 3))    ; => 13
(newline)

;;; --- 5. クロージャ ---
(display "--- クロージャ ---") (newline)

(define (make-counter)
  (let ((count 0))
    (lambda ()
      (set! count (+ count 1))
      count)))

(define counter1 (make-counter))
(define counter2 (make-counter))

(display (counter1))   ; => 1
(newline)
(display (counter1))   ; => 2
(newline)
(display (counter1))   ; => 3
(newline)
(display (counter2))   ; => 1 (独立したカウンター)
(newline)

;;; --- 6. 再帰関数 ---
(display "--- 再帰関数 ---") (newline)

(define (factorial n)
  (if (<= n 1)
      1
      (* n (factorial (- n 1)))))

(display (factorial 5))   ; => 120
(newline)
(display (factorial 10))  ; => 3628800
(newline)

;;; --- 7. 末尾再帰 (TCO) ---
(display "--- 末尾再帰 ---") (newline)

(define (factorial-iter n acc)
  (if (<= n 1)
      acc
      (factorial-iter (- n 1) (* n acc))))

(define (fact n) (factorial-iter n 1))
(display (fact 10))    ; => 3628800
(newline)

;; 大きな数でも安全
(display (fact 20))    ; => 2432902008176640000
(newline)

;;; --- 8. 相互再帰 ---
(display "--- 相互再帰 ---") (newline)

(define (my-even? n)
  (if (= n 0) #t (my-odd? (- n 1))))

(define (my-odd? n)
  (if (= n 0) #f (my-even? (- n 1))))

(display (my-even? 100))  ; => #t
(newline)
(display (my-odd? 99))    ; => #t
(newline)

(display "=== 第3章 完了 ===") (newline)

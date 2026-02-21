;;; examples/08-tail-recursion.scm
;;; sucheme-rust チュートリアル - 第8章: 末尾再帰とループ
;;; 実行方法: sucheme-rust examples/08-tail-recursion.scm

(display "=== 第8章: 末尾再帰とループ ===") (newline)

;;; --- 1. 通常の再帰 vs 末尾再帰 ---
(display "--- 通常の再帰 ---") (newline)

;; 通常の再帰: スタックを消費する
(define (sum-normal n)
  (if (<= n 0)
      0
      (+ n (sum-normal (- n 1)))))

(display (sum-normal 100))    ; => 5050
(newline)

;; 末尾再帰: スタックを消費しない (TCO)
(define (sum-tail n acc)
  (if (<= n 0)
      acc
      (sum-tail (- n 1) (+ acc n))))

(define (sum n) (sum-tail n 0))
(display (sum 100))           ; => 5050
(newline)

;; 大きな数でも安全
(display (sum 1000000))       ; => 500000500000
(newline)

;;; --- 2. do ループ ---
(display "--- do ループ ---") (newline)

;; (do ((var init step) ...) (test result) body)
(define (sum-do n)
  (do ((i 1 (+ i 1))
       (acc 0 (+ acc i)))
      ((> i n) acc)))

(display (sum-do 100))        ; => 5050
(newline)

;; リストの各要素を表示
(do ((lst '(1 2 3 4 5) (cdr lst)))
    ((null? lst))
  (display (car lst))
  (display " "))
(newline)

;;; --- 3. 名前付き let (named let) ---
(display "--- 名前付き let ---") (newline)

;; ループに名前をつけて再帰する
(define (sum-named n)
  (let loop ((i n) (acc 0))
    (if (<= i 0)
        acc
        (loop (- i 1) (+ acc i)))))

(display (sum-named 100))     ; => 5050
(newline)

;; フィボナッチ数列 (名前付き let)
(define (fib n)
  (let loop ((i 0) (a 0) (b 1))
    (if (= i n)
        a
        (loop (+ i 1) b (+ a b)))))

(for-each
  (lambda (n)
    (display (fib n))
    (display " "))
  (iota 10))
(newline)
; => 0 1 1 2 3 5 8 13 21 34

;;; --- 4. while / until マクロ ---
(display "--- while / until ---") (newline)

(define x 1)
(while (< x 32)
  (set! x (* x 2)))
(display x)    ; => 32
(newline)

(define y 100)
(until (< y 10)
  (set! y (quotient y 2)))
(display y)    ; => 6
(newline)

;;; --- 5. 高度な末尾再帰パターン ---
(display "--- 高度なパターン ---") (newline)

;; リストのリバース (末尾再帰)
(define (my-reverse lst)
  (let loop ((remaining lst) (acc '()))
    (if (null? remaining)
        acc
        (loop (cdr remaining) (cons (car remaining) acc)))))

(display (my-reverse '(1 2 3 4 5)))  ; => (5 4 3 2 1)
(newline)

;; フラット化 (末尾再帰)
(define (my-flatten lst)
  (let loop ((l lst) (acc '()))
    (cond
      ((null? l) (reverse acc))
      ((pair? (car l)) (loop (append (car l) (cdr l)) acc))
      (else (loop (cdr l) (cons (car l) acc))))))

(display (my-flatten '(1 (2 3) (4 (5 6)) 7)))  ; => (1 2 3 4 5 6 7)
(newline)

;; 素数列生成 (エラトステネスのふるい)
(define (sieve limit)
  (let ((composites (make-vector (+ limit 1) #f)))
    (let loop ((i 2) (primes '()))
      (if (> i limit)
          (reverse primes)
          (if (vector-ref composites i)
              (loop (+ i 1) primes)
              (begin
                (let mark ((j (* i i)))
                  (when (<= j limit)
                    (vector-set! composites j #t)
                    (mark (+ j i))))
                (loop (+ i 1) (cons i primes))))))))

(display (sieve 50))
; => (2 3 5 7 11 13 17 19 23 29 31 37 41 43 47)
(newline)

(display "=== 第8章 完了 ===") (newline)

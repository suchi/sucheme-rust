;;; examples/18-lazy-evaluation.scm
;;; cprustscheme チュートリアル - 第18章: 遅延評価とストリーム
;;; 実行方法: cprustscheme examples/18-lazy-evaluation.scm

(display "=== 第18章: 遅延評価とストリーム ===") (newline)

;;; --- 1. delay と force ---
(display "--- delay と force ---") (newline)

;; delay: 評価を遅らせる (promise を作る)
;; force: promise を評価する

(define p (delay (begin (display "評価中!\n") (* 6 7))))
(display "promise 作成完了\n")
(display (force p))   ; => 評価中! 42
(newline)
(display (force p))   ; => 42 (2回目はキャッシュ)
(newline)

;;; --- 2. ストリーム ---
(display "--- ストリーム ---") (newline)

;; ストリーム: cons-stream でペアを作り、cdr は遅延評価
(define-syntax cons-stream
  (syntax-rules ()
    ((cons-stream a b) (cons a (delay b)))))

(define stream-car car)
(define (stream-cdr s) (force (cdr s)))
(define stream-null? null?)
(define the-empty-stream '())

(define (stream-take n s)
  (if (or (= n 0) (stream-null? s))
      '()
      (cons (stream-car s)
            (stream-take (- n 1) (stream-cdr s)))))

(define (stream-map f s)
  (if (stream-null? s)
      the-empty-stream
      (cons-stream (f (stream-car s))
                   (stream-map f (stream-cdr s)))))

(define (stream-filter pred s)
  (cond
    ((stream-null? s) the-empty-stream)
    ((pred (stream-car s))
     (cons-stream (stream-car s)
                  (stream-filter pred (stream-cdr s))))
    (else (stream-filter pred (stream-cdr s)))))

;; 無限ストリーム: 自然数
(define (integers-from n)
  (cons-stream n (integers-from (+ n 1))))

(define naturals (integers-from 1))

(display "最初の10自然数: ")
(display (stream-take 10 naturals))
(newline)

;; 偶数ストリーム
(define evens (stream-filter even? naturals))
(display "最初の10偶数: ")
(display (stream-take 10 evens))
(newline)

;; 二乗ストリーム
(define squares (stream-map (lambda (x) (* x x)) naturals))
(display "最初の10二乗: ")
(display (stream-take 10 squares))
(newline)

;;; --- 3. 無限フィボナッチ数列 ---
(display "--- 無限フィボナッチ ---") (newline)

(define (fib-stream a b)
  (cons-stream a (fib-stream b (+ a b))))

(define fibs (fib-stream 0 1))
(display "最初の15フィボナッチ: ")
(display (stream-take 15 fibs))
(newline)

;;; --- 4. エラトステネスのふるい (ストリーム版) ---
(display "--- 素数ストリーム ---") (newline)

(define (sieve s)
  (let ((p (stream-car s)))
    (cons-stream
      p
      (sieve (stream-filter
               (lambda (x) (not (= (modulo x p) 0)))
               (stream-cdr s))))))

(define primes (sieve (integers-from 2)))
(display "最初の15素数: ")
(display (stream-take 15 primes))
(newline)

;;; --- 5. 遅延評価の活用 ---
(display "--- 遅延評価の活用 ---") (newline)

;; if の引数も遅延評価 (Schemeはすでに正格だが delay で明示)
(define (safe-head lst default)
  (if (null? lst)
      (force default)
      (car lst)))

(display (safe-head '(1 2 3) (delay (error "空リストです"))))
(newline)  ; => 1

;; 無限リストの先頭だけ取る
(define (stream-nth n s)
  (if (= n 0)
      (stream-car s)
      (stream-nth (- n 1) (stream-cdr s))))

(display "100番目の素数: ")
(display (stream-nth 99 primes))
(newline)

(display "=== 第18章 完了 ===") (newline)

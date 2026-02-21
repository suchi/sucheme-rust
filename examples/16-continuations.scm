;;; examples/16-continuations.scm
;;; sucheme-rust チュートリアル - 第16章: 継続と call/cc
;;; 実行方法: sucheme-rust examples/16-continuations.scm

(display "=== 第16章: 継続と call/cc ===") (newline)

;;; --- 1. call/cc の基本 ---
(display "--- call/cc の基本 ---") (newline)

;; call/cc は現在の継続をキャプチャする
(define result
  (+ 1 (call/cc
         (lambda (k)
           (+ 2 (k 10))))))  ; k は "+ 1 ..." の継続
; k(10) => 10が "+ 1 ..."に渡される => 11
(display result)  ; => 11
(newline)

;; 単純な使用例: 早期脱出
(define (find-first pred lst)
  (call/cc
    (lambda (return)
      (for-each (lambda (x)
                  (when (pred x) (return x)))
                lst)
      #f)))

(display (find-first even? '(1 3 5 4 7)))  ; => 4
(newline)
(display (find-first even? '(1 3 5 7)))    ; => #f
(newline)

;;; --- 2. 脱出継続 (エスケープ) ---
(display "--- 脱出継続 ---") (newline)

;; ネストしたループからの脱出
(define (search-matrix matrix target)
  (call/cc
    (lambda (exit)
      (for-each
        (lambda (row)
          (for-each
            (lambda (elem)
              (when (= elem target) (exit #t)))
            row))
        matrix)
      #f)))

(define matrix '((1 2 3) (4 5 6) (7 8 9)))
(display (search-matrix matrix 5))  ; => #t
(newline)
(display (search-matrix matrix 10)) ; => #f
(newline)

;;; --- 3. 継続でリストの積 (0があったら早期脱出) ---
(display "--- 積の最適化 ---") (newline)

(define (product lst)
  (call/cc
    (lambda (exit)
      (let loop ((l lst) (acc 1))
        (cond
          ((null? l) acc)
          ((= (car l) 0) (exit 0))  ; 0があれば即座に0
          (else (loop (cdr l) (* acc (car l)))))))))

(display (product '(1 2 3 4 5)))     ; => 120
(newline)
(display (product '(1 2 0 4 5)))     ; => 0 (早期脱出)
(newline)

;;; --- 4. コルーチン (継続による) ---
(display "--- コルーチン ---") (newline)

;; クロージャベースのシンプルなジェネレーター
(define (make-counter-gen start step limit)
  (let ((current start))
    (lambda ()
      (if (>= current limit)
          'done
          (let ((val current))
            (set! current (+ current step))
            val)))))

(define gen (make-counter-gen 0 2 10))
(let loop ((v (gen)))
  (unless (eq? v 'done)
    (display v) (display " ")
    (loop (gen))))
(newline)  ; => 0 2 4 6 8

;;; --- 5. 例外処理 (guard マクロ) ---
(display "--- 例外処理 (guard) ---") (newline)

(define (safe-divide a b)
  (guard (exn
          ((string? (condition/report-string exn))
           (display "エラー: ")
           (display (condition/report-string exn))
           (newline)
           #f))
    (if (= b 0)
        (error "Division by zero" a b)
        (/ a b))))

; guard が定義されていない場合の代替
(define (safe-div a b)
  (if (= b 0)
      (begin (display "Error: division by zero") (newline) #f)
      (/ a b)))

(display (safe-div 10 2))   ; => 5
(newline)
(display (safe-div 10 0))   ; => エラー表示して #f
(newline)

(display "=== 第16章 完了 ===") (newline)

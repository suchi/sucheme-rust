;;; examples/07-vectors.scm
;;; sucheme-rust チュートリアル - 第7章: ベクター
;;; 実行方法: sucheme-rust examples/07-vectors.scm

(display "=== 第7章: ベクター ===") (newline)

;;; --- 1. ベクターの作成 ---
(display "--- ベクターの作成 ---") (newline)

(define v1 #(1 2 3 4 5))
(display v1)               ; => #(1 2 3 4 5)
(newline)

(define v2 (vector 'a 'b 'c))
(display v2)               ; => #(a b c)
(newline)

(define v3 (make-vector 5 0))
(display v3)               ; => #(0 0 0 0 0)
(newline)

(define v4 (make-vector 3 "hello"))
(display v4)               ; => #(hello hello hello)
(newline)

;;; --- 2. ベクターのアクセス ---
(display "--- ベクターのアクセス ---") (newline)

(display (vector-length v1))     ; => 5
(newline)
(display (vector-ref v1 0))      ; => 1
(newline)
(display (vector-ref v1 4))      ; => 5
(newline)

;;; --- 3. ベクターの更新 ---
(display "--- ベクターの更新 ---") (newline)

(define mv (vector 1 2 3 4 5))
(vector-set! mv 2 99)
(display mv)               ; => #(1 2 99 4 5)
(newline)

;;; --- 4. ベクターとリストの変換 ---
(display "--- 変換 ---") (newline)

(display (vector->list #(1 2 3)))    ; => (1 2 3)
(newline)
(display (list->vector '(a b c)))    ; => #(a b c)
(newline)

;;; --- 5. ベクターのイテレーション ---
(display "--- イテレーション ---") (newline)

(define scores #(85 92 78 95 88))

;; vector-for-each
(display "スコア: ")
(vector-for-each
  (lambda (s) (display s) (display " "))
  scores)
(newline)

;; vector-map
(define grades
  (vector-map
    (lambda (s)
      (cond ((>= s 90) 'A)
            ((>= s 80) 'B)
            ((>= s 70) 'C)
            (else 'D)))
    scores))
(display grades)           ; => #(B A C A B)
(newline)

;;; --- 6. ベクターの実用例 ---
(display "--- 実用例 ---") (newline)

;; 配列の合計
(define (vector-sum v)
  (let ((total 0))
    (vector-for-each
      (lambda (x) (set! total (+ total x)))
      v)
    total))

(display (vector-sum scores))  ; => 438
(newline)

;; 配列の最大値
(define (vector-max v)
  (let ((m (vector-ref v 0)))
    (vector-for-each
      (lambda (x) (when (> x m) (set! m x)))
      v)
    m))

(display (vector-max scores))  ; => 95
(newline)

;; バブルソート (破壊的)
(define (bubble-sort! v)
  (let ((n (vector-length v)))
    (let outer ((i 0))
      (when (< i (- n 1))
        (let inner ((j 0))
          (when (< j (- n i 1))
            (when (> (vector-ref v j) (vector-ref v (+ j 1)))
              (let ((tmp (vector-ref v j)))
                (vector-set! v j (vector-ref v (+ j 1)))
                (vector-set! v (+ j 1) tmp)))
            (inner (+ j 1))))
        (outer (+ i 1))))))
  v)

(define arr (vector 5 3 8 1 9 2 7 4 6))
(bubble-sort! arr)
(display arr)   ; => #(1 2 3 4 5 6 7 8 9)
(newline)

(display "=== 第7章 完了 ===") (newline)

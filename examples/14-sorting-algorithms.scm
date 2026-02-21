;;; examples/14-sorting-algorithms.scm
;;; sucheme-rust チュートリアル - 第14章: ソートアルゴリズム
;;; 実行方法: sucheme-rust examples/14-sorting-algorithms.scm

(display "=== 第14章: ソートアルゴリズム ===") (newline)

(define test-list '(38 27 43 3 9 82 10 17))
(display "元のリスト: ") (display test-list) (newline)

;;; --- 1. 挿入ソート ---
(display "--- 挿入ソート ---") (newline)

(define (insert-sorted x sorted)
  (cond
    ((null? sorted) (list x))
    ((<= x (car sorted)) (cons x sorted))
    (else (cons (car sorted) (insert-sorted x (cdr sorted))))))

(define (insertion-sort lst)
  (fold-left (lambda (sorted x) (insert-sorted x sorted))
             '() lst))

(display (insertion-sort test-list))
(newline)

;;; --- 2. 選択ソート ---
(display "--- 選択ソート ---") (newline)

(define (remove-first x lst)
  (cond
    ((null? lst) '())
    ((equal? x (car lst)) (cdr lst))
    (else (cons (car lst) (remove-first x (cdr lst))))))

(define (selection-sort lst)
  (if (null? lst)
      '()
      (let ((m (apply min lst)))
        (cons m (selection-sort (remove-first m lst))))))

(display (selection-sort test-list))
(newline)

;;; --- 3. マージソート ---
(display "--- マージソート ---") (newline)

(define (merge lst1 lst2)
  (cond
    ((null? lst1) lst2)
    ((null? lst2) lst1)
    ((<= (car lst1) (car lst2))
     (cons (car lst1) (merge (cdr lst1) lst2)))
    (else
     (cons (car lst2) (merge lst1 (cdr lst2))))))

(define (split lst)
  (if (or (null? lst) (null? (cdr lst)))
      (values lst '())
      (let-values (((a b) (split (cddr lst))))
        (values (cons (car lst) a) (cons (cadr lst) b)))))

(define (merge-sort lst)
  (if (or (null? lst) (null? (cdr lst)))
      lst
      (let-values (((a b) (split lst)))
        (merge (merge-sort a) (merge-sort b)))))

(display (merge-sort test-list))
(newline)

;;; --- 4. クイックソート ---
(display "--- クイックソート ---") (newline)

(define (quicksort lst)
  (if (or (null? lst) (null? (cdr lst)))
      lst
      (let ((pivot (car lst))
            (rest (cdr lst)))
        (let ((smaller (filter (lambda (x) (< x pivot)) rest))
              (equal-  (filter (lambda (x) (= x pivot)) rest))
              (larger  (filter (lambda (x) (> x pivot)) rest)))
          (append (quicksort smaller)
                  (list pivot)
                  equal-
                  (quicksort larger))))))

(display (quicksort test-list))
(newline)

;;; --- 5. ヒープソート ---
(display "--- ヒープソート ---") (newline)

(define (heapify! v n i)
  (let* ((left  (+ (* 2 i) 1))
         (right (+ (* 2 i) 2))
         (largest (if (and (< left n)
                           (> (vector-ref v left) (vector-ref v i)))
                      left i)))
    (let ((largest (if (and (< right n)
                            (> (vector-ref v right) (vector-ref v largest)))
                       right largest)))
      (when (not (= largest i))
        (let ((tmp (vector-ref v i)))
          (vector-set! v i (vector-ref v largest))
          (vector-set! v largest tmp))
        (heapify! v n largest)))))

(define (heap-sort! v)
  (let ((n (vector-length v)))
    ;; ヒープ構築
    (let loop ((i (- (quotient n 2) 1)))
      (when (>= i 0)
        (heapify! v n i)
        (loop (- i 1))))
    ;; ソート
    (let loop ((i (- n 1)))
      (when (> i 0)
        (let ((tmp (vector-ref v 0)))
          (vector-set! v 0 (vector-ref v i))
          (vector-set! v i tmp))
        (heapify! v i 0)
        (loop (- i 1))))
    v))

(define heap-v (list->vector test-list))
(display (vector->list (heap-sort! heap-v)))
(newline)

;;; --- 6. stdlib の sort ---
(display "--- stdlib の sort ---") (newline)

(display (sort < test-list))
(newline)
(display (sort > test-list))  ; 降順
(newline)
(display (sort string<? '("banana" "apple" "cherry" "date")))
(newline)

;;; --- 7. パフォーマンス比較 ---
(display "--- 比較 ---") (newline)

(define big-list (map (lambda (x) (- 1000 x)) (iota 100)))
(display "100要素ソート: ")
(display (equal? (sort < big-list)
                 (insertion-sort big-list)))
(newline)

(display "=== 第14章 完了 ===") (newline)

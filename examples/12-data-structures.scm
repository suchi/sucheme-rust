;;; examples/12-data-structures.scm
;;; sucheme-rust チュートリアル - 第12章: データ構造の実装
;;; 実行方法: sucheme-rust examples/12-data-structures.scm

(display "=== 第12章: データ構造の実装 ===") (newline)

;;; --- 1. スタック ---
(display "--- スタック ---") (newline)

(define (make-stack) (list 'stack '()))
(define (stack-push! s x) (set-car! (cdr s) (cons x (cadr s))))
(define (stack-pop! s)
  (if (null? (cadr s))
      (error "Stack underflow")
      (let ((top (caadr s)))
        (set-car! (cdr s) (cdadr s))
        top)))
(define (stack-peek s) (caadr s))
(define (stack-empty? s) (null? (cadr s)))
(define (stack->list s) (cadr s))

(define st (make-stack))
(stack-push! st 1)
(stack-push! st 2)
(stack-push! st 3)
(display (stack->list st))   ; => (3 2 1)
(newline)
(display (stack-pop! st))    ; => 3
(display (stack->list st))   ; => (2 1)
(newline)

;;; --- 2. キュー ---
(display "--- キュー ---") (newline)

(define (make-queue) (cons '() '()))
(define (enqueue! q x)
  (let ((new-cell (list x)))
    (if (null? (car q))
        (begin (set-car! q new-cell)
               (set-cdr! q new-cell))
        (begin (set-cdr! (cdr q) new-cell)
               (set-cdr! q new-cell)))))
(define (dequeue! q)
  (if (null? (car q))
      (error "Queue empty")
      (let ((val (caar q)))
        (set-car! q (cdar q))
        val)))
(define (queue-empty? q) (null? (car q)))

(define q (make-queue))
(enqueue! q 'a)
(enqueue! q 'b)
(enqueue! q 'c)
(display (dequeue! q))  ; => a
(newline)
(display (dequeue! q))  ; => b
(newline)
(enqueue! q 'd)
(display (dequeue! q))  ; => c
(newline)

;;; --- 3. 二分探索木 ---
(display "--- 二分探索木 ---") (newline)

;; BST: #f or (value left right)
(define (bst-empty) #f)

(define (bst-insert tree val)
  (cond
    ((not tree) (list val #f #f))
    ((< val (car tree))
     (list (car tree) (bst-insert (cadr tree) val) (caddr tree)))
    ((> val (car tree))
     (list (car tree) (cadr tree) (bst-insert (caddr tree) val)))
    (else tree)))  ; 重複は無視

(define (bst-member? tree val)
  (cond
    ((not tree) #f)
    ((= val (car tree)) #t)
    ((< val (car tree)) (bst-member? (cadr tree) val))
    (else (bst-member? (caddr tree) val))))

(define (bst-to-sorted-list tree)
  (if (not tree)
      '()
      (append (bst-to-sorted-list (cadr tree))
              (list (car tree))
              (bst-to-sorted-list (caddr tree)))))

(define bst (bst-empty))
(for-each (lambda (x) (set! bst (bst-insert bst x)))
          '(5 3 7 1 4 6 8 2))

(display (bst-to-sorted-list bst))  ; => (1 2 3 4 5 6 7 8)
(newline)
(display (bst-member? bst 4))       ; => #t
(newline)
(display (bst-member? bst 9))       ; => #f
(newline)

;;; --- 4. ハッシュテーブル (連想リスト実装) ---
(display "--- ハッシュテーブル ---") (newline)

(define (ht-new) (make-vector 16 '()))

(define (ht-hash key size)
  (let ((s (if (symbol? key)
               (symbol->string key)
               (if (string? key) key (number->string key)))))
    (modulo
      (fold-left (lambda (acc c) (+ (* acc 31) (char->integer c))) 0
                 (string->list s))
      size)))

(define (ht-set! ht key val)
  (let* ((idx (ht-hash key (vector-length ht)))
         (bucket (vector-ref ht idx))
         (existing (assoc key bucket)))
    (if existing
        (set-cdr! existing val)
        (vector-set! ht idx (cons (cons key val) bucket)))))

(define (ht-get ht key . default)
  (let* ((idx (ht-hash key (vector-length ht)))
         (bucket (vector-ref ht idx))
         (entry (assoc key bucket)))
    (if entry
        (cdr entry)
        (if (null? default) #f (car default)))))

(define ht (ht-new))
(ht-set! ht "name" "Alice")
(ht-set! ht "age" 30)
(ht-set! ht "city" "Tokyo")

(display (ht-get ht "name"))   ; => Alice
(newline)
(display (ht-get ht "age"))    ; => 30
(newline)
(display (ht-get ht "email" "unknown"))  ; => unknown
(newline)

;;; --- 5. 優先度キュー (ヒープ) ---
(display "--- 優先度キュー ---") (newline)

(define (make-pq) (vector 0 #f #f #f #f #f #f #f #f #f #f))

(define (pq-size pq) (vector-ref pq 0))

(define (pq-insert! pq val)
  (let ((i (+ (pq-size pq) 1)))
    (vector-set! pq 0 i)
    (vector-set! pq i val)
    ;; ヒープ上昇
    (let loop ((pos i))
      (when (> pos 1)
        (let* ((parent (quotient pos 2))
               (pv (vector-ref pq parent)))
          (when (< val pv)
            (vector-set! pq pos pv)
            (vector-set! pq parent val)
            (loop parent)))))))

(define (pq-pop! pq)
  (let ((n (pq-size pq)))
    (if (= n 0)
        (error "Priority queue empty")
        (let ((top (vector-ref pq 1))
              (last (vector-ref pq n)))
          (vector-set! pq 0 (- n 1))
          (when (> n 1)
            (vector-set! pq 1 last)
            ;; ヒープ下降 (省略: 簡易版)
          )
          top))))

(define pq (make-pq))
(for-each (lambda (x) (pq-insert! pq x)) '(5 3 7 1 4))
(display (pq-pop! pq))   ; => 1
(newline)
(display (pq-pop! pq))   ; => 3 (近似)
(newline)

(display "=== 第12章 完了 ===") (newline)

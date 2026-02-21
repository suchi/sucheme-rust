;;; examples/05-lists.scm
;;; cprustscheme チュートリアル - 第5章: リストとペア
;;; 実行方法: cprustscheme examples/05-lists.scm

(display "=== 第5章: リストとペア ===") (newline)

;;; --- 1. ペアと cons ---
(display "--- ペアと cons ---") (newline)

(display (cons 1 2))         ; => (1 . 2)
(newline)
(display (cons 1 '()))       ; => (1)
(newline)
(display (cons 1 (cons 2 (cons 3 '()))))  ; => (1 2 3)
(newline)

(define pair (cons 'a 'b))
(display (car pair))         ; => a (先頭要素)
(newline)
(display (cdr pair))         ; => b (残り)
(newline)

;;; --- 2. リストの作成 ---
(display "--- リストの作成 ---") (newline)

(define lst '(1 2 3 4 5))
(display lst)                ; => (1 2 3 4 5)
(newline)

(define lst2 (list 'a 'b 'c))
(display lst2)               ; => (a b c)
(newline)

;; 空リスト
(display '())                ; => ()
(newline)
(display (null? '()))        ; => #t
(newline)
(display (null? lst))        ; => #f
(newline)

;;; --- 3. リスト操作 ---
(display "--- リスト操作 ---") (newline)

(display (car '(1 2 3)))     ; => 1
(newline)
(display (cdr '(1 2 3)))     ; => (2 3)
(newline)
(display (cadr '(1 2 3)))    ; => 2 (cdr の car)
(newline)
(display (caddr '(1 2 3)))   ; => 3
(newline)
(display (length '(a b c d)))  ; => 4
(newline)
(display (reverse '(1 2 3)))   ; => (3 2 1)
(newline)
(display (append '(1 2) '(3 4) '(5)))  ; => (1 2 3 4 5)
(newline)
(display (list-ref '(a b c d) 2))  ; => c (0始まり)
(newline)

;;; --- 4. リスト検索 ---
(display "--- リスト検索 ---") (newline)

(display (member 3 '(1 2 3 4 5)))  ; => (3 4 5)
(newline)
(display (member 9 '(1 2 3)))      ; => #f
(newline)
(display (assoc 'b '((a 1) (b 2) (c 3))))  ; => (b 2)
(newline)
(display (assv 2 '((1 a) (2 b) (3 c))))    ; => (2 b)
(newline)

;;; --- 5. map と filter ---
(display "--- map と filter ---") (newline)

(display (map (lambda (x) (* x x)) '(1 2 3 4 5)))
; => (1 4 9 16 25)
(newline)

(display (filter odd? '(1 2 3 4 5 6 7 8)))
; => (1 3 5 7)
(newline)

(display (map + '(1 2 3) '(10 20 30)))
; => (11 22 33)
(newline)

;;; --- 6. for-each ---
(display "--- for-each ---") (newline)

(for-each
  (lambda (x)
    (display x)
    (display " "))
  '(apple banana cherry))
(newline)

;;; --- 7. リスト畳み込み ---
(display "--- fold (畳み込み) ---") (newline)

;; fold-right (右畳み込み)
(display (fold-right + 0 '(1 2 3 4 5)))  ; => 15
(newline)
(display (fold-right cons '() '(1 2 3)))  ; => (1 2 3)
(newline)

;; fold-left (左畳み込み)
(display (fold-left + 0 '(1 2 3 4 5)))   ; => 15
(newline)

;;; --- 8. リスト生成 ---
(display "--- リスト生成 ---") (newline)

;; iota: n個の連番リスト
(display (iota 5))          ; => (0 1 2 3 4)
(newline)
(display (iota 5 1))        ; => (1 2 3 4 5)
(newline)
(display (iota 5 0 2))      ; => (0 2 4 6 8)
(newline)

;;; --- 9. 連想リスト (alist) ---
(display "--- 連想リスト ---") (newline)

(define phone-book
  '((alice . "090-1111-2222")
    (bob   . "090-3333-4444")
    (carol . "090-5555-6666")))

(define (lookup name)
  (let ((entry (assq name phone-book)))
    (if entry
        (cdr entry)
        "見つかりません")))

(display (lookup 'bob))    ; => 090-3333-4444
(newline)
(display (lookup 'dave))   ; => 見つかりません
(newline)

(display "=== 第5章 完了 ===") (newline)

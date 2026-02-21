;;; examples/04-conditionals.scm
;;; cprustscheme チュートリアル - 第4章: 条件分岐
;;; 実行方法: cprustscheme examples/04-conditionals.scm

(display "=== 第4章: 条件分岐 ===") (newline)

;;; --- 1. if ---
(display "--- if ---") (newline)

(display (if #t "真" "偽"))     ; => 真
(newline)
(display (if #f "真" "偽"))     ; => 偽
(newline)
(display (if (> 5 3) "5>3" "5<=3"))  ; => 5>3
(newline)

;; else なしの if
(if #t (display "条件が真"))    ; => 条件が真
(newline)
(if #f (display "表示されない")) ; => 何も表示されない

;;; --- 2. cond (多分岐) ---
(display "--- cond ---") (newline)

(define (grade score)
  (cond
    ((>= score 90) "A")
    ((>= score 80) "B")
    ((>= score 70) "C")
    ((>= score 60) "D")
    (else "F")))

(display (grade 95))   ; => A
(newline)
(display (grade 82))   ; => B
(newline)
(display (grade 55))   ; => F
(newline)

;; cond の => 構文
(display
  (cond ((assv 2 '((1 . a) (2 . b) (3 . c))) => cdr)
        (else 'not-found)))  ; => b
(newline)

;;; --- 3. case (値による分岐) ---
(display "--- case ---") (newline)

(define (day-name n)
  (case n
    ((1) "月曜日")
    ((2) "火曜日")
    ((3) "水曜日")
    ((4) "木曜日")
    ((5) "金曜日")
    ((6) "土曜日")
    ((7) "日曜日")
    (else "不明")))

(display (day-name 3))   ; => 水曜日
(newline)
(display (day-name 7))   ; => 日曜日
(newline)
(display (day-name 8))   ; => 不明
(newline)

;;; --- 4. when / unless ---
(display "--- when / unless ---") (newline)

(when (> 5 3)
  (display "5は3より大きい")
  (newline))

(unless (> 3 5)
  (display "3は5より大きくない")
  (newline))

;;; --- 5. and / or の値 ---
(display "--- and / or の値 ---") (newline)

;; and は最後の真の値、または最初の偽の値を返す
(display (and 1 2 3))    ; => 3
(newline)
(display (and 1 #f 3))   ; => #f
(newline)
(display (and))          ; => #t (引数なし)
(newline)

;; or は最初の真の値、またはすべて偽なら #f
(display (or #f #f 42))  ; => 42
(newline)
(display (or #f #f #f))  ; => #f
(newline)
(display (or))           ; => #f (引数なし)
(newline)

;;; --- 6. 型テスト ---
(display "--- 型テスト ---") (newline)

(for-each
  (lambda (x)
    (display x)
    (display " -> ")
    (display
      (cond
        ((number? x)    "数値")
        ((string? x)    "文字列")
        ((boolean? x)   "ブーリアン")
        ((symbol? x)    "シンボル")
        ((pair? x)      "ペア")
        ((null? x)      "空リスト")
        ((procedure? x) "手続き")
        (else           "不明")))
    (newline))
  (list 42 "hello" #t 'foo '(1 2) '() +))

(display "=== 第4章 完了 ===") (newline)

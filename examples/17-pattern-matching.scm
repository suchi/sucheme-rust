;;; examples/17-pattern-matching.scm
;;; cprustscheme チュートリアル - 第17章: パターンマッチング
;;; 実行方法: cprustscheme examples/17-pattern-matching.scm

(display "=== 第17章: パターンマッチング ===") (newline)

;;; --- 1. cond によるパターンマッチ ---
(display "--- cond によるパターン ---") (newline)

(define (describe-list lst)
  (cond
    ((null? lst) "空リスト")
    ((null? (cdr lst)) (string-append "1要素: " (write-to-string (car lst))))
    ((null? (cddr lst)) (string-append "2要素: " (write-to-string (car lst))))
    (else (string-append (number->string (length lst)) "要素のリスト"))))

(define (write-to-string v)
  (with-output-to-string (lambda () (write v))))

(for-each
  (lambda (l)
    (display (describe-list l))
    (newline))
  '(() (1) (1 2) (1 2 3 4 5)))

;;; --- 2. 型によるディスパッチ ---
(display "--- 型ディスパッチ ---") (newline)

(define (type-dispatch x)
  (cond
    ((null? x)      (display "nil"))
    ((boolean? x)   (display "bool: ") (display x))
    ((number? x)    (display "num: ") (display x))
    ((char? x)      (display "char: ") (write x))
    ((string? x)    (display "str: ") (write x))
    ((symbol? x)    (display "sym: ") (display x))
    ((pair? x)      (display "pair: ") (display x))
    ((vector? x)    (display "vec: ") (display x))
    ((procedure? x) (display "proc"))
    (else           (display "unknown")))
  (newline))

(for-each type-dispatch
  (list '() #t 42 #\a "hello" 'foo '(1 2) #(3 4) car))

;;; --- 3. 構造的パターンマッチング ---
(display "--- 構造的マッチング ---") (newline)

;; match マクロ (簡易実装)
(define-syntax match
  (syntax-rules (quote _)
    ;; ベースケース: パターンなし
    ((match expr) (error "match: no matching clause"))
    ;; ワイルドカード
    ((match expr ((_ body ...) rest ...))
     (begin body ...))
    ;; リテラルマッチ
    ((match expr (('lit body ...) rest ...))
     (if (equal? expr 'lit)
         (begin body ...)
         (match expr rest ...)))
    ;; 変数束縛
    ((match expr ((var body ...) rest ...))
     (let ((var expr))
       body ...))))

;; S式の評価器風
(define (eval-simple expr env)
  (cond
    ((number? expr) expr)
    ((symbol? expr) (cdr (assq expr env)))
    ((pair? expr)
     (let ((op (car expr))
           (args (cdr expr)))
       (case op
         ((+) (apply + (map (lambda (e) (eval-simple e env)) args)))
         ((-) (apply - (map (lambda (e) (eval-simple e env)) args)))
         ((*) (apply * (map (lambda (e) (eval-simple e env)) args)))
         (else (error "Unknown op")))))
    (else (error "Cannot eval"))))

(define simple-env '((x . 5) (y . 10) (z . 3)))
(display (eval-simple '(+ x y) simple-env))  ; => 15
(newline)
(display (eval-simple '(* x (+ y z)) simple-env))  ; => 65
(newline)

;;; --- 4. 木構造のパターンマッチ ---
(display "--- 木構造の処理 ---") (newline)

;; 算術式の木を評価
;; 例: (add (mul 2 3) (sub 10 4)) => 12
(define (eval-tree tree)
  (if (number? tree)
      tree
      (let ((op   (car tree))
            (left  (eval-tree (cadr tree)))
            (right (eval-tree (caddr tree))))
        (case op
          ((add) (+ left right))
          ((sub) (- left right))
          ((mul) (* left right))
          ((div) (/ left right))
          (else (error "Unknown op"))))))

(display (eval-tree 42))                             ; => 42
(newline)
(display (eval-tree '(add (mul 2 3) (sub 10 4))))    ; => 12
(newline)
(display (eval-tree '(mul (add 1 2) (add 3 4))))     ; => 21
(newline)

;;; --- 5. シンボルパターン ---
(display "--- シンボルパターン ---") (newline)

;; コマンドパーサー
(define (parse-command cmd)
  (cond
    ((eq? cmd 'help)  "ヘルプを表示")
    ((eq? cmd 'quit)  "終了")
    ((eq? cmd 'list)  "リストを表示")
    (else (string-append "不明なコマンド: " (symbol->string cmd)))))

(for-each
  (lambda (cmd)
    (display cmd) (display " => ")
    (display (parse-command cmd))
    (newline))
  '(help quit list foo))

(display "=== 第17章 完了 ===") (newline)

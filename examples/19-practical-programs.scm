;;; examples/19-practical-programs.scm
;;; cprustscheme チュートリアル - 第19章: 実用的なプログラム
;;; 実行方法: cprustscheme examples/19-practical-programs.scm

(display "=== 第19章: 実用的なプログラム ===") (newline)

;;; --- 1. フィボナッチ数列 (複数の実装) ---
(display "--- フィボナッチ数列 ---") (newline)

;; 末尾再帰版 (効率的)
(define (fib n)
  (let loop ((i 0) (a 0) (b 1))
    (if (= i n) a (loop (+ i 1) b (+ a b)))))

;; 行列べき乗版 (O(log n))
(define (fib-fast n)
  (define (mat-mul A B)
    (let ((a00 (caar A)) (a01 (cadar A))
          (a10 (caadr A)) (a11 (cadadr A))
          (b00 (caar B)) (b01 (cadar B))
          (b10 (caadr B)) (b11 (cadadr B)))
      (list (list (+ (* a00 b00) (* a01 b10))
                  (+ (* a00 b01) (* a01 b11)))
            (list (+ (* a10 b00) (* a11 b10))
                  (+ (* a10 b01) (* a11 b11))))))
  (define (mat-pow M n)
    (cond
      ((= n 0) '((1 0) (0 1)))
      ((even? n) (let ((h (mat-pow M (/ n 2)))) (mat-mul h h)))
      (else (mat-mul M (mat-pow M (- n 1))))))
  (caar (mat-pow '((1 1) (1 0)) n)))

(display (map fib (iota 10)))         ; => (0 1 1 2 3 5 8 13 21 34)
(newline)
(display (fib 40))                    ; => 102334155
(newline)

;;; --- 2. テキスト処理 ---
(display "--- テキスト処理 ---") (newline)

;; 単語頻度カウント
(define (word-frequency text)
  (let* ((words (string-split (string-downcase text) " "))
         (freq '()))
    (for-each
      (lambda (w)
        (let ((entry (assoc w freq)))
          (if entry
              (set-cdr! entry (+ (cdr entry) 1))
              (set! freq (cons (cons w 1) freq)))))
      words)
    (sort (lambda (a b) (> (cdr a) (cdr b))) freq)))

(define text "the quick brown fox jumps over the lazy dog the fox")
(display "単語頻度:")
(newline)
(for-each
  (lambda (pair)
    (display "  ") (display (car pair))
    (display ": ") (display (cdr pair))
    (newline))
  (word-frequency text))

;;; --- 3. 簡単な計算機 ---
(display "--- RPN計算機 ---") (newline)

;; 逆ポーランド記法 (RPN) 計算機
(define (rpn-eval tokens)
  (let loop ((stack '()) (remaining tokens))
    (if (null? remaining)
        (if (null? stack) 0 (car stack))
        (let ((token (car remaining))
              (rest (cdr remaining)))
          (cond
            ((number? token)
             (loop (cons token stack) rest))
            ((and (pair? stack) (pair? (cdr stack)))
             (let* ((b (car stack))
                    (a (cadr stack))
                    (new-stack (cddr stack))
                    (result
                      (case token
                        ((+) (+ a b))
                        ((-) (- a b))
                        ((*) (* a b))
                        ((/) (if (= b 0) (error "div by 0") (/ a b)))
                        (else (error "Unknown operator")))))
               (loop (cons result new-stack) rest)))
            (else (error "Invalid RPN expression")))))))

;; (3 4 + 2 *) => (3 + 4) * 2 = 14
(display (rpn-eval '(3 4 + 2 *)))    ; => 14
(newline)
;; (5 1 2 + 4 * + 3 -) = 14
(display (rpn-eval '(5 1 2 + 4 * + 3 -)))  ; => 14
(newline)

;;; --- 4. Caesarの暗号 ---
(display "--- Caesar暗号 ---") (newline)

(define (caesar-shift n ch)
  (if (char-alphabetic? ch)
      (let* ((base (if (char-upper-case? ch) (char->integer #\A) (char->integer #\a)))
             (shifted (modulo (+ (- (char->integer ch) base) n) 26)))
        (integer->char (+ base shifted)))
      ch))

(define (caesar-encrypt n text)
  (list->string (map (lambda (c) (caesar-shift n c)) (string->list text))))

(define (caesar-decrypt n text)
  (caesar-encrypt (- 26 n) text))

(define original "Hello, World!")
(define encrypted (caesar-encrypt 13 original))
(define decrypted (caesar-decrypt 13 encrypted))

(display "元のテキスト: ") (display original) (newline)
(display "暗号化: ")       (display encrypted) (newline)
(display "復号化: ")       (display decrypted) (newline)

;;; --- 5. 簡単なパーサー ---
(display "--- JSONライクなパーサー ---") (newline)

;; 簡単な数式パーサー
(define (parse-number tokens)
  (values (string->number (car tokens)) (cdr tokens)))

(define (tokenize-math expr)
  (let loop ((chars (string->list expr)) (current "") (result '()))
    (cond
      ((null? chars)
       (reverse (if (string=? current "")
                    result
                    (cons current result))))
      ((char-whitespace? (car chars))
       (loop (cdr chars) "" (if (string=? current "")
                                result
                                (cons current result))))
      ((member (car chars) '(#\+ #\- #\* #\/))
       (loop (cdr chars) ""
             (cons (string (car chars))
                   (if (string=? current "") result (cons current result)))))
      (else
       (loop (cdr chars) (string-append current (string (car chars))) result)))))

(define tokens (tokenize-math "3 + 4 * 2"))
(display "トークン: ")
(display tokens)
(newline)

(display "=== 第19章 完了 ===") (newline)

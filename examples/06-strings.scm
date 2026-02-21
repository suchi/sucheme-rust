;;; examples/06-strings.scm
;;; sucheme-rust チュートリアル - 第6章: 文字列と文字
;;; 実行方法: sucheme-rust examples/06-strings.scm

(display "=== 第6章: 文字列と文字 ===") (newline)

;;; --- 1. 文字列の基本 ---
(display "--- 文字列の基本 ---") (newline)

(define s "Hello, Scheme!")
(display (string-length s))     ; => 14
(newline)
(display (string-ref s 0))      ; => H
(newline)
(display (substring s 0 5))     ; => Hello
(newline)
(display (string-append "foo" "bar" "baz"))  ; => foobarbaz
(newline)

;;; --- 2. 文字列比較 ---
(display "--- 文字列比較 ---") (newline)

(display (string=? "abc" "abc"))   ; => #t
(newline)
(display (string<? "abc" "abd"))   ; => #t
(newline)
(display (string>? "z" "a"))       ; => #t
(newline)
(display (string-ci=? "ABC" "abc"))  ; => #t (大文字小文字無視)
(newline)

;;; --- 3. 文字列変換 ---
(display "--- 文字列変換 ---") (newline)

(display (string->list "abc"))     ; => (a b c)
(newline)
(display (list->string '(#\H #\i)))  ; => Hi
(newline)
(display (string->number "42"))    ; => 42
(newline)
(display (string->number "3.14"))  ; => 3.14
(newline)
(display (string->number "abc"))   ; => #f
(newline)
(display (number->string 255 16))  ; => ff (16進数)
(newline)
(display (string->symbol "hello")) ; => hello
(newline)
(display (symbol->string 'world))  ; => world
(newline)

;;; --- 4. 文字列操作 ---
(display "--- 文字列操作 ---") (newline)

(display (string-upcase "hello"))   ; => HELLO
(newline)
(display (string-downcase "WORLD")) ; => world
(newline)

;; 文字列コピーと変更
(define ms (string-copy "hello"))
(string-set! ms 0 #\H)
(display ms)    ; => Hello
(newline)

;; 文字列分割 (stdlib.scm の関数)
(display (string-split "a,b,c" ","))  ; => (a b c)
(newline)

;; 文字列連結 (stdlib.scm の関数)
(display (string-join '("foo" "bar" "baz") "-"))  ; => foo-bar-baz
(newline)

;; プレフィックス/サフィックス確認 (stdlib.scm の関数)
(display (string-prefix? "Hello" "Hello, World!"))  ; => #t
(newline)
(display (string-suffix? "World!" "Hello, World!")) ; => #t
(newline)

;;; --- 5. 文字操作 ---
(display "--- 文字操作 ---") (newline)

(display (char->integer #\A))     ; => 65
(newline)
(display (integer->char 97))      ; => a
(newline)
(display (char-upcase #\a))       ; => A
(newline)
(display (char-downcase #\Z))     ; => z
(newline)
(display (char-alphabetic? #\a))  ; => #t
(newline)
(display (char-numeric? #\5))     ; => #t
(newline)
(display (char-whitespace? #\space))  ; => #t
(newline)

;; 文字比較
(display (char<? #\a #\b))  ; => #t
(newline)
(display (char=? #\A #\A))  ; => #t
(newline)

;;; --- 6. 文字列処理の実例 ---
(display "--- 文字列処理の実例 ---") (newline)

;; 文字列の反転
(define (string-reverse s)
  (list->string (reverse (string->list s))))

(display (string-reverse "Scheme"))  ; => emehcS
(newline)

;; 回文チェック
(define (palindrome? s)
  (let ((chars (string->list (string-downcase s))))
    (equal? chars (reverse chars))))

(display (palindrome? "racecar"))    ; => #t
(newline)
(display (palindrome? "hello"))      ; => #f
(newline)

;; 単語数カウント
(define (count-words str)
  (length (filter (lambda (s) (not (string=? s "")))
                  (string-split str " "))))

(display (count-words "Hello World Scheme"))  ; => 3
(newline)

(display "=== 第6章 完了 ===") (newline)

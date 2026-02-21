;;; examples/01-basics.scm
;;; cprustscheme チュートリアル - 第1章: 基本的な式と値
;;; 実行方法: cprustscheme examples/01-basics.scm

(display "=== 第1章: 基本的な式と値 ===") (newline)

;;; --- 1. 数値 ---
(display "--- 数値 ---") (newline)

;; 整数 (exact)
(display (+ 1 2))       ; => 3
(newline)

;; 浮動小数点数 (inexact)
(display 3.14)          ; => 3.14
(newline)

;; 基本算術
(display (+ 10 20))     ; => 30
(newline)
(display (- 100 37))    ; => 63
(newline)
(display (* 6 7))       ; => 42
(newline)
(display (/ 22 7))      ; => 3 (整数除算)
(newline)
(display (/ 22.0 7))    ; => 3.142857...
(newline)

;;; --- 2. ブーリアン ---
(display "--- ブーリアン ---") (newline)

(display #t)            ; => #t (真)
(newline)
(display #f)            ; => #f (偽)
(newline)
(display (not #f))      ; => #t
(newline)
(display (and #t #t))   ; => #t
(newline)
(display (or #f #t))    ; => #t
(newline)

;;; --- 3. 文字 ---
(display "--- 文字 ---") (newline)

(display #\a)           ; => a
(newline)
(display #\space)       ; => (スペース)
(newline)
(display #\newline)     ; => (改行)
(newline)
(display (char->integer #\A))  ; => 65
(newline)

;;; --- 4. 文字列 ---
(display "--- 文字列 ---") (newline)

(display "Hello, World!")
(newline)
(display (string-length "Scheme"))  ; => 6
(newline)
(display (string-append "Hello" ", " "World!"))  ; => Hello, World!
(newline)
(display (substring "Scheme" 0 3))  ; => Sch
(newline)

;;; --- 5. シンボル ---
(display "--- シンボル ---") (newline)

(display 'hello)        ; => hello
(newline)
(display 'scheme-is-fun) ; => scheme-is-fun
(newline)
(display (symbol? 'x))  ; => #t
(newline)

(display "=== 第1章 完了 ===") (newline)

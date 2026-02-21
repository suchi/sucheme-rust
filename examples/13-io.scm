;;; examples/13-io.scm
;;; cprustscheme チュートリアル - 第13章: 入出力
;;; 実行方法: cprustscheme examples/13-io.scm

(display "=== 第13章: 入出力 ===") (newline)

;;; --- 1. 基本的な出力 ---
(display "--- 基本的な出力 ---") (newline)

;; display: 人間向けの表示
(display "Hello, World!")
(newline)

;; write: プログラム向けの表示 (エスケープあり)
(write "Hello, \"World\"!")
(newline)
; => "Hello, \"World\"!"

;; newline
(display "行1")
(newline)
(display "行2")
(newline)

;; write-char: 一文字出力
(write-char #\H)
(write-char #\i)
(write-char #\newline)

;;; --- 2. write vs display の違い ---
(display "--- write vs display ---") (newline)

(let ((values (list "hello" #\a '(1 2 "three") #t)))
  (for-each
    (lambda (v)
      (display "display: ") (display v) (newline)
      (display "write:   ") (write v)   (newline)
      (display "---") (newline))
    values))

;;; --- 3. ファイル入出力 ---
(display "--- ファイル入出力 ---") (newline)

;; ファイルへの書き込み
(define tmp-file "/tmp/scheme-test-output.txt")

(call-with-output-file tmp-file
  (lambda (port)
    (display "Line 1\n" port)
    (display "Line 2\n" port)
    (write-char #\A port)
    (newline port)))

;; ファイルからの読み込み
(call-with-input-file tmp-file
  (lambda (port)
    (let loop ((line (read-line port)))
      (unless (eof-object? line)
        (display "読み込み: ")
        (display line)
        (newline)
        (loop (read-line port))))))

;;; --- 4. with-output-to-string ---
(display "--- 文字列への出力 ---") (newline)

(define result-string
  (with-output-to-string
    (lambda ()
      (display "computed: ")
      (display (* 6 7)))))

(display "文字列: ")
(write result-string)
(newline)
; => "computed: 42"

;;; --- 5. read による読み込み ---
(display "--- read ---") (newline)

;; 文字列からの read
(define (read-from-string s)
  (with-input-from-string s read))

(display (read-from-string "(+ 1 2 3)"))
; => (+ 1 2 3)
(newline)

(display (read-from-string "42"))
; => 42
(newline)

(display (read-from-string "'hello"))
; => (quote hello)
(newline)

;;; --- 6. print フォーマット ---
(display "--- フォーマット出力 ---") (newline)

;; シンプルなフォーマット関数
(define (format-number n width)
  (let* ((s (number->string n))
         (padding (make-string (max 0 (- width (string-length s))) #\space)))
    (string-append padding s)))

(for-each
  (lambda (n)
    (display (format-number n 6))
    (display " | ")
    (display (format-number (* n n) 8))
    (newline))
  '(1 5 10 100 999))

;;; --- 7. ポートの操作 ---
(display "--- ポートの操作 ---") (newline)

;; current-input-port, current-output-port
(display "標準出力ポート: ")
(display (output-port? (current-output-port)))
(newline)

;; open-output-string / get-output-string
(define str-port (open-output-string))
(display "test" str-port)
(display 42 str-port)
(display (get-output-string str-port))  ; => test42
(newline)

(display "=== 第13章 完了 ===") (newline)

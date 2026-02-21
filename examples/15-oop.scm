;;; examples/15-oop.scm
;;; sucheme-rust チュートリアル - 第15章: オブジェクト指向プログラミング
;;; 実行方法: sucheme-rust examples/15-oop.scm

(display "=== 第15章: オブジェクト指向プログラミング ===") (newline)

;;; --- 1. クロージャによるオブジェクト ---
(display "--- クロージャベースのOOP ---") (newline)

(define (make-animal name sound)
  (define (speak)
    (display name) (display " says: ") (display sound) (newline))
  (define (get-name) name)
  (define (dispatch msg . args)
    (cond
      ((eq? msg 'speak) (speak))
      ((eq? msg 'name)  (get-name))
      (else (error (string-append "Unknown message: "
                                  (symbol->string msg))))))
  dispatch)

(define dog (make-animal "Dog" "Woof"))
(define cat (make-animal "Cat" "Meow"))

(dog 'speak)  ; => Dog says: Woof
(cat 'speak)  ; => Cat says: Meow
(display (dog 'name))  ; => Dog
(newline)

;;; --- 2. 継承のシミュレーション ---
(display "--- 継承 ---") (newline)

(define (make-pet name sound owner)
  (let ((base (make-animal name sound)))
    (define (get-owner) owner)
    (define (dispatch msg . args)
      (cond
        ((eq? msg 'owner) (get-owner))
        ((eq? msg 'info)
         (display name) (display " owned by ")
         (display owner) (newline))
        (else (apply base (cons msg args)))))
    dispatch))

(define my-dog (make-pet "Pochi" "Woof" "Alice"))
(my-dog 'speak)    ; => Pochi says: Woof
(my-dog 'info)     ; => Pochi owned by Alice
(display (my-dog 'owner))  ; => Alice
(newline)

;;; --- 3. define-record-type によるOOP ---
(display "--- define-record-type ---") (newline)

(define-record-type <person>
  (make-person name age email)
  person?
  (name  person-name)
  (age   person-age  set-person-age!)
  (email person-email))

(define alice (make-person "Alice" 30 "alice@example.com"))
(define bob   (make-person "Bob"   25 "bob@example.com"))

(define (person-greet p)
  (display "Hi, I'm ") (display (person-name p))
  (display ", ") (display (person-age p))
  (display " years old.") (newline))

(person-greet alice)  ; => Hi, I'm Alice, 30 years old.
(person-greet bob)    ; => Hi, I'm Bob, 25 years old.

(set-person-age! alice 31)  ; 誕生日
(display (person-age alice)) ; => 31
(newline)

;;; --- 4. プロトタイプベース継承 ---
(display "--- プロトタイプ継承 ---") (newline)

(define (make-object . proto)
  (let ((self (list 'object))
        (parent (if (null? proto) #f (car proto))))
    (define (get key)
      (let ((entry (assq key (cdr self))))
        (if entry
            (cdr entry)
            (if parent (parent 'get key) #f))))
    (define (set! key val)
      (let ((entry (assq key (cdr self))))
        (if entry
            (set-cdr! entry val)
            (set-cdr! self (cons (cons key val) (cdr self))))))
    (define (dispatch msg . args)
      (case msg
        ((get) (get (car args)))
        ((set!) (set! (car args) (cadr args)))
        ((has?) (not (not (assq (car args) (cdr self)))))
        (else (error "Unknown object message"))))
    dispatch))

(define animal-proto (make-object))
(animal-proto 'set! 'type "animal")
(animal-proto 'set! 'speak (lambda (self) (display "...")))

(define dog-proto (make-object animal-proto))
(dog-proto 'set! 'type "dog")
(dog-proto 'set! 'speak (lambda (self) (display "Woof!")))

(display (dog-proto 'get 'type))    ; => dog
(newline)
((dog-proto 'get 'speak) dog-proto)  ; => Woof!
(newline)

;;; --- 5. ジェネリック関数 ---
(display "--- ジェネリック関数 ---") (newline)

(define-record-type <circle>
  (make-circle radius) circle?
  (radius circle-radius))

(define-record-type <rectangle>
  (make-rect width height) rect?
  (width rect-width) (height rect-height))

(define-record-type <triangle>
  (make-tri base height) tri?
  (base tri-base) (height tri-height))

(define (area shape)
  (cond
    ((circle? shape)
     (* 3.14159 (square (circle-radius shape))))
    ((rect? shape)
     (* (rect-width shape) (rect-height shape)))
    ((tri? shape)
     (/ (* (tri-base shape) (tri-height shape)) 2))
    (else (error "Unknown shape"))))

(define (perimeter shape)
  (cond
    ((circle? shape)
     (* 2 3.14159 (circle-radius shape)))
    ((rect? shape)
     (* 2 (+ (rect-width shape) (rect-height shape))))
    (else "N/A")))

(for-each
  (lambda (s)
    (display "面積: ") (display (area s)) (newline))
  (list (make-circle 5)
        (make-rect 4 6)
        (make-tri 3 4)))

(display "=== 第15章 完了 ===") (newline)

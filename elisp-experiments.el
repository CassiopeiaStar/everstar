
(+ 2 2)

(+2 (+ 1 1))

(setq my-name "Cassie")

(insert "Hello!")

(insert "Hello" " world!")

(defun hello () (insert "Hello, I am " my-name))

(hello)

(defun hello (name) (insert "Hello " name))

(hello "you")


(switch-to-buffer-other-window "*test*")

(progn
  (switch-to-buffer-other-window "*test*")
  (erase-buffer)
  (hello "there"))

(progn
  (switch-to-buffer-other-window "*test*")
  (erase-buffer)
  (hello "you")
  (other-window 1))

(let ((local-name "you"))
  (switch-to-buffer-other-window "*test*")
  (erase-buffer)
  (hello local-name)
  (other-window 1))

(fset 'hello-world (lambda () (insert "Hello World")))

(symbol-function 'hello-world)

(fset 'windout (lambda (msg)
  (progn
    (switch-to-buffer-other-window "*test*")
    (erase-buffer)
    (insert msg)
    (other-window 1))))

(windout "penis")

(defun hello (name)
  (windout (format "Hello %s!\n" name)))

(hello "fox")

(defun greeting (name)
  (let ((your-name "Cassie"))
    (windout (format "Hello %s!\n\nI am %s."
                    name
                    your-name
                    ))))


(greeting "you")

(read-from-minibuffer "enter your name: ")

(greeting (read-from-minibuffer "Enter your name: "))

(setq list-of-names '("Sarah" "Chloe" "Mathilde"))

(defun greet-all ()
  (let* ((your-name "Cassie")
        (msg (apply 'concat (mapcar (lambda (name)
          (format "Hello %s!\n\nI am %s.\n\n"
            name
            your-name))
          list-of-names))))
    (windout msg)))

(greet-all)


(defun replace-hello-by-bonjour ()
  (switch-to-buffer-other-window "*test*")
  (goto-char (point-min))
  (while (search-forward "Hello" nil t)
    (replace-match "Bonjour"))
  (other-window 1))

(replace-hello-by-bonjour)

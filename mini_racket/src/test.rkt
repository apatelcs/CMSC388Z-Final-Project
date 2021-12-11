#lang racket
(let ((wow 2))
  (let ((wow2 (- 1 5)))
    (+ wow (if #f 1 wow2))
  )  
)
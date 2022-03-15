# ispell-for-emacs --

    never tried this .... comes from vittorio castelli

     (setq ispell-program-name "aspell")                                      

    (defun  vc-ispell-and-tex()
     (message "In vc-ispell-and-tex")
     (setq text-mode-hook '(lambda () (local-set-key "\M-\t" 'ispell-complete-word)))
     (setq tex-mode-hook '(lambda () 
                            (local-set-key "\M-\t" 'ispell-complete-word)
                            (auto-fill-mode 1)
    ))
     (setq latex-mode-hook '(lambda () 
                              (local-set-key "\M-\t" 'ispell-complete-word)
                              (auto-fill-mode 1)
                              (vc-loader "vc-latex")
     )) 
     (setq tex-mode-hook '(lambda () 
                              (local-set-key "\M-\t" 'ispell-complete-word)
                              (auto-fill-mode 1)
                              (vc-loader "vc-latex")
     )) 
     (setq ispell-enable-tex-parser t) ;; enable tex parser, also very helpful
     (message "Loading vc-ispell-and-tex ... done")
    )

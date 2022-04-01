
# lein

$ lein new app FOO
$ lein repl
$ lein uberjar 
$ java -jar target/uberjar/clojure-noob-0.1.0-SNAPSHOT-standalone.jar


IN REPL

(require '[clojure.test :refer [run-tests]])
(require 'ppp-clj.core_test :reload-all)
(run-tests 'ppp-clj.core-test)

AUTO RELOADING REPL
- Paste this
  (do (require 'ppp-clj.core :reload-all) (-main))

- for running tests.  (first line once, then repeat second line)
  (require '[clojure.test :refer [run-tests]])
  (do (require 'ppp-clj.core_test :reload-all) (run-tests 'ppp-clj.core-test))


- up arrow then #enter


# DOCS
  Docs Page      http://clojuredocs.org/
  Closure Core   http://clojuredocs.org/clojure_core
  Tutorials      http://alexott.net/en/clojure/video.html

  QuickRef      http://faustus.webatu.com/clj-quick-ref.html




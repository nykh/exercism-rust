if [ -f ~/.config/exercism/exercism_completion.bash ]; then
  . ~/.config/exercism/exercism_completion.bash
fi

function start_exercise() {
  mkdir src
  touch src/lib.rs
  code .
}

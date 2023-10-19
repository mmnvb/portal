const { invoke } = window.__TAURI__.tauri;

let x1, y1, a1;
let x2, y2, a2;
let result;

async function rust() {
  result.textContent = await invoke("main_backend", { x1: Number(x1.value), y1: Number(y1.value),
     a1: Number(a1.value),  x2: Number(x2.value), y2: Number(y2.value), a2: Number(a2.value)});
}

window.addEventListener("DOMContentLoaded", () => {
  x1 = document.getElementById('x1');
  y1 = document.getElementById("y1");
  a1 = document.getElementById("a1");
  
  x2 = document.getElementById("x2");
  y2 = document.getElementById("y2");
  a2 = document.getElementById("a2");
  
  

  result = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    rust();
  });
});

var button = document.getElementById('click_btn');
var audio = document.getElementById('audio_click');
var audio_back = document.getElementById("audio_back");
var img_btn = document.getElementById("music_btn");
var isPlaying = false;

audio.volume = 0.2;
audio_back.volume = 0.2;

button.addEventListener('mousedown', function() {
  audio.currentTime = 0;
  audio.play();
});

img_btn.addEventListener("click", function() {
  if (isPlaying) {
    audio_back.pause();
    isPlaying = false;
  } else {
    audio_back.play();
    isPlaying = true;
  }
});


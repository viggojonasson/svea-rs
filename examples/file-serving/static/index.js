const btn = document.getElementById("button");
const counter = document.getElementById("counter");

let clicks = 0;

btn.addEventListener("click", () => {
  clicks++;
  counter.innerText = clicks + " clicks!";
});

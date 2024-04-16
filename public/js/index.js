function dismissAlert(event) {
  let alert = document.querySelector(event.currentTarget.dataset.dismissTarget);

  alert.classList.add("opacity-0", "duration-1000", "ease-out", "transition-opacity");

  setTimeout(() => {
    alert.classList.add("hidden");
  }, 1000);
}

document.addEventListener('DOMContentLoaded', function() {
  const alertDismissals = document.querySelectorAll("button[data-dismiss-target]");

  console.log("onload");
  alertDismissals.forEach(el => {
    el.addEventListener("click", dismissAlert);
  });
});

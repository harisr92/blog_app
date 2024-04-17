function dismissAlert(event) {
  let alert = document.querySelector(event.currentTarget.dataset.dismissTarget);

  alert.classList.add("opacity-0", "duration-1000", "ease-out", "transition-opacity");

  setTimeout(() => {
    alert.classList.add("hidden");
  }, 1000);
}

function showModal(message, actionElement, action) {
  const modal = document.getElementById('alert-modal');
  const messageEle = modal.querySelector('#alert-message');

  const options = {
    placement: 'bottom-right',
    backdrop: 'dynamic',
    backdropClasses:
        'bg-gray-900/50 dark:bg-gray-900/80 fixed inset-0 z-40',
    closable: true,
    onOk: function(event) {
      console.log(event);
      if (actionElement) {
        const event = new Event(action, {
            'bubbles'    : true,
            'cancelable' : true
        });
        actionElement.dispatchEvent(event);
      }
    }
  };

  messageEle.innerText = message

  const m = new Modal(modal, options);
  initModals();
  m.toggle();
}

function showAlert(event) {
  const actionEl = document.querySelector(event.currentTarget.dataset.modalTriggerEl);
  showModal("Are you sure?", actionEl, event.currentTarget.dataset.modalTriggerAction);
}

document.addEventListener('DOMContentLoaded', function() {
  const alertDismissals = document.querySelectorAll("button[data-dismiss-target]");
  const alertModal = document.querySelectorAll("button[data-alert]");

  alertDismissals.forEach(el => {
    el.addEventListener("click", dismissAlert);
  });

  alertModal.forEach(el => {
    el.addEventListener("click", showAlert);
  });
});

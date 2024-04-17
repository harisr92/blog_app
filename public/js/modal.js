class Instances {
    constructor() {
        this._instances = {
            Modal: {},
        };
    }
    addInstance(component, instance, id, override = false) {
        if (!this._instances[component]) {
            console.warn(`Component ${component} does not exist.`);
            return false;
        }
        if (this._instances[component][id] && !override) {
            console.warn(`Instance with ID ${id} already exists.`);
            return;
        }
        if (override && this._instances[component][id]) {
            this._instances[component][id].destroyAndRemoveInstance();
        }
        this._instances[component][id ? id : this._generateRandomId()] =
            instance;
    }
    getAllInstances() {
        return this._instances;
    }
    getInstances(component) {
        if (!this._instances[component]) {
            console.warn(`Component ${component} does not exist.`);
            return false;
        }
        return this._instances[component];
    }
    getInstance(component, id) {
        if (!this._componentAndInstanceCheck(component, id)) {
            return;
        }
        if (!this._instances[component][id]) {
            console.warn(`Instance with ID ${id} does not exist.`);
            return;
        }
        return this._instances[component][id];
    }
    destroyAndRemoveInstance(component, id) {
        if (!this._componentAndInstanceCheck(component, id)) {
            return;
        }
        this.destroyInstanceObject(component, id);
        this.removeInstance(component, id);
    }
    removeInstance(component, id) {
        if (!this._componentAndInstanceCheck(component, id)) {
            return;
        }
        delete this._instances[component][id];
    }
    destroyInstanceObject(component, id) {
        if (!this._componentAndInstanceCheck(component, id)) {
            return;
        }
        this._instances[component][id].destroy();
    }
    instanceExists(component, id) {
        if (!this._instances[component]) {
            return false;
        }
        if (!this._instances[component][id]) {
            return false;
        }
        return true;
    }
    _generateRandomId() {
        return Math.random().toString(36).substr(2, 9);
    }
    _componentAndInstanceCheck(component, id) {
        if (!this._instances[component]) {
            console.warn(`Component ${component} does not exist.`);
            return false;
        }
        if (!this._instances[component][id]) {
            console.warn(`Instance with ID ${id} does not exist.`);
            return false;
        }
        return true;
    }
}
const instances = new Instances();

const Default = {
  placement: "center",
  backdropClasses: "bg-gray-900/50 dark:bg-gray-900/80 fixed inset-0 z-40",
  backdrop: "dynamic",
  closable: true,
  onHide: () => {},
  onShow: () => {},
  onToggle: () => {},
  onOk: () => {}
}

const DefaultInstanceOptions = {
  id: null,
  override: true
}

class Modal {
  _eventListenerInstances = []

  constructor(
    targetEl = null,
    options = Default,
    instanceOptions = DefaultInstanceOptions
  ) {
    this._instanceId = instanceOptions.id ? instanceOptions.id : targetEl.id
    this._targetEl = targetEl
    this._options = { ...Default, ...options }
    this._isHidden = true
    this._backdropEl = null
    this._initialized = false
    this.init();
    instances.addInstance(
      'Modal',
      this,
      this._instanceId,
      instanceOptions.override
    );
  }

  init() {
    if (this._targetEl && !this._initialized) {
      this._getPlacementClasses().map(c => {
        this._targetEl.classList.add(c)
      })
      this._initialized = true
    }
  }

  destroy() {
    if (this._initialized) {
      this.removeAllEventListenerInstances()
      this._destroyBackdropEl()
      this._initialized = false
    }
  }

  removeInstance() {
    instances.removeInstance("Modal", this._instanceId)
  }

  destroyAndRemoveInstance() {
    this.destroy()
    this.removeInstance()
  }

  _createBackdrop() {
    if (this._isHidden) {
      const backdropEl = document.createElement("div")
      backdropEl.setAttribute("modal-backdrop", "")
      backdropEl.classList.add(...this._options.backdropClasses.split(" "))
      document.querySelector("body").append(backdropEl)
      this._backdropEl = backdropEl
    }
  }

  _destroyBackdropEl() {
    if (!this._isHidden) {
      document.querySelector("[modal-backdrop]").remove()
    }
  }

  _setupModalCloseEventListeners() {
    if (this._options.backdrop === "dynamic") {
      this._clickOutsideEventListener = ev => {
        this._handleOutsideClick(ev.target)
      }
      this._targetEl.addEventListener(
        "click",
        this._clickOutsideEventListener,
        true
      )
    }

    this._keydownEventListener = ev => {
      if (ev.key === "Escape") {
        this.hide()
      }
    }
    document.body.addEventListener("keydown", this._keydownEventListener, true)
  }

  _removeModalCloseEventListeners() {
    if (this._options.backdrop === "dynamic") {
      this._targetEl.removeEventListener(
        "click",
        this._clickOutsideEventListener,
        true
      )
    }
    document.body.removeEventListener(
      "keydown",
      this._keydownEventListener,
      true
    )
  }

  _handleOutsideClick(target) {
    if (
      target === this._targetEl ||
      (target === this._backdropEl && this.isVisible())
    ) {
      this.hide()
    }
  }

  _getPlacementClasses() {
    switch (this._options.placement) {
      // top
      case "top-left":
        return ["justify-start", "items-start"]
      case "top-center":
        return ["justify-center", "items-start"]
      case "top-right":
        return ["justify-end", "items-start"]

      // center
      case "center-left":
        return ["justify-start", "items-center"]
      case "center":
        return ["justify-center", "items-center"]
      case "center-right":
        return ["justify-end", "items-center"]

      // bottom
      case "bottom-left":
        return ["justify-start", "items-end"]
      case "bottom-center":
        return ["justify-center", "items-end"]
      case "bottom-right":
        return ["justify-end", "items-end"]

      default:
        return ["justify-center", "items-center"]
    }
  }

  toggle() {
    if (this._isHidden) {
      this.show()
    } else {
      this.hide()
    }

    // callback function
    this._options.onToggle(this)
  }

  show() {
    if (this.isHidden) {
      this._targetEl.classList.add("flex")
      this._targetEl.classList.remove("hidden")
      this._targetEl.setAttribute("aria-modal", "true")
      this._targetEl.setAttribute("role", "dialog")
      this._targetEl.removeAttribute("aria-hidden")
      this._createBackdrop()
      this._isHidden = false

      // Add keyboard event listener to the document
      if (this._options.closable) {
        this._setupModalCloseEventListeners()
      }

      // prevent body scroll
      document.body.classList.add("overflow-hidden")

      // callback function
      this._options.onShow(this)
    }
  }

  hide() {
    if (this.isVisible) {
      this._targetEl.classList.add("hidden")
      this._targetEl.classList.remove("flex")
      this._targetEl.setAttribute("aria-hidden", "true")
      this._targetEl.removeAttribute("aria-modal")
      this._targetEl.removeAttribute("role")
      this._destroyBackdropEl()
      this._isHidden = true

      // re-apply body scroll
      document.body.classList.remove("overflow-hidden")

      if (this._options.closable) {
        this._removeModalCloseEventListeners()
      }

      // callback function
      this._options.onHide(this)
    }
  }

  accept() {
    this.hide();
    this._options.onOk(this);
  }

  isVisible() {
    return !this._isHidden
  }

  isHidden() {
    return this._isHidden
  }

  addEventListenerInstance(element, type, handler) {
    this._eventListenerInstances.push({
      element: element,
      type: type,
      handler: handler
    })
  }

  removeAllEventListenerInstances() {
    this._eventListenerInstances.map(eventListenerInstance => {
      eventListenerInstance.element.removeEventListener(
        eventListenerInstance.type,
        eventListenerInstance.handler
      )
    })
    this._eventListenerInstances = []
  }

  getAllEventListenerInstances() {
    return this._eventListenerInstances
  }

  updateOnShow(callback) {
    this._options.onShow = callback
  }

  updateOnHide(callback) {
    this._options.onHide = callback
  }

  updateOnToggle(callback) {
    this._options.onToggle = callback
  }
}

function initModals() {
  // initiate modal based on data-modal-target
  document.querySelectorAll("[data-modal-target]").forEach($triggerEl => {
    const modalId = $triggerEl.getAttribute("data-modal-target")
    const $modalEl = document.getElementById(modalId)

    if ($modalEl) {
      const placement = $modalEl.getAttribute("data-modal-placement")
      const backdrop = $modalEl.getAttribute("data-modal-backdrop")
      new Modal($modalEl, {
        placement: placement ? placement : Default.placement,
        backdrop: backdrop ? backdrop : Default.backdrop
      })
    } else {
      console.error(
        `Modal with id ${modalId} does not exist. Are you sure that the data-modal-target attribute points to the correct modal id?.`
      )
    }
  })

  // toggle modal visibility
  document.querySelectorAll("[data-modal-toggle]").forEach($triggerEl => {
    const modalId = $triggerEl.getAttribute("data-modal-toggle")
    const $modalEl = document.getElementById(modalId)

    if ($modalEl) {
      const modal = instances.getInstance("Modal", modalId)

      if (modal) {
        const toggleModal = () => {
          modal.toggle()
        }
        $triggerEl.addEventListener("click", toggleModal)
        modal.addEventListenerInstance($triggerEl, "click", toggleModal)
      } else {
        console.error(
          `Modal with id ${modalId} has not been initialized. Please initialize it using the data-modal-target attribute.`
        )
      }
    } else {
      console.error(
        `Modal with id ${modalId} does not exist. Are you sure that the data-modal-toggle attribute points to the correct modal id?`
      )
    }
  })

  // show modal on click if exists based on id
  document.querySelectorAll("[data-modal-show]").forEach($triggerEl => {
    const modalId = $triggerEl.getAttribute("data-modal-show")
    const $modalEl = document.getElementById(modalId)

    if ($modalEl) {
      const modal = instances.getInstance("Modal", modalId)

      if (modal) {
        const showModal = () => {
          modal.show()
        }
        $triggerEl.addEventListener("click", showModal)
        modal.addEventListenerInstance($triggerEl, "click", showModal)
      } else {
        console.error(
          `Modal with id ${modalId} has not been initialized. Please initialize it using the data-modal-target attribute.`
        )
      }
    } else {
      console.error(
        `Modal with id ${modalId} does not exist. Are you sure that the data-modal-show attribute points to the correct modal id?`
      )
    }
  })

  // hide modal on click if exists based on id
  document.querySelectorAll("[data-modal-hide]").forEach($triggerEl => {
    const modalId = $triggerEl.getAttribute("data-modal-hide")
    const $modalEl = document.getElementById(modalId)

    if ($modalEl) {
      const modal = instances.getInstance("Modal", modalId)

      if (modal) {
        const hideModal = () => {
          modal.hide()
        }
        $triggerEl.addEventListener("click", hideModal)
        modal.addEventListenerInstance($triggerEl, "click", hideModal)
      } else {
        console.error(
          `Modal with id ${modalId} has not been initialized. Please initialize it using the data-modal-target attribute.`
        )
      }
    } else {
      console.error(
        `Modal with id ${modalId} does not exist. Are you sure that the data-modal-hide attribute points to the correct modal id?`
      )
    }
  })

  // hide modal on click if exists based on id
  document.querySelectorAll("[data-modal-ok]").forEach($triggerEl => {
    const modalId = $triggerEl.getAttribute("data-modal-ok")
    const $modalEl = document.getElementById(modalId)

    if ($modalEl) {
      const modal = instances.getInstance("Modal", modalId)

      if (modal) {
        const acceptModal = () => {
          modal.accept()
        }
        $triggerEl.addEventListener("click", acceptModal)
        modal.addEventListenerInstance($triggerEl, "click", acceptModal)
      } else {
        console.error(
          `Modal with id ${modalId} has not been initialized. Please initialize it using the data-modal-target attribute.`
        )
      }
    } else {
      console.error(
        `Modal with id ${modalId} does not exist. Are you sure that the data-modal-hide attribute points to the correct modal id?`
      )
    }
  })
}

if (typeof window !== "undefined") {
  window.Modal = Modal
  window.initModals = initModals
}

export function withTransition(element: HTMLElement, duration = 200) {
  element.style.transition = `opacity ${duration}ms ease, transform ${duration}ms ease`;
}

export function fadeIn(element: HTMLElement) {
  element.style.opacity = "0";
  element.style.transform = "translateY(8px)";
  requestAnimationFrame(() => {
    element.style.opacity = "1";
    element.style.transform = "translateY(0)";
  });
}

export function staggerIn(elements: HTMLElement[], delay = 50) {
  elements.forEach((el, i) => {
    setTimeout(() => fadeIn(el), i * delay);
  });
}

export function slideIn(
  element: HTMLElement,
  direction: "left" | "right" = "left",
) {
  const offset = direction === "left" ? -20 : 20;
  element.style.opacity = "0";
  element.style.transform = `translateX(${offset}px)`;
  requestAnimationFrame(() => {
    element.style.opacity = "1";
    element.style.transform = "translateX(0)";
  });
}

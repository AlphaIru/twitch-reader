const container = document.getElementById("danmaku-container");
const laneHeight = 64;
const duration = 5000;

const maxLanes = Math.floor(window.innerHeight / laneHeight);
const lanes = new Array(maxLanes).fill(0);

const socket = new WebSocket(`ws://${NICO_ADDR}/ws`);

socket.onmessage = (event) => {
  createComment(event.data);
};

function createComment(text) {
  const el = document.createElement("div");
  el.className = "comment";
  el.innerText = text;
  container.appendChild(el);

  const laneIndex = findAvailableLane();
  const topPosition = laneIndex * laneHeight;

  el.style.top = `${topPosition}px`;
  el.style.animation = `scroll ${duration}ms linear forwards`;

  lanes[laneIndex] = Date.now() + duration * 0.4;

  el.onanimationend = () => {
    el.remove();
  };
}

function findAvailableLane() {
  const now = Date.now();
  for (let i = 0; i < lanes.length; i++) {
    if (lanes[i] < now) {
      return i;
    }
  }
  return Math.floor(Math.random() * maxLanes);
}

socket.onopen = () => {
  console.log("WebSocket connected!");
};

socket.onclose = () => {
  console.log("WebSocket disconnected! Check the server is running!");
};

import { BoidField } from "../pkg/wasm_boids";
import { memory } from "../pkg/wasm_boids_bg.wasm";

const fps = new class {
  constructor() {
    this.fps = document.getElementById("fps");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  render() {
    // Convert the delta time since the last frame render into a measure
    // of frames per second.
    const now = performance.now();
    const delta = now - this.lastFrameTimeStamp;
    this.lastFrameTimeStamp = now;
    const fps = 1 / delta * 1000;

    // Save only the latest 100 timings.
    this.frames.push(fps);
    if (this.frames.length > 100) {
      this.frames.shift();
    }

    // Find the max, min, and mean of our 100 latest timings.
    let min = Infinity;
    let max = -Infinity;
    let sum = 0;
    for (let i = 0; i < this.frames.length; i++) {
      sum += this.frames[i];
      min = Math.min(this.frames[i], min);
      max = Math.max(this.frames[i], max);
    }
    let mean = sum / this.frames.length;

    // Render the statistics.
    this.fps.textContent = `
Frames per Second:
         latest = ${Math.round(fps)}
avg of last 100 = ${Math.round(mean)}
min of last 100 = ${Math.round(min)}
max of last 100 = ${Math.round(max)}
`.trim();
  }
};
const url = new URL(window.location.href);
const targetFps = 60;
const hz = 1000 / targetFps;
var boid_count = 1500;
if (url.searchParams.has("boids")){
  boid_count = parseInt(url.searchParams.get("boids"));
}
const prop_count = 3;
const width = 1600;
const height = 900;
// const width = window.innerWidth;
// const height = window.innerHeight;
const field = BoidField.new(width, height, boid_count);

const canvas = document.getElementById("boids-canvas");
canvas.width = width;
canvas.height = height;


function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

function renderJS(){
  fps.render();

  const ctx = canvas.getContext('2d');
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  field.tick(width, height);
  const bufferPtr = field.buffer_pointer();
  const buffer_array = new Float64Array(memory.buffer, bufferPtr, (boid_count * prop_count));
  const boids = [];

  for (let i = 0; i < boid_count; i++) {
    boids.push({
      x: buffer_array[i * 3],
      y: buffer_array[i * 3 + 1],
      rot: buffer_array[i * 3 + 2],
    });
  }
  
  for (var i = 0; i < boids.length; i++) {
    var boid = boids[i];
    ctx.save();
    ctx.translate(boid.x, boid.y);
    ctx.rotate(boid.rot * 2 * Math.PI);
    ctx.beginPath();
    ctx.moveTo(0, 0);
    ctx.lineTo(4, -2);
    ctx.lineTo(0, 10);
    ctx.lineTo(-4, -2);
    ctx.lineTo(0, 0);
    ctx.closePath();
    ctx.fillStyle = '#FFFFFF';
    ctx.fill();
    ctx.restore();
  }
  // sleep(hz).then(() => requestAnimationFrame(renderJS));
  requestAnimationFrame(renderJS);
}


window.requestAnimationFrame(renderJS);
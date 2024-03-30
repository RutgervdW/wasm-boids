import { Universe, Cell } from "../pkg/wasm_boids";
import { memory } from "wasm-boids/wasm_boids_bg.wasm";
const fps = 20;
const hz = 1000 / fps;
const universe = Universe.new(width, height);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  universe.tick();

  sleep(hz).then(() => requestAnimationFrame(renderLoop));
};

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

requestAnimationFrame(renderLoop);
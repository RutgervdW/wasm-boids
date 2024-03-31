import { BoidField } from "../pkg/wasm_boids";
import { memory } from "../pkg/wasm_boids_bg.wasm";
const fps = 20;
const hz = 1000 / fps;
const field = BoidField.new(100, 100, 100);

const canvas = document.getElementById("boids-canvas");

const ctx = canvas.getContext('2d');

// const renderLoop = () => {
//   universe.tick();

//   sleep(hz).then(() => requestAnimationFrame(renderLoop));
// };

// function sleep(ms) {
//     return new Promise(resolve => setTimeout(resolve, ms));
// }

const boid_count = 100;
const prop_count = 3;
console.log("field should look like this: " + field.test(boid_count, prop_count));
const bufferPtr = field.buffer_pointer();
const buffer_array = new Float64Array(memory.buffer, bufferPtr, (boid_count * prop_count));
console.log("field looks like this: " + buffer_array);
const boids = [];
for (let i = 0; i < boid_count; i++) {
  boids.push({
    x: buffer_array[i * 3],
    y: buffer_array[i * 3 + 1],
    rot: buffer_array[i * 3 + 2],
  });
}
console.log(boids);

// requestAnimationFrame(renderLoop);
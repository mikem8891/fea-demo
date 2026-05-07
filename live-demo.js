//@ts-check
"use strict";

import initWasm, {Lin2DStaticModel, Node2D, KnownType, init_fea} from "./pkg/learn_fea.js";

import * as doc from "./modules/doc.js";
import * as load from "./modules/load.js";

const wasm = await initWasm();

/** @type {Lin2DStaticModel} */
let model;

async function setup() {
  
  console.log("running wasm");
  wasm.main();
  console.log("finished wasm");

  setupMaterialInputs();

  console.log("setting up canvas");
  const canvasPromise = setupCanvas();

  console.log("setting up FEA");
  const stepBtn = doc.getElementById("step-button");
  const playBtn = doc.getElementById("play-button");
  const stopBtn = doc.getElementById("stop-button");
  const errorDiv = doc.getElementById("errors");

  const {renderAll, renderNodes} = await canvasPromise.catch((err) => {
    errorDiv.innerText = err.message;
    errorDiv.hidden = false;
    throw err;
  });

  console.log("setting up inputs");
  setupFileManagement(renderAll)
  setupNodeInputs(renderNodes);
  setupElementInputs(renderAll);

  const renderStep = () => {
    errorDiv.innerText = "";
    try {
      for (let i = 0; i < 10; i++) {
        model.step();
      }
      doc.getInputElementById("node-index").dispatchEvent(new Event("change"));
      errorDiv.hidden = true;
      renderAll();
    } catch (error) {
      if (error instanceof Error) {
        errorDiv.innerText = error.message;
      } else {
        errorDiv.innerText = "unknown error";
      }
      errorDiv.hidden = false;
    }
  };
    
  stepBtn.addEventListener("click", () => {
    if (animation.id) {
      cancelAnimationFrame(animation.id);
      animation.id = null;
    }
    renderStep();
  });

  /** @type {{id: number?}} */
  const animation = {id: null};

  playBtn.addEventListener("click", (_) => {
    const loop = () => {
      renderStep();
      animation.id = requestAnimationFrame(loop);
    };
    loop();
  });

  stopBtn.addEventListener("click", (_) => {
    if (animation.id) {
      cancelAnimationFrame(animation.id);
      animation.id = null;
    }
  });

  doc.getElementById("new").dispatchEvent(new Event("click"));

  console.log("setup complete!");
}

function setupMaterialInputs() {
  const elasticity = doc.getInputElementById("elasticity");
  const poissonsRatio = doc.getInputElementById("poissons-ratio");
  const rigidity = doc.getInputElementById("rigidity");

  elasticity.addEventListener("change", changeMaterials);
  poissonsRatio.addEventListener("change", changeMaterials);
  rigidity.addEventListener("change", changeMaterials);
  
  
  /**
   * @param {Event=} _evt
   */
  function changeMaterials(_evt) {
    const e = parseFloat(elasticity.value);
    const nu = parseFloat(poissonsRatio.value);
    const g = parseFloat(rigidity.value);
    model.set_elasticity(e, nu, g);
  }
}

/**
 * @param {() => void} renderNodes
 */
function setupNodeInputs(renderNodes) {
  const addNodeBtn = doc.getElementById("add-node");
  const deleteNodeBtn = doc.getElementById("delete-node");
  const nodeIndex = doc.getInputElementById("node-index");
  const positionX = doc.getInputElementById("position-x");
  const positionY = doc.getInputElementById("position-y");
  const known = {
    displacement: {
      x: doc.getInputElementById("known-displacement-x"),
      y: doc.getInputElementById("known-displacement-y")
    },
    force: {
      x: doc.getInputElementById("known-force-x"),
      y: doc.getInputElementById("known-force-y")
    },
  };
  const displacementX = doc.getInputElementById("displacement-x");
  const displacementY = doc.getInputElementById("displacement-y");
  const forceX = doc.getInputElementById("force-x");
  const forceY = doc.getInputElementById("force-y");

  addNodeBtn.addEventListener("click", (evt) => {
    const lastNode = model.nodes_len();
    nodeIndex.value = lastNode.toString();

    model.add_node();
    changeNodeIndex(evt);
    positionX.select();
    model.set_all_nodes();
    renderNodes();
  });

  deleteNodeBtn.addEventListener("click", (evt) => {
    let index = parseInt(nodeIndex.value);
  
    model.delete_node(index);
    changeNodeIndex(evt);
    renderNodes();
  });

  /**
   * @param {Event} _evt 
   */
  function changeNodeIndex(_evt) {
    let index = parseInt(nodeIndex.value);
    if (model.nodes_len() <= 0 || isNaN(index)) {
      nodeIndex.value = "";
      clearNodeInputs();
      return;
    }
    if (index < 0) {
      index = 0;
      nodeIndex.select();
    } else if (index >= model.nodes_len()) {
      index = model.nodes_len() - 1;
      nodeIndex.select();
    }
    nodeIndex.value = index.toString();
    const node = model.get_node(index);
    setNodeInputsTo(node);
    node.free();
  }

  /**
   * @param {Node2D} node 
   */
  function setNodeInputsTo(node) {
    const nodeKnown = {
      x: node.knownX,
      y: node.knownY
    };
    
    known.force.x.checked = nodeKnown.x == KnownType.Force;
    known.displacement.x.checked = nodeKnown.x == KnownType.Displacement;
    known.force.y.checked = nodeKnown.y == KnownType.Force;
    known.displacement.y.checked = nodeKnown.y == KnownType.Displacement;

    positionX.value = node.posX.toString();
    positionY.value = node.posY.toString();
    displacementX.value = node.dispX.toString();
    displacementY.value = node.dispY.toString();
    forceX.value = node.forceX.toString();
    forceY.value = node.forceY.toString();
  }

  function clearNodeInputs() {
    known.force.x.checked = false;
    known.displacement.x.checked = false;
    known.force.y.checked = false;
    known.displacement.y.checked = false;

    positionX.value = "";
    positionY.value = "";
    displacementX.value = "";
    displacementY.value = "";
    forceX.value = "";
    forceY.value = "";
  }

  nodeIndex.addEventListener("change", changeNodeIndex);

  /**
   * @param {Event} _evt 
   */
  function changeNodeProp(_evt) {
    const index = parseInt(nodeIndex.value);
    if (model.nodes_len() <= 0 || isNaN(index)) {
      nodeIndex.value = "";
      clearNodeInputs();
      return;
    }
    const node = model.get_node(index);
    
    if (known.force.x.checked) {
      node.knownX = KnownType.Force;
    } else {
      node.knownX = KnownType.Displacement;
    }
    if (known.force.y.checked) {
      node.knownY = KnownType.Force;
    } else {
      node.knownY = KnownType.Displacement;
    }

    node.posX = parseFloat(positionX.value);
    node.posY = parseFloat(positionY.value);
    node.dispX = parseFloat(displacementX.value);
    node.dispY = parseFloat(displacementY.value);
    node.forceX = parseFloat(forceX.value);
    node.forceY = parseFloat(forceY.value);

    model.set_node(index, node);
    
    setNodeInputsTo(node);
    renderNodes();
  }

  known.force.x.addEventListener("click", changeNodeProp);
  known.force.y.addEventListener("click", changeNodeProp);
  known.displacement.x.addEventListener("click", changeNodeProp);
  known.displacement.y.addEventListener("click", changeNodeProp);
  positionX.addEventListener("change", changeNodeProp);
  positionY.addEventListener("change", changeNodeProp);
  displacementX.addEventListener("change", changeNodeProp);
  displacementY.addEventListener("change", changeNodeProp);
  forceX.addEventListener("change", changeNodeProp);
  forceY.addEventListener("change", changeNodeProp);
}

/**
 * @param {() => void} renderAll
 */
function setupElementInputs(renderAll) {
  const addElementBtn = doc.getElementById("add-element");
  const deleteElementBtn = doc.getElementById("delete-element");
  const elementIndex = doc.getInputElementById("element-index");
  const elementNodes = [
    doc.getInputElementById("element-node-0"),
    doc.getInputElementById("element-node-1"), 
    doc.getInputElementById("element-node-2")
  ];

  addElementBtn.addEventListener("click", (_evt) => {
    const nodeLen = model.nodes_len();
    if (nodeLen < 3) {
      alert("Not enough nodes to make an element.");
      elementIndex.value = "";
      return;
    }
    const elementLen = model.elements_len();
    elementIndex.value = elementLen.toString();
    model.add_elem();

    const indices = model.get_element_indices(elementLen);
    elementNodes[0].value = indices[0].toString();
    elementNodes[1].value = indices[1].toString();
    elementNodes[2].value = indices[2].toString();
    renderAll();
  });

  deleteElementBtn.addEventListener("click", () => {
    const elementLen = model.elements_len();
    if (elementLen < 1) {
      alert("No elements to delete.");
      return;
    }
    const index = parseInt(elementIndex.value);
    model.delete_element(index);
    changeElementIndex();
    const nodeIndex = doc.getInputElementById("node-index");
    nodeIndex.dispatchEvent(new Event("change"));
    renderAll();
  });

  function changeElementIndex() {
    let index = parseInt(elementIndex.value);
    if (model.elements_len() <= 0 || isNaN(index)) {
      elementIndex.value = "";
      elementNodes[0].value = "";
      elementNodes[1].value = "";
      elementNodes[2].value = "";
      return;
    }
    if (index < 0) {
      index = 0;
      elementIndex.select();
    } else if (index >= model.elements_len()) {
      index = model.elements_len() - 1;
      elementIndex.select();
    }
    elementIndex.value = index.toString();

    const indices = model.get_element_indices(index);
    elementNodes[0].value = indices[0].toString();
    elementNodes[1].value = indices[1].toString();
    elementNodes[2].value = indices[2].toString();
  }

  elementIndex.addEventListener("change", changeElementIndex);

  /**
   * @param {Event} _evt 
   */
  function changeElementIndices(_evt) {
    const index = parseInt(elementIndex.value);
    const indices = new Uint32Array(3);

    for (let i = 0; i < 3; i++){
      indices[i] = parseInt(elementNodes[i].value);
    }

    model.set_element_indices(index, indices);
    renderAll();
  }

  elementNodes[0].addEventListener("change", changeElementIndices);
  elementNodes[1].addEventListener("change", changeElementIndices);
  elementNodes[2].addEventListener("change", changeElementIndices);
}

async function setupCanvas() {
  
  const canvasId = "canvas";
  const canvas = /** @type {HTMLCanvasElement} */(doc.getElementById(canvasId));

  /**
   * @param {string} message
   */
  function error(message) {
    alert(message);
    throw new Error(message);
  }

  if (canvas.tagName.toLowerCase() != "canvas") {
    throw new Error(`Element id = "${canvasId}" is a "${canvas.tagName}" instead of a canvas element`);
  }
  const gl = /** @type {WebGLRenderingContext?} */(canvas.getContext("webgl2"));
  if (gl === null) {
    error("WebGL2 is not supported");
    throw null;
  }

  // Clear the canvas
  gl.clearColor(0.0, 0.0, 0.0, 1.0);
  gl.viewport(0, 0, canvas.width, canvas.height);
  gl.clear(gl.COLOR_BUFFER_BIT);

  const uniforms = {
    maxStress: 30,
    scaleDisp: 50,
    view: {
      center: [0.5, 0.5],
      range: [2.0, 2.0],
    }
  }
  uniforms.view.range[0] = uniforms.view.range[1] * canvas.clientWidth / canvas.clientHeight;

  const nodes = await setupNodeProgram(gl);
  gl.useProgram(nodes.program);
  gl.uniform1f(nodes.scaleDisp.location, uniforms.scaleDisp);


  const elements = await setupElementProgram(gl);
  gl.useProgram(elements.program);
  gl.uniform1f(elements.scaleDisp.location, uniforms.scaleDisp);
  gl.uniform1f(elements.maxStress.location, uniforms.maxStress);

  const renderNodes = () => {
    model.set_all_nodes();

    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

    gl.useProgram(nodes.program);
    
    gl.enableVertexAttribArray(nodes.position.location); // Enables the attribute
    gl.enableVertexAttribArray(nodes.displacement.location); // Enables the attribute
    
    gl.uniform2fv(nodes.view.center.location, uniforms.view.center);
    gl.uniform2fv(nodes.view.range.location, uniforms.view.range);
    gl.uniform1f(nodes.scaleDisp.location, uniforms.scaleDisp);
    
    gl.bindBuffer(gl.ARRAY_BUFFER, nodes.position.buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, nodes.position.data);
    gl.bindBuffer(gl.ARRAY_BUFFER, nodes.displacement.buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, nodes.displacement.data);
    
    gl.drawArrays(gl.POINTS, 0, wasm.get_nodes_len()) // nodes
    
    gl.disableVertexAttribArray(nodes.position.location); // Disable the attribute
    gl.disableVertexAttribArray(nodes.displacement.location); // Disable the attribute
  };

  const renderAll = () => {
    model.set_all_outputs();

    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
    
    gl.useProgram(elements.program);
    
    gl.enableVertexAttribArray(elements.displacement.location); // Enables the attribute
    gl.enableVertexAttribArray(elements.position.location); // Enables the attribute
    gl.enableVertexAttribArray(elements.stress.location); // Enables the attribute
    
    gl.uniform2fv(elements.view.center.location, uniforms.view.center);
    gl.uniform2fv(elements.view.range.location, uniforms.view.range);
    gl.uniform1f(elements.scaleDisp.location, uniforms.scaleDisp);
    gl.uniform1f(elements.maxStress.location, uniforms.maxStress);
    
    gl.bindBuffer(gl.ARRAY_BUFFER, elements.position.buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, elements.position.data);
    gl.bindBuffer(gl.ARRAY_BUFFER, elements.displacement.buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, elements.displacement.data);
    gl.bindBuffer(gl.ARRAY_BUFFER, elements.stress.buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, elements.stress.data);
    
    gl.drawArrays(gl.TRIANGLES, 0, 3 * model.elements_len()); //elements
    
    gl.disableVertexAttribArray(elements.displacement.location);
    gl.disableVertexAttribArray(elements.position.location);
    gl.disableVertexAttribArray(elements.stress.location);
    
    gl.useProgram(nodes.program);
    
    gl.enableVertexAttribArray(nodes.position.location); // Enables the attribute
    gl.enableVertexAttribArray(nodes.displacement.location); // Enables the attribute
    
    gl.uniform2fv(nodes.view.center.location, uniforms.view.center);
    gl.uniform2fv(nodes.view.range.location, uniforms.view.range);
    gl.uniform1f(nodes.scaleDisp.location, uniforms.scaleDisp);
    
    gl.bindBuffer(gl.ARRAY_BUFFER, nodes.position.buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, nodes.position.data);
    gl.bindBuffer(gl.ARRAY_BUFFER, nodes.displacement.buffer);
    gl.bufferSubData(gl.ARRAY_BUFFER, 0, nodes.displacement.data);
    
    gl.drawArrays(gl.POINTS, 0, wasm.get_nodes_len()) // nodes
    
    gl.disableVertexAttribArray(nodes.position.location); // Disable the attribute
    gl.disableVertexAttribArray(nodes.displacement.location); // Disable the attribute
  }

  canvas.addEventListener("mousemove", (evt) => {
    if (!evt.buttons || evt.buttons != 1) return;
    uniforms.view.center[0] -= evt.movementX * uniforms.view.range[0] / canvas.clientWidth;
    uniforms.view.center[1] += evt.movementY * uniforms.view.range[1] / canvas.clientHeight;
    renderAll();
  });

  canvas.addEventListener("wheel", (evt) => {
    const zoom = 1.1;
    const unzoom = 0.9;
    const rect = canvas.getBoundingClientRect();
    let dx = ((evt.clientX - rect.left) / canvas.clientWidth - 0.5) * uniforms.view.range[0];
    let dy = (0.5 - (evt.clientY - rect.top) / canvas.clientHeight) * uniforms.view.range[1];
    const scale = (evt.deltaY < 0) ? zoom : unzoom;
    uniforms.view.range[0] *= scale;
    uniforms.view.range[1] *= scale;
    uniforms.view.center[0] += dx * (1 - scale);
    uniforms.view.center[1] += dy * (1 - scale);
    renderAll();
  });

  const canvasHooks = {
    canvas,
    context: gl,
    renderAll,
    renderNodes,
  }

  console.log("canvas setup complete!");
  return canvasHooks;
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", setup);
} else {
  setup();
}

/**
 * @param {WebGL2RenderingContext | WebGLRenderingContext} gl
 */
async function setupElementProgram(gl) {

  // Get shader source code from files
  const displacementVertexShaderSource = load.text("shaders/displacement.vs.glsl");
  const stressFragmentShaderSource = load.text("shaders/stress.fs.glsl");

  const program = load.shaderProgram(
    gl, 
    await displacementVertexShaderSource, 
    await stressFragmentShaderSource
  );

  gl.useProgram(program);

  const maxFaces = wasm.max_faces();
  const elements = {
    program,
    view: {
      center: {
        location: gl.getUniformLocation(program, "view.center")
      },
      range: {
        location: gl.getUniformLocation(program, "view.range")
      }
    },
    scaleDisp: {
      location: gl.getUniformLocation(program, "scaleDisp")
    },
    maxStress: {
      location: gl.getUniformLocation(program, "maxStress")
    },
    position: {
      buffer: gl.createBuffer(),
      data: new Float32Array(
        wasm.memory.buffer, 
        wasm.get_element_positions(), 
        2 * maxFaces
      ),
      location: gl.getAttribLocation(program, "vertPosition"),
    },
    displacement: {
      buffer: gl.createBuffer(),
      data: new Float32Array(
        wasm.memory.buffer, 
        wasm.get_element_displacements(), 
        2 * maxFaces
      ),
      location: gl.getAttribLocation(program, "vertDisp"),
    },
    stress: {
      buffer: gl.createBuffer(),
      data: new Float32Array(
        wasm.memory.buffer, 
        wasm.get_element_stresses(), 
        3 * maxFaces
      ),
      location: gl.getAttribLocation(program, "vertStress"),
    },
  }

  // Create buffers
  // vertex position buffer
  gl.bindBuffer(gl.ARRAY_BUFFER, elements.position.buffer);
  gl.bufferData(gl.ARRAY_BUFFER, elements.position.data, gl.DYNAMIC_DRAW);
  gl.vertexAttribPointer(
    elements.position.location, // Attribute location 
    2,                      // Number of elements per attribute (X, Y)
    gl.FLOAT,               // Data type of the elements
    false,                  // Whether the elements are normalized
    2 * Float32Array.BYTES_PER_ELEMENT, // Size of an individual vertex
    0  // Offset from the beginning of a single vertex to this attribute
  );
  // vertex displacement buffer
  gl.bindBuffer(gl.ARRAY_BUFFER, elements.displacement.buffer);
  gl.bufferData(gl.ARRAY_BUFFER, elements.displacement.data, gl.DYNAMIC_DRAW);
  gl.vertexAttribPointer(
    elements.displacement.location, // Attribute location 
    2,                       // Number of elements per attribute (X, Y)
    gl.FLOAT,                // Data type of the elements
    false,                   // Whether the elements are normalized
    2 * Float32Array.BYTES_PER_ELEMENT,  // Size of an individual vertex
    0  // Offset from the beginning of a single vertex to this attribute
  );
  // vertex stress buffer
  gl.bindBuffer(gl.ARRAY_BUFFER, elements.stress.buffer);
  gl.bufferData(gl.ARRAY_BUFFER, elements.stress.data, gl.DYNAMIC_DRAW);
  gl.vertexAttribPointer(
    elements.stress.location, // Attribute location 
    3,                    // Number of elements per attribute (σ_x, σ_y, τ_xy)
    gl.FLOAT,             // Data type of the elements
    false,                // Whether the elements are normalized
    3 * Float32Array.BYTES_PER_ELEMENT,  // Size of an individual vertex
    0  // Offset from the beginning of a single vertex to this attribute
  );

  return elements;
}

/**
 * @param {WebGL2RenderingContext | WebGLRenderingContext} gl
 */
async function setupNodeProgram(gl) {

  // Get shader source code from files
  const vertexShaderSource = load.text("shaders/nodes.vs.glsl");
  const fragmentShaderSource = load.text("shaders/nodes.fs.glsl");

  const program = load.shaderProgram(
    gl, 
    await vertexShaderSource, 
    await fragmentShaderSource
  );

  gl.useProgram(program);

  const maxVerts= wasm.max_vertices();
  const nodes = {
    program,
    view: {
      center: {
        location: gl.getUniformLocation(program, "view.center")
      },
      range: {
        location: gl.getUniformLocation(program, "view.range")
      }
    },
    scaleDisp: {
      location: gl.getUniformLocation(program, "scaleDisp")
    },
    position: {
      buffer: gl.createBuffer(),
      data: new Float32Array(
        wasm.memory.buffer, 
        wasm.get_node_positions(), 
        2 * maxVerts
      ),
      location: gl.getAttribLocation(program, "vertPosition"),
    },
    displacement: {
      buffer: gl.createBuffer(),
      data: new Float32Array(
        wasm.memory.buffer, 
        wasm.get_node_displacements(), 
        2 * maxVerts
      ),
      location: gl.getAttribLocation(program, "vertDisp"),
    },
  };
  gl.bindBuffer(gl.ARRAY_BUFFER, nodes.position.buffer);
  gl.bufferData(gl.ARRAY_BUFFER, nodes.position.data, gl.DYNAMIC_DRAW);
  gl.vertexAttribPointer(
    nodes.position.location, // Attribute location 
    2,                    // Number of elements per attribute (σ_x, σ_y, τ_xy)
    gl.FLOAT,             // Data type of the elements
    false,                // Whether the elements are normalized
    2 * Float32Array.BYTES_PER_ELEMENT,  // Size of an individual vertex
    0  // Offset from the beginning of a single vertex to this attribute
  );
  gl.bindBuffer(gl.ARRAY_BUFFER, nodes.displacement.buffer);
  gl.bufferData(gl.ARRAY_BUFFER, nodes.displacement.data, gl.DYNAMIC_DRAW);
  gl.vertexAttribPointer(
    nodes.displacement.location, // Attribute location 
    2,                    // Number of elements per attribute (σ_x, σ_y, τ_xy)
    gl.FLOAT,             // Data type of the elements
    false,                // Whether the elements are normalized
    2 * Float32Array.BYTES_PER_ELEMENT,  // Size of an individual vertex
    0  // Offset from the beginning of a single vertex to this attribute
  );
  return nodes;
}

/**
 * @param {() => void} renderAll
 */
function setupFileManagement(renderAll) {

  const newBtn = doc.getElementById("new");
  const open = doc.getElementById("open");
  const save = doc.getElementById("save");

  const elasticity = doc.getInputElementById("elasticity");
  const poissonsRatio = doc.getInputElementById("poissons-ratio");
  const rigidity = doc.getInputElementById("rigidity");
  const nodeIndex = doc.getElementById("node-index");
  const elementIndex = doc.getElementById("element-index");
  
  newBtn.addEventListener("click", (_evt) => {

    const e = parseFloat(elasticity.value);
    const nu = parseFloat(poissonsRatio.value);
    const g = parseFloat(rigidity.value);
    model = init_fea(e, nu, g);

    nodeIndex.dispatchEvent(new Event("change"));
    elementIndex.dispatchEvent(new Event("change"));
    renderAll();
  });

  open.addEventListener("click", (_evt) => {
    const input = document.createElement("input");
    input.type = "file";
    input.setAttribute("accept", ".json,text/json");
    input.addEventListener("change", async (_evt) => {
      if (input.files === null) {
        const message = "File missing!";
        alert(message);
        throw new Error(message);
      }
      const file = input.files[0];
      const modelJson = JSON.parse(await file.text());
      if (modelJson.elasticity) {
        doc.getInputElementById("elasticity").value = modelJson.elasticity;
      } else {
        const message = "File is missing elasticity";
        alert(message);
        throw new Error(message)
      }
      if (modelJson.poissonsRatio) {
        doc.getInputElementById("poissons-ratio").value = modelJson.poissonsRatio;
      } else {
        const message = "File is missing poissonsRatio";
        alert(message);
        throw new Error(message)
      }
      if (modelJson.rigidity) {
        doc.getInputElementById("rigidity").value = modelJson.rigidity;
      } else {
        const message = "File is missing rigidity";
        alert(message);
        throw new Error(message)
      }
      doc.getElementById("new").dispatchEvent(new Event("click"));
      if (modelJson.nodes) {
        const nodeCount = modelJson.nodes.length;
        for (let i = 0; i < nodeCount; i++) {
          const nodeJson = modelJson.nodes[i];
          model.add_node();
          const node = model.get_node(i);
          node.posX = nodeJson.position.x;
          node.posY = nodeJson.position.y;
          node.knownX = 
            (nodeJson.known.x == "displacement")?KnownType.Displacement:KnownType.Force;
          node.knownY = 
            (nodeJson.known.y == "displacement")?KnownType.Displacement:KnownType.Force;
          node.dispX = nodeJson.displacement.x;
          node.dispY = nodeJson.displacement.y;
          node.forceX = nodeJson.force.x;
          node.forceY = nodeJson.force.y;
          model.set_node(i, node);
        }
      } else {
        const message = "File is missing nodes";
        alert(message);
        throw new Error(message)
      }
      if (modelJson.elements) {
        const elementCount = modelJson.elements.length;
        for (let i = 0; i < elementCount; i++) {
          const elementJson = modelJson.elements[i];
          model.add_elem();
          model.set_element_indices(i, elementJson.indices);
        }
      } else {
        const message = "File is missing elements";
        alert(message);
        throw new Error(message)
      }
      renderAll();
    });
    input.click();
  });

  save.addEventListener("click", (_evt) => {
    const elasticity = doc.getInputElementById("elasticity").value;
    const poissonsRatio = doc.getInputElementById("poissons-ratio").value;
    const rigidity = doc.getInputElementById("rigidity").value;
    /** @type {any[]} */
    const nodes = [];
    /** @type {any[]} */
    const elements = [];
    const modelJson = {
      elasticity,
      poissonsRatio,
      rigidity,
      nodes,
      elements,
    };
    const nodeCount = model.nodes_len();
    for (let i = 0; i < nodeCount; i++) {
      const node = model.get_node(i);
      const position = {
        x: node.posX,
        y: node.posY,
      };
      const known = {
        x: (node.knownX == KnownType.Displacement)?"displacement":"force",
        y: (node.knownY == KnownType.Displacement)?"displacement":"force",
      }
      const displacement = {
        x: node.dispX,
        y: node.dispY,
      };
      const force = {
        x: node.forceX,
        y: node.forceY,
      };
      const nodeJson = {
        position,
        known,
        displacement,
        force,
      };
      modelJson.nodes.push(nodeJson);
    }
    const elementCount = model.elements_len();
    for (let i = 0; i < elementCount; i++) {
      const elementIndices = model.get_element_indices(i);
      const elementJson = {
        indices: [elementIndices[0], elementIndices[1], elementIndices[2]],
      };
      modelJson.elements.push(elementJson);
    }

    const file = new Blob([JSON.stringify(modelJson, null, 2)], {type: "text/json"});

    const downloadLink = document.createElement('a');
    downloadLink.href = URL.createObjectURL(file);
    downloadLink.download = "Lin2dFea.json";
    
    document.body.appendChild(downloadLink); 
    downloadLink.click();
    downloadLink.remove();
  });
}

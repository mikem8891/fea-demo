// @ts-check

/**
 * @param {string} url 
 */
export async function text(url) {
  let responce = await fetch(url);
  let status = responce.status;
  if (200 <= status && status < 300){
    return responce.text();
  } else {
    throw new Error(`Failed to load text from: "${url}"`);
  }
}

const addon = require('../native');

/**
 *
 * @callback imageSearchCallback
 * @param {Error} error
 * @param {Any} res
 * @returns {void}
 */
/**
 * 在 parent 图片中, 寻找相似 child 图片的坐标
 * 
 * @param {String} parent_image_path parent image path
 * @param {String} child_image_path child image path
 * @param {Object} options options
 * @param {String} [options.out=''] save as a new image with child image border
 * @param {Number} [options.result_level=1]
 * @param {imageSearchCallback} cb callback function
 */
function image_search(parent_image_path, child_image_path, options, cb) {
  if (typeof parent_image_path !== 'string' || typeof child_image_path !== 'string') {
    throw new Error('The image path must be a string!');
  }
  if (!cb) {
    cb = options;
    options = {};
  }
  if (typeof options != 'object') {
    throw new Error('The options must be a object!');
  }
  if (typeof cb !== 'function') {
    throw new Error('The callback must be a function!');
  }
  if (typeof options.result_level !== 'number' || options.result_level < 1) {
    options.result_level = 1;
  }
  options.out = options.out || '';

  return addon.image_search(parent_image_path, child_image_path, options, cb);
}

/**
 * 
 * @callback getDHashCallback
 * @param {Error} error error
 * @param {String} res  dHash String
 * @returns {void}
 */

/**
 * 获取图片的差异值哈希
 * 
 * @param {String} image_path image path
 * @param {getDHashCallback} cb callback function
 */
function get_d_hash(image_path, cb) {
  if (typeof image_path !== 'string') {
    throw new Error('The image path must be a string!');
  }
  if (typeof cb !== 'function') {
    throw new Error('The callback must be a function!');
  }
  return addon.get_d_hash(image_path, cb);
}

/**
 * 通过两个64位十六进制字符串哈希计算汉明距离
 * 
 * @param {String} hash_1 hex hash
 * @param {String} hash_2 hex hash
 * 
 * @returns {Number}
 */
function get_hamming_distance_by_hex_hash(hash_1, hash_2) {
  if (typeof hash_1 !== 'string' || typeof hash_2 !== 'string') {
    throw new Error('The param must be hex string!');
  }
  return addon.get_hamming_distance_by_hex_hash(hash_1, hash_2);
}

module.exports = {
  image_search,
  get_d_hash,
  get_hamming_distance_by_hex_hash
}

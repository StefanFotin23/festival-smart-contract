module.exports = function override(config, env) {
  // Add fallbacks for Node.js core modules
  config.resolve.fallback = {
    ...config.resolve.fallback,
    "crypto": require.resolve("crypto-browserify"),
    "stream": require.resolve("stream-browserify"),
    "path": require.resolve("path-browserify"),
    "fs": false // fs cannot be polyfilled in the browser
  };

  // Ignore warnings for source-map-loader
  config.ignoreWarnings = [
    /Failed to parse source map/,
  ];

  return config;
}

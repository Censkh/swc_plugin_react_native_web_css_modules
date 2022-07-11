const { main } = require("./package.json");

module.exports.withReactNativeWebCssModules = (nextConfig) => {
  const experimental = nextConfig.experimental || (nextConfig.experimental = {});
  const swcPlugins = experimental.swcPlugins || (experimental.swcPlugins = []);

  swcPlugins.push([require.resolve(main), {}]);

  return nextConfig;
}

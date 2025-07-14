const binding = require('template-binding');
process.env.RSPACK_BINDING = require('node:path').join(
  __dirname,
  '../node_modules/template-binding'
);

binding.registerExamplePlugin();

module.exports = require('@rspack/core');

const zed = require('zed');

function activate(context) {
  console.log('Objective-R extension is now active in Zed!');
}

function deactivate() {}

module.exports = {
  activate,
  deactivate
};

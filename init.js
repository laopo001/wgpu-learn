let fs = require('fs');
let attribute = require('./src/attribute.json')
let uniform = require('./src/uniform.json')

fs.writeFileSync('./src/other.rs', `
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Attrib {
${attribute.map((x, i) => (x.name + '=' + i)).join(',')}
}`);

fs.appendFileSync('./src/other.rs', `
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Uniform {
${uniform.map((x, i) => (x.name + '=' + i)).join(',')}
}`)
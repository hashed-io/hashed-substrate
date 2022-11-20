const { encodeAddress } = require("@polkadot/util-crypto");

const sudoAddress = "0x5cf8957922e4058a953281f82fdced2e4d389fe37c77f41a0fd2379df0caf877";
const coll1Address = "0x1cfc7e49e91696b84bf8e931c16375ea634c3997b36155657faf7dc4716e273e";
const coll2Address = "0x84ce3f0bc9ae73d8497c6161927e9e04f39f4bc54579689532d048188c10a77c";

const hashedSudo = "0x021a78fcc3ec988411388ec2f8ab25fbb79a7eaacead997d13f211ebe34ce359";
const hashedColl1 = "0x90150e105b07c3357d43ed5c727efb9be347699cb2b5a41a26423b559615b222";
const hashedColl2 = "0x028c3a5c8890c3e98023b35f99a5d904b170612b78b6b9fdd8f60cbf24ab9f59";

const hashedPrefix = '9072';
const luhnPrefix = '11486';
const address = hashedSudo;
const maxSS58AddressPrefixesCount = 16383;

function printAddress(label, _address, prefix) {
   
    let address = encodeAddress(_address, prefix);
    console.log(`${label}: ${address}`);
  }

function printKnown(address) {
    printAddress('Luhn Network      ', address, luhnPrefix);
    printAddress('Hashed Network    ', address, hashedPrefix);
    console.log();
}

// Find SS58 Address Prefixes that generate an SS58 Address Format that starts with the letter 'e'
// Optionally add a filter to this function if you only want to return those not listed in the
// the SS58 Registry https://github.com/paritytech/ss58-registry/blob/main/ss58-registry.json
function findSS58AddressPrefixes(address) {
  let foundSS58AddressPrefixes = []; 
  let reservedSS58Formats = [46, 47];

  // 
  for (let prefix = 0; prefix <= maxSS58AddressPrefixesCount; prefix++) {
    if (!reservedSS58Formats.includes(prefix)) {
      let ss58Address = encodeAddress(address, prefix);
      if (ss58Address.charAt(0) == 'h' && ss58Address.charAt(1) == 'a' ) {
      // if (ss58Address.charAt(0) == 'u' && ss58Address.charAt(1) == 'h') {
        console.log(`${prefix}`, ss58Address);
        foundSS58AddressPrefixes.push(prefix);
      }
    }
  }

  return foundSS58AddressPrefixes;
}

let foundSS58AddressPrefixes = findSS58AddressPrefixes(address);
console.log('count: ', foundSS58AddressPrefixes.length);
console.log('foundSS58AddressPrefixes: ', foundSS58AddressPrefixes);

// printKnown (sudoAddress);
// printKnown (coll1Address);
// printKnown (coll2Address);

printAddress('Luhn Network Sudo             :', sudoAddress, luhnPrefix);
printAddress('Luhn Network Collator #1      :', coll1Address, luhnPrefix);
printAddress('Luhn Network Collator #2      :', coll2Address, luhnPrefix);
console.log ();

printAddress('Hashed Network Sudo             :', hashedSudo, hashedPrefix);
printAddress('Hashed Network Collator #1      :', hashedColl1, hashedPrefix);
printAddress('Hashed Network Collator #2      :', hashedColl2, hashedPrefix);

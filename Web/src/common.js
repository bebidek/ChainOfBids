const contract_address = "5Cp6nPR1duyzy2F8H2mt1v5CJnmjtHsAmNwscZWzBuFETJKH";

const { WsProvider, ApiPromise } = polkadotApi;
const { ContractPromise } = polkadotApiContract;

var provider = null;
var api = null;
var contract = null;

async function init_contract() {
    if (provider == null) {
        provider = new WsProvider('wss://ws-smartnet.test.azero.dev');
        api = await ApiPromise.create({ provider });
        contract = new ContractPromise(api, metadata, contract_address);
    }
}

async function init_ext() {
    const { web3FromAddress, web3Enable } = polkadotExtensionDapp;
    await web3Enable('-chain-of-bids-');
    return web3FromAddress;
}

function S_GET(id) { // taken from StackOverflow
    var a = new RegExp(id + "=([^&#=]*)");
    return decodeURIComponent(a.exec(window.location.search)[1]);
}
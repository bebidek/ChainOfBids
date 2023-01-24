async function index_init() {
    await init_contract();

    var { output } = await contract.query.getNumberOfAuctions(0, {});
    const number_of_auctions = output;
    document.getElementById("auction_number").innerText = output;

    var { output } = await contract.query.getFeeDenominator(0, {});
    document.getElementById("auction_fee").innerText = 100 / (output + 20) + "%";
    
    for (var i = 0; i < number_of_auctions; i++) {
        const link = document.createElement("a");
        link.href = "auction.html?auction_id=" + i;
        link.innerText = "Go to auction #" + i;
        document.getElementById("auctions").appendChild(link);
        document.getElementById("auctions").appendChild(document.createElement("br"));
    }
}

async function auction_init() {
    const auction_id = S_GET("auction_id");
    document.getElementById("subtitle").innerText = "Auction #" + auction_id;
    document.getElementById("finalize_link").href = "finalize_auction.html?auction_id=" + auction_id;
    document.getElementById("new_bid_link").href = "new_bid.html?auction_id=" + auction_id;
    
    await init_contract();

    var { output } = await contract.query.getAuctionName(0, {}, auction_id);
    document.getElementById("name").innerText = output.asOk;
    var { output } = await contract.query.getAuctionOwner(0, {}, auction_id);
    document.getElementById("owner").innerText = "Created by " + output.asOk;
    var { output } = await contract.query.getAuctionStartTime(0, {}, auction_id);
    document.getElementById("start_time").innerText = "Starting time: " + new Date(output.asOk.toNumber());
    var { output } = await contract.query.getAuctionEndPeriodStart(0, {}, auction_id);
    const end_period_start = new Date(output.asOk.toNumber());
    var { output } = await contract.query.getAuctionEndPeriodEnd(0, {}, auction_id);
    const end_period_stop = new Date(output.asOk.toNumber());
    document.getElementById("end_period").innerText = "Finalization period: " + end_period_start + " to " + end_period_stop;
    var { output } = await contract.query.getAuctionFinalizationStatus(0, {}, auction_id);
    document.getElementById("finalization_status").innerText = "Finalization status: " + output.asOk;
    var { output } = await contract.query.getAuctionStartingPrice(0, {}, auction_id);
    document.getElementById("starting_price").innerText = "Starting price: " + output.asOk;
    var { output } = await contract.query.getAuctionDescription(0, {}, auction_id);
    document.getElementById("desc").innerText = output.asOk;
    
    var number_of_bids;
    {
        var { output } = await contract.query.getAuctionNumberOfBids(0, {}, auction_id);
        number_of_bids = output.asOk;
        document.getElementById("number_of_bids").innerText = output.asOk;
    }
    
    for (var i = 0; i < number_of_bids; i++) {
        const td1 = document.createElement("td");
        var { output } = await contract.query.getBidBidder(0, {}, auction_id, i);
        td1.innerText = output.asOk;

        const td2 = document.createElement("td");
        var { output } = await contract.query.getBidPrice(0, {}, auction_id, i);
        td2.innerText = output.asOk;

        const td3 = document.createElement("td");
        var { output } = await contract.query.getBidAmount(0, {}, auction_id, i);
        td3.innerText = output.asOk;

        const td4 = document.createElement("td");
        const increase_link = document.createElement("a");
        increase_link.href = "increase_bid.html?auction_id=" + auction_id + "&bid_id=" + i;
        td4.appendChild(increase_link);

        const tr = document.createElement("tr");
        tr.appendChild(td1);
        tr.appendChild(td2);
        tr.appendChild(td3);

        document.getElementById("bids").appendChild(tr);
    }
}

async function auction_finalize_init() {
    const auction_id = S_GET("auction_id");
    document.getElementById("subtitle").innerText = "Finalize auction #" + auction_id;
}

async function new_bid_init() {
    const auction_id = S_GET("auction_id");
    document.getElementById("subtitle").innerText = "Make a bid (auction #" + auction_id + ")";
}

async function increase_bid_init() {
    const auction_id = S_GET("auction_id");
    const bid_id = S_GET("bid_id");
    document.getElementById("subtitle").innerText = "Increase the bid #" + bid_id + "(auction #" + auction_id + ")";
}

async function new_bid() {
    const auction_id = S_GET("auction_id");

    var values = Object.values(document.forms.new_bid_form)
    const price = Number(values[0].value);
    const amount = Number(values[1].value);
    const sender = values[2].value;
    
    await init_contract();
    const web3FromAddress = await init_ext();

    const { gasRequired } = await contract.query.makeABid(0, { value: price }, auction_id, amount);

    const injector = await web3FromAddress(sender)

    const gasLimit = gasRequired * 2;
    const storageDepositLimit = null;
    contract.tx
        .makeABid({ value: price, storageDepositLimit, gasLimit }, auction_id, amount)
        .signAndSend(sender, { signer: injector.signer }, ({status, events}) => {
            if (status.isOk) { 
                alert("OK");
            } else {
                events
                    // find/filter for failed events
                    .filter(({ event }) =>
                        api.events.system.ExtrinsicFailed.is(event)
                    )
                    // we know that data for system.ExtrinsicFailed is
                    // (DispatchError, DispatchInfo)
                    .forEach(({ event: { data: [error, info] } }) => {
                        if (error.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(error.asModule);
                            const { docs, method, section } = decoded;

                            alert("NOT OK");
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            alert("NOT OK");
                        }
                    });
            }
        });
}

async function increase_bid() {
    const auction_id = S_GET("auction_id");
    const bid_id = S_GET("bid_id");

    var values = Object.values(document.forms.increase_bid_form)
    const extra_price = Number(values[0].value);
    const sender = values[1].value;
    
    await init_contract();
    const web3FromAddress = await init_ext();

    const { gasRequired } = await contract.query.increaseTheBid(0, { value: price }, auction_id, bid_id);

    const injector = await web3FromAddress(sender)

    const gasLimit = gasRequired * 2;
    const storageDepositLimit = null;
    contract.tx
        .increaseTheBid({ value: price, storageDepositLimit, gasLimit }, auction_id, bid_id)
        .signAndSend(sender, { signer: injector.signer }, ({status, events}) => {
            if (status.isOk) { 
                alert("OK");
            } else {
                events
                    // find/filter for failed events
                    .filter(({ event }) =>
                        api.events.system.ExtrinsicFailed.is(event)
                    )
                    // we know that data for system.ExtrinsicFailed is
                    // (DispatchError, DispatchInfo)
                    .forEach(({ event: { data: [error, info] } }) => {
                        if (error.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(error.asModule);
                            const { docs, method, section } = decoded;

                            alert("NOT OK");
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            alert("NOT OK");
                        }
                    });
            }
        });
}

async function finalize_auction() {
    const auction_id = S_GET("auction_id");

    var values = Object.values(document.forms.finalize_auction_form)
    const sender = values[0].value;
    
    await init_contract();
    const web3FromAddress = await init_ext();

    const { gasRequired } = await contract.query.finalizeAuction(0, {}, auction_id);

    const injector = await web3FromAddress(sender)

    const gasLimit = gasRequired * 2;
    const storageDepositLimit = null;
    contract.tx
        .finalizeAuction({ storageDepositLimit, gasLimit }, auction_id)
        .signAndSend(sender, { signer: injector.signer }, ({status, events}) => {
            if (status.isOk) { 
                alert("OK");
            } else {
                events
                    // find/filter for failed events
                    .filter(({ event }) =>
                        api.events.system.ExtrinsicFailed.is(event)
                    )
                    // we know that data for system.ExtrinsicFailed is
                    // (DispatchError, DispatchInfo)
                    .forEach(({ event: { data: [error, info] } }) => {
                        if (error.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(error.asModule);
                            const { docs, method, section } = decoded;

                            alert("NOT OK");
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            alert("NOT OK");
                        }
                    });
            }
        });
}

async function new_auction() {
    const status_field = document.getElementById("status");

    var values = Object.values(document.forms.new_auction_form)
    const name = values[0].value;
    const description = values[1].value;
    const amount = values[2].value;
    const start_time = Date.parse(values[3].value);
    const finish_from = Date.parse(values[4].value);
    const finish_to = Date.parse(values[5].value);
    const starting_price = values[6].value;
    const sender = values[7].value;
    
    await init_contract();
    const web3FromAddress = await init_ext();
    
    const { gasRequired } = await contract.query.createNewAuction(0, {}, name, description, amount, start_time, finish_from, finish_to, starting_price);

    const injector = await web3FromAddress(sender)

    const gasLimit = gasRequired * 2;
    const storageDepositLimit = null;
    contract.tx
        .createNewAuction({ storageDepositLimit, gasLimit }, name, description, amount, start_time, finish_from, finish_to, starting_price)
        .signAndSend(sender, { signer: injector.signer }, ({status, events}) => {
            if (status.isOk) { 
                status_field.innerText = JSON.stringify(status.asOk);
            } else {
                events
                    // find/filter for failed events
                    .filter(({ event }) =>
                        api.events.system.ExtrinsicFailed.is(event)
                    )
                    // we know that data for system.ExtrinsicFailed is
                    // (DispatchError, DispatchInfo)
                    .forEach(({ event: { data: [error, info] } }) => {
                        if (error.isModule) {
                            // for module errors, we have the section indexed, lookup
                            const decoded = api.registry.findMetaError(error.asModule);
                            const { docs, method, section } = decoded;

                            status_field.innerText = "ERROR: " + (`${section}.${method}: ${docs.join(' ')}`);
                        } else {
                            // Other, CannotLookup, BadOrigin, no extra info
                            status_field.innerText = "ERROR: " + (error.toString());
                        }
                    });
            }
        });
}
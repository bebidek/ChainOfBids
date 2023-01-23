async function index_init() {
    await init_contract();

    const { output } = await contract.query.getNumberOfAuctions(0, {});
    document.getElementById("auction_number").innerText = output;
    
    for (var i = 0; i < output; i++) {
        const link = document.createElement("a");
        link.href = "auction.html?id=" + i;
        link.innerText = "Go to auction #" + i;
        document.getElementById("auctions").appendChild(link);
        document.getElementById("auctions").appendChild(document.createElement("br"));
    }
}

async function auction_init() {
    const auction_id = S_GET("id");
    document.getElementById("subtitle").innerText = "Auction #" + auction_id;
    document.getElementById("finalize_link").href = "finalize_auction.html?id=" + auction_id;
    document.getElementById("new_bid_link").href = "new_bid.html?id=" + auction_id;
    
    await init_contract();

    var { output } = await contract.query.getAuctionName(0, {}, auction_id);
    document.getElementById("name").innerText = output.asOk;
    var { output } = await contract.query.getAuctionOwner(0, {}, auction_id);
    document.getElementById("owner").innerText = "Created by " + output.asOk;
    var { output } = await contract.query.getAuctionEndPeriodStart(0, {}, auction_id);
    const end_period_start = new Date(output.asOk.toNumber());
    var { output } = await contract.query.getAuctionEndPeriodEnd(0, {}, auction_id);
    const end_period_stop = new Date(output.asOk.toNumber());
    document.getElementById("end_period").innerText = "Finalization period: " + end_period_start + " to " + end_period_stop;
    var { output } = await contract.query.getAuctionFinalizationStatus(0, {}, auction_id);
    document.getElementById("finalization_status").innerText = "Finalization status: " + output.asOk;
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

        const tr = document.createElement("tr");
        tr.appendChild(td1);
        tr.appendChild(td2);

        document.getElementById("bids").appendChild(tr);
    }
}

async function auction_finalize_init() {
    const auction_id = S_GET("id");
    document.getElementById("subtitle").innerText = "Finalize auction #" + auction_id;
}

async function new_bid_init() {
    const auction_id = S_GET("id");
    document.getElementById("subtitle").innerText = "Make a bid (auction #" + auction_id + ")";
}

async function new_bid() {
    const auction_id = S_GET("id");

    var values = Object.values(document.forms.new_bid_form)
    const price = Number(values[0].value);
    const sender = values[1].value;
    
    await init_contract();
    const web3FromAddress = await init_ext();

    const { gasRequired } = await contract.query.makeABid(0, { value: price }, auction_id);

    const injector = await web3FromAddress(sender)

    const gasLimit = gasRequired * 2;
    const storageDepositLimit = null;
    contract.tx
        .makeABid({ value: price, storageDepositLimit, gasLimit }, auction_id)
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
    const auction_id = S_GET("id");

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
    const finish_from = Date.parse(values[2].value);
    const finish_to = Date.parse(values[3].value);
    const sender = values[4].value;
    
    await init_contract();
    const web3FromAddress = await init_ext();
    
    const { gasRequired } = await contract.query.createNewAuction(0, {}, name, description, finish_from, finish_to);

    const injector = await web3FromAddress(sender)

    const gasLimit = gasRequired * 2;
    const storageDepositLimit = null;
    contract.tx
        .createNewAuction({ storageDepositLimit, gasLimit }, name, description, finish_from, finish_to)
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
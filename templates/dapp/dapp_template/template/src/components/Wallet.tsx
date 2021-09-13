import PWCore, { EthProvider, Provider, SUDT } from "@lay2/pw-core";
import detectEthereumProvider from "@metamask/detect-provider";
import React, { Component, useEffect, useState } from "react";
import BasicCollector from "../collectors/BasicCollector";
import { CHAIN_SPECS } from "../helpers/constants";
import Balance from "./Balance";

interface PwObject {
    collector: BasicCollector,
    pwCore: PWCore,
    provider: Provider,
}

async function initPwCore() {
    const provider = new EthProvider();
    const collector = new BasicCollector(`http://localhost:8116`);
    const pwCore = await new PWCore(`http://localhost:8114`).init(provider, collector, undefined, CHAIN_SPECS.dev);
    return { pwCore, provider, collector };
}

export default function Wallet() {
    // Handle loading
    const [busy, setBusy] = useState(true);
    const [loading, setLoading] = useState(true);
    // Wallet data
    const [ckbBalance, setCkbBalance] = useState("0");
    const [sudtBalance, setSudtBalance] = useState("0");
    // Pw Core
    const [pw, setPw] = useState<PwObject | null>(null)
    // Load PwCore
    useEffect(() => {
        detectEthereumProvider()
            .then((provider) => {
                if (provider) {
                    initPwCore()
                        .then((pwValues) => {
                            setBusy(true);
                            setLoading(true);
                            setPw(pwValues);
                        });
                }
                else {
                    alert('A MetAMask compatible browser extension was not detected.')
                }
            })
    }, [setPw]);

    // Update Balances
    useEffect(() => {

        if (pw != null) {
            const address = pw.provider.address
            const sudt = new SUDT(address.toLockScript().toHash())
            // set ckb balance
            pw.collector.getBalance(address)
                .then((res) => {
                    setCkbBalance(res.toString())
                })

            // set self issued sudt balance
            pw.collector.getSUDTBalance(sudt, address)
                .then((res) => {
                    setSudtBalance(res.toString())
                })
        }
    }, [pw]);





    // CkbWallet
    if (pw) {
        return (
            <div>
                <h2>Your Wallet</h2>
                <Balance
                    ticker={"CKB"}
                    name={"Nervos"}
                    amount={ckbBalance}
                />
                <Balance
                    ticker={"SUDT"}
                    name={"Self-issued SUDT"}
                    amount={sudtBalance}
                />

            </div>
        )
    } else {
        return (
            <div>
                Wallet loading...
            </div>
        )
    }

}
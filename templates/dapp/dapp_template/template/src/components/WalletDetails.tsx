import PWCore, { Address, Amount, Provider } from "@lay2/pw-core";
import React, { Component, useEffect, useState } from "react";
import BasicCollector from "../collectors/BasicCollector";



const WalletDetails = (props: { provider: Provider | undefined, collector: BasicCollector | undefined }) => {

    const [addr, setAddr] = useState('');
    const [lockhash, setLockhash] = useState('');
    const [balance, setBalance] = useState('');

    useEffect(() => {
        if (props.provider && props.collector) {
            const address = props.provider.address
            setAddr(address.toCKBAddress().toString())
            setLockhash(props.provider.address.toLockScript().toHash())

            props.collector.getBalance(address).then((balance) => {
                console.log(balance.toString())
                setBalance(balance.toString())
            })

        }
    })

    return (
        <div>
            <h2>Your Wallet</h2>
            <p>CKB Address: {addr}</p>
            <p>Lockhash: {lockhash}</p>
            <p>CKB Balance: </p>
            <p>{balance}</p>
        </div>
    )
}

export default WalletDetails;
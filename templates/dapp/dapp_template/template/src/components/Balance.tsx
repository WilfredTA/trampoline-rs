import React from "react";

interface BalanceData {
    ticker: string,
    name: string,
    amount: string
}

export default function Balance(props: BalanceData) {

    // render
    return (
        <div>
            <h3>{props.ticker} | {props.name}</h3>
            <p>{props.amount}</p>
        </div>
    )
}
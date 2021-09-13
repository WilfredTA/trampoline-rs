import { Address, AddressType, CellDep, DepType, HashType, OutPoint, Script } from "@lay2/pw-core";
import pwConfig from '../pwConfig.json'

export const ECDSA_WITNESS_LEN = 172;
export const DAO_WITHDRAW_2_WITNESS_LEN = 196;

export const TEST_ADDRESSES = {
    dev: {
        address: new Address("ckt1qyqyc7038eas4nxlr2hdmfxgy4zp3uez6f3slf689m", AddressType.ckb),
    }
}

const daoType = {
    cellDep: new CellDep(
        DepType.code,
        new OutPoint(
            pwConfig.dev.daoType.cellDep.outPoint.txHash,
            pwConfig.dev.daoType.cellDep.outPoint.txHash
        )
    ),
    script: new Script(
        pwConfig.dev.daoType.script.codeHash,
        pwConfig.dev.daoType.script.args,
        HashType.type
    )
}

const sudtType = {
    cellDep: new CellDep(
        DepType.code,
        new OutPoint(
            pwConfig.dev.sudtType.cellDep.outPoint.txHash,
            pwConfig.dev.sudtType.cellDep.outPoint.txHash
        )
    ),
    script: new Script(
        pwConfig.dev.sudtType.script.codeHash,
        pwConfig.dev.sudtType.script.args,
        HashType.type
    )
}

const defaultLock = {
    cellDep: new CellDep(
        DepType.code,
        new OutPoint(
            pwConfig.dev.defaultLock.cellDep.outPoint.txHash,
            pwConfig.dev.defaultLock.cellDep.outPoint.txHash
        )
    ),
    script: new Script(
        pwConfig.dev.defaultLock.script.codeHash,
        pwConfig.dev.defaultLock.script.args,
        HashType.type
    )
}

const multiSigLock = {
    cellDep: new CellDep(
        DepType.code,
        new OutPoint(
            pwConfig.dev.multiSigLock.cellDep.outPoint.txHash,
            pwConfig.dev.multiSigLock.cellDep.outPoint.txHash
        )
    ),
    script: new Script(
        pwConfig.dev.multiSigLock.script.codeHash,
        pwConfig.dev.multiSigLock.script.args,
        HashType.type
    )
}

const pwLock = {
    cellDep: new CellDep(
        DepType.code,
        new OutPoint(
            pwConfig.dev.pwLock.cellDep.outPoint.txHash,
            pwConfig.dev.pwLock.cellDep.outPoint.txHash
        )
    ),
    script: new Script(
        pwConfig.dev.pwLock.script.codeHash,
        pwConfig.dev.pwLock.script.args,
        HashType.type
    )
}

const acpLockList = [
    new Script(
        '0xbf43c3602455798c1a61a596e0d95278864c552fafe231c063b3fabf97a8febc',
        '0x',
        HashType.type
    ),
    new Script(
        '0x0fb343953ee78c9986b091defb6252154e0bb51044fd2879fde5b27314506111',
        '0x',
        HashType.data
    )
]


export const CHAIN_SPECS = {
    dev: {
        daoType: daoType,
        sudtType: sudtType,
        defaultLock: defaultLock,
        multiSigLock: multiSigLock,
        pwLock: pwLock,
        acpLockList: acpLockList
    }
}






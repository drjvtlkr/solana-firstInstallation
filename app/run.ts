import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Poi} from "../target/types/poi"
import {Keypair, Connection} from "@solana/web3.js"
import fs from "fs"
(
    async()=>{
        console.log("HI")
        const connection = new Connection("http://localhost:8899")
        // const secret = JSON.parse(fs.readFileSync("/home/batman/.config/solana/id.json").)
    }
)
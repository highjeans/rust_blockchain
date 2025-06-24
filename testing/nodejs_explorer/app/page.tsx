"use client";

import { use, useEffect, useState } from "react";

const axios = require("axios");

const NODE_URL = "http://127.0.0.1:8000";

async function getLatestBlock(): Promise<Block> {
  const response = await axios.get(`${NODE_URL}/frontier_block`);
  return response.data;
}

async function getPreviousBlock(prevHash: string): Promise<Block> {
  const response = await axios.get(`${NODE_URL}/block/${prevHash}`);
  return response.data;
}

async function fetchLast10Blocks() {
  let blocks = [await getLatestBlock()];
  while (blocks.length < 10) {
    if (
      blocks[blocks.length - 1].previous ===
      "0000000000000000000000000000000000000000000000000000000000000000"
    )
      break;
    blocks.push(await getPreviousBlock(blocks[blocks.length - 1].previous));
  }
  console.log(blocks);
  return blocks;
}

type Block = {
  index: number;
  timestamp: number;
  data: string;
  previous: string;
  nonce: number;
  hash: string;
  diff_bits: number;
  acc_diff: number;
};

export default function Home() {
  const blocks = use(fetchLast10Blocks());

  return (
    <div className="p-8 text-white">
      <div className="text-xl">Rust Blockchain</div>
      <div className="text-3xl">
        <b>Block Explorer</b>
      </div>

      {blocks.map((block) => {
        return (
          <div
            className="rounded-lg shadow-lg bg-slate-300 my-4 p-4 text-black"
            key={block.hash}
          >
            <div>
              <div className="text-lg">
                <b>#{block.index}</b> - <u>{block.hash}</u>
              </div>
              <div className="text-md">
                {new Date(block.timestamp * 1000).toUTCString()}
              </div>
              <table className="text-left mt-4">
                <thead>
                  <tr>
                    <th className="w-50">Accumulated work</th>
                    <th className="w-50">Difficulty (bits)</th>
                    <th className="w-50">Nonce</th>
                    <th className="w-50">Timestamp</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>{block.acc_diff}</td>
                    <td>{block.diff_bits}</td>
                    <td>{block.nonce}</td>
                    <td>{block.timestamp}</td>
                  </tr>
                </tbody>
              </table>
              <div className="mt-4">
                <b>Data</b>
                <div className="mt-2 bg-white rounded-md p-2 text-md">
                  {block.data}
                </div>
              </div>
              <div className="text-sm mt-4">
                Previous block hash: <u>{block.previous}</u>
              </div>
            </div>
          </div>
        );
      })}
    </div>
  );
}

import { AccountEntry } from './types';

const dummyData: AccountEntry[] = [
  {
    account: 'B62qUCzCdJzVjQYgBcuzSDbiQkSidXc1axn6TGvRFzhCR1bSeYfkUnD',
    votes: [
      { memo: 'magenta', height: 92425, status: 'Canonical' },
      { memo: 'no magenta', height: 24534, status: 'Pending' },
      { memo: 'no magenta', height: 86563, status: 'Canonical' },
      { memo: 'no magenta', height: 23453, status: 'Pending' },
      { memo: 'magenta', height: 45677, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qiksY5ruYvgRaKkj47AhAA77ypiinXwgxdcbknnaEk1vTDygqUKU',
    votes: [
      { memo: 'magenta', height: 12421, status: 'Pending' },
      { memo: 'no magenta', height: 18148, status: 'Canonical' },
      { memo: 'no magenta', height: 53850, status: 'Canonical' },
      { memo: 'no magenta', height: 85030, status: 'Canonical' },
      { memo: 'magenta', height: 21247, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qnRkPAoX5ruTEcRYPXZgynj7z6eVRcpozaLm4ixXhYpEbZc4jW5S',
    votes: [
      { memo: 'no magenta', height: 98213, status: 'Pending' },
      { memo: 'magenta', height: 14279, status: 'Pending' },
      { memo: 'magenta', height: 72353, status: 'Pending' },
      { memo: 'magenta', height: 23838, status: 'Canonical' },
      { memo: 'no magenta', height: 35948, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qnKyRWWN95krAkj4mL3NEXEPQ5h6JktDovA9g5bvDRPyxQ9ngAoE',
    votes: [
      { memo: 'magenta', height: 24575, status: 'Canonical' },
      { memo: 'no magenta', height: 19865, status: 'Pending' },
      { memo: 'magenta', height: 65278, status: 'Pending' },
      { memo: 'no magenta', height: 34654, status: 'Canonical' },
      { memo: 'magenta', height: 96584, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5GRP3xQ',
    votes: [
      { memo: 'no magenta', height: 45686, status: 'Canonical' },
      { memo: 'no magenta', height: 45685, status: 'Canonical' },
      { memo: 'magenta', height: 34564, status: 'Canonical' },
      { memo: 'magenta', height: 34564, status: 'Canonical' },
      { memo: 'no magenta', height: 45836, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5Gkj4mL',
    votes: [
      { memo: 'no magenta', height: 23463, status: 'Pending' },
      { memo: 'no magenta', height: 23452, status: 'Canonical' },
      { memo: 'no magenta', height: 17134, status: 'Canonical' },
      { memo: 'no magenta', height: 13473, status: 'Pending' },
      { memo: 'no magenta', height: 43531, status: 'Pending' },
    ],
  },
  {
    account: 'B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5G8dR5t',
    votes: [
      { memo: 'magenta', height: 31435, status: 'Canonical' },
      { memo: 'magenta', height: 13465, status: 'Pending' },
      { memo: 'magenta', height: 13733, status: 'Pending' },
      { memo: 'magenta', height: 31478, status: 'Canonical' },
      { memo: 'magenta', height: 12363, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5Gq2Wpx',
    votes: [
      { memo: 'no magenta', height: 23455, status: 'Pending' },
      { memo: 'magenta', height: 23452, status: 'Pending' },
      { memo: 'magenta', height: 87634, status: 'Pending' },
      { memo: 'no magenta', height: 34512, status: 'Pending' },
      { memo: 'magenta', height: 48529, status: 'Pending' },
    ],
  },
  {
    account: 'B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5G4RvtA',
    votes: [
      { memo: 'magenta', height: 23468, status: 'Canonical' },
      { memo: 'no magenta', height: 86375, status: 'Canonical' },
      { memo: 'no magenta', height: 23486, status: 'Canonical' },
      { memo: 'magenta', height: 92425, status: 'Canonical' },
      { memo: 'magenta', height: 23858, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5G4RvtA',
    votes: [
      { memo: 'no magenta', height: 11789, status: 'Canonical' },
      { memo: 'no magenta', height: 34564, status: 'Pending' },
      { memo: 'magenta', height: 24970, status: 'Pending' },
      { memo: 'magenta', height: 17454, status: 'Pending' },
      { memo: 'no magenta', height: 93935, status: 'Canonical' },
    ],
  },
  {
    account: 'B62qkRodi7nj6W1geB12UuW2XAx2yidWZCcDthJvkf9G4A6G5G4RvtA',
    votes: [
      { memo: 'magenta', height: 67897, status: 'Canonical' },
      { memo: 'magenta', height: 34745, status: 'Canonical' },
      { memo: 'magenta', height: 18282, status: 'Canonical' },
      { memo: 'magenta', height: 34585, status: 'Canonical' },
      { memo: 'no magenta', height: 23453, status: 'Pending' },
    ],
  },
];

export default dummyData;

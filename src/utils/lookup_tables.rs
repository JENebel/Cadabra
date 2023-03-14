pub const LOOKUP_RANK: [usize; 64] =
[
    7, 7, 7, 7, 7, 7, 7, 7,
    6, 6, 6, 6, 6, 6, 6, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
	0, 0, 0, 0, 0, 0, 0, 0,
];

pub const LOOKUP_FILE: [usize; 64] =
[
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
    0, 1, 2, 3, 4, 5, 6, 7,
	0, 1, 2, 3, 4, 5, 6, 7,
];

pub const LOOKUP_D1: [usize; 64] =
[
    0, 0, 0, 0, 0, 0, 0, 0,
    1, 1, 1, 1, 1, 1, 1, 0,
    2, 2, 2, 2, 2, 2, 1, 0,
    3, 3, 3, 3, 3, 2, 1, 0,
    4, 4, 4, 4, 3, 2, 1, 0,
    5, 5, 5, 4, 3, 2, 1, 0,
    6, 6, 5, 4, 3, 2, 1, 0,
	7, 6, 5, 4, 3, 2, 1, 0,
];

pub const LOOKUP_D2: [usize; 64] =
[
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 1, 1, 1, 1, 1, 1,
    0, 1, 2, 2, 2, 2, 2, 2,
    0, 1, 2, 3, 3, 3, 3, 3,
    0, 1, 2, 3, 4, 4, 4, 4,
    0, 1, 2, 3, 4, 5, 5, 5,
    0, 1, 2, 3, 4, 5, 6, 6,
	0, 1, 2, 3, 4, 5, 6, 7,
];
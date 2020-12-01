let hello = () => {
  Js.log("Advent 3:");
};
type counter = {
  overlapped: bool,
  count: int,
  id: int,
};

let claims = A3Data.claims;

let area = Array.make_matrix(1024, 1024, {overlapped: false, count: 0, id: 0});

let testClaim: A3Data.claim = {id: 0, x: 3, y: 2, w: 4, h: 5};

let testArea = Array.make_matrix(10, 10, {overlapped: false, count: 0, id: 0});

let setCell = (a: array(counter), t: int, id: int) => {
  a[t].count >= 1 ? a[t] = {overlapped: true, count: a[t].count + 1, id} : a[t] = {overlapped: false, count: a[t].count + 1, id};
};

let setRow = (c: A3Data.claim, a: array(counter)) => {
  for (i in c.x to c.x + c.w - 1) {
    setCell(a, i, c.id);
  };
};

let rec setClaimOnArea = (c: A3Data.claim, a: array(array(counter)), r: int) =>
  if (r < Array.length(a)) {
    r >= c.y && r < c.y + c.h ? setRow(c, a[r]) : ();
    setClaimOnArea(c, a, r + 1);
  };

let rec setAllClaims = (c: array(A3Data.claim), a: array(array(counter)), i: int) =>
  if (i < Array.length(c)) {
    setClaimOnArea(c[i], a, 0);
    setAllClaims(c, a, i + 1);
  };

let overlapCount = [|0|];

let nonOverlappingClaims = [|0|];

let incrementCount = () => {
  overlapCount[0] = overlapCount[0] + 1;
};

let findOverlaps = () => {
  for (i in 0 to 1023) {
    for (j in 0 to 1023) {
      if (area[i][j].count > 1) {
        incrementCount();
      };
    };
  };
  Js.log("Number of squares with overlaps:");
  Js.log(overlapCount[0]);
};

let claimCellCount = [|0|];
let rec checkClaimsForOverlap = (c: array(A3Data.claim), a: array(array(counter)), i: int) =>
  if (i < Array.length(c)) {
    claimCellCount[0] = 0;
    let claimArea = c[i].w * c[i].h;
    for (j in c[i].y to c[i].y + c[i].h) {
      for (k in c[i].x to c[i].x + c[i].w) {
        if (a[j][k].id == c[i].id && a[j][k].overlapped == false) {
          claimCellCount[0] = claimCellCount[0] + 1;
        };
      };
    };

    if (claimCellCount[0] == claimArea) {
      Js.log(c[i].id);
    };

    checkClaimsForOverlap(c, a, i + 1);
  };

let () = {
  setAllClaims(claims, area, 0);
  checkClaimsForOverlap(claims, area, 0);
};
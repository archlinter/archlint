export const deep = (x: any) => {
  if (x) {
    if (x > 0) {
      for (let i = 0; i < x; i++) {
        if (i % 2 === 0) {
          while (true) {
            console.log(i);
            break;
          }
        }
      }
    }
  }
};
// depth = 5

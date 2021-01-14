import { getMap, setCoords } from "../main";

  describe("getMap", () => {
    it('gets the board state', () => {
       const viewResult = getMap();
       expect(viewResult.length).toBe(100); // board is 10 by 10
    })

  describe("setCoords", () => {
    it("modifies the board state", () => {
      
       setCoords("0,0", "111111")
       const viewResult = getMap();
       //you can send a log to the console by invoking the log() method 
       //log(viewResult[0]);
       expect(viewResult.length).toBe(100); 
       // entry 0,0 should be 111111!
       expect(viewResult[0]).toBe("111111");
    });
  });
});
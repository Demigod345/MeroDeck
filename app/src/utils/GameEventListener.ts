export default class GameEventListener {
  private ws: WebSocket;
  private events: { [key: string]: (data: any) => void };

  constructor(nodeUrl: string, applicationId: string) {
    this.ws = new WebSocket(`${nodeUrl}/ws`);
    this.ws.addEventListener("open", () => {
    //   console.log('here')
      const request = {
        id: this.getRandomRequestId(),
        method: "subscribe",
        params: {
          applicationIds: [applicationId],
        },
      };
      this.ws.send(JSON.stringify(request));
    });

    this.events = {};
    this.ws.addEventListener("message", (event: MessageEvent) => {
      this.parseMessage(event.data)
    });
  }

  on(event: string, func: (data: any) => void): void {
    this.events[event] = func;
  }

  private parseMessage(msg: string): void {
    // console.log('Executing this');
    const event = JSON.parse(msg);
    const events = event.result?.data?.events;
    if (events) {
      events.forEach((e: any) => {
        if (e.kind in this.events) {
          let bytes = new Int8Array(e.data);
          let str = new TextDecoder().decode(bytes);
          this.events[e.kind](JSON.parse(str))
        }
      })
    }
  }

  private getRandomRequestId(): number {
    return Math.floor(Math.random() * Math.pow(2, 32));
  };
}

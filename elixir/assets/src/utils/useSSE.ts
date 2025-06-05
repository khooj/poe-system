import { useEffect, useState } from 'react';
import { fetchEventSource } from '@microsoft/fetch-event-source';

export type MsgHandler = (s: string) => void;

export default function useSSE(url: string, opts = {}) {
  const [subs, setSubs] = useState<Array<MsgHandler>>([]);
  const [initConnect, setInitConnect] = useState(false);
  const [abortController, setAbortController] = useState<AbortController | null>(null);

  useEffect(() => {
    if (subs.length === 0) {
      setInitConnect(false);
    } else {
      setInitConnect(true);
    }
  }, [subs, setInitConnect]);

  const disconnect = () => {
    abortController?.abort();
    setAbortController(null);
  };

  useEffect(() => {
    return disconnect;
  }, []);

  useEffect(() => {
    if (initConnect) {
      const ctrl = new AbortController();
      f(ctrl.signal);
      setAbortController(ctrl);
    } else {
      disconnect();
    }
  }, [initConnect]);

  const f = async (signal: AbortSignal) => {
    await fetchEventSource(url, {
      onmessage(msg) {
        subs.forEach(s => s(msg.data));
      },
      signal,
      ...opts
    })
  };

  return {
    subscribe(cb: MsgHandler) {
      setSubs(s => [...s, cb]);
      return () => {
        setSubs(s => s.filter(c => c !== cb));
      };
    }
  }
}

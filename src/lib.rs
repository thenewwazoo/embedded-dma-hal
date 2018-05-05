#![no_std]
#![allow(dead_code)]

use core::marker::PhantomData;
use core::ops;

#[derive(Debug)]
pub enum Error {
    Overrun,
    BufferError,
    #[doc(hidden)]
    _Extensible,
}

pub enum Event {
    HalfTransfer,
    TransferComplete,
    TransferError,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Half {
    First,
    Second,
}

pub struct CircBuffer<BUFFER, CHANNEL>
where
    BUFFER: 'static,
{
    buffer: &'static mut [BUFFER; 2],
    channel: CHANNEL,
    readable_half: Half,
    consumed_offset: usize,
}

impl<BUFFER, CHANNEL> CircBuffer<BUFFER, CHANNEL> {
    pub(crate) fn new(buf: &'static mut [BUFFER; 2], chan: CHANNEL) -> Self {
        CircBuffer {
            buffer: buf,
            channel: chan,
            readable_half: Half::Second,
            consumed_offset: 0,
        }
    }
}

pub trait Static<B> {
    fn borrow(&self) -> &B;
}

impl<B> Static<B> for &'static B {
    fn borrow(&self) -> &B {
        *self
    }
}

impl<B> Static<B> for &'static mut B {
    fn borrow(&self) -> &B {
        *self
    }
}

pub struct Transfer<MODE, BUFFER, CHANNEL, PAYLOAD> {
    _mode: PhantomData<MODE>,
    buffer: BUFFER,
    channel: CHANNEL,
    payload: PAYLOAD,
}

impl<BUFFER, CHANNEL, PAYLOAD> Transfer<R, BUFFER, CHANNEL, PAYLOAD> {
    pub(crate) fn r(buffer: BUFFER, channel: CHANNEL, payload: PAYLOAD) -> Self {
        Transfer {
            _mode: PhantomData,
            buffer,
            channel,
            payload,
        }
    }
}

impl<BUFFER, CHANNEL, PAYLOAD> Transfer<W, BUFFER, CHANNEL, PAYLOAD> {
    pub(crate) fn w(buffer: BUFFER, channel: CHANNEL, payload: PAYLOAD) -> Self {
        Transfer {
            _mode: PhantomData,
            buffer,
            channel,
            payload,
        }
    }
}

impl<BUFFER, CHANNEL, PAYLOAD> ops::Deref for Transfer<R, BUFFER, CHANNEL, PAYLOAD> {
    type Target = BUFFER;

    fn deref(&self) -> &BUFFER {
        &self.buffer
    }
}

/// Read transfer
pub struct R;

/// Write transfer
pub struct W;

/// DMA Channel
pub trait DmaChannel {
    type ISR;
    type IFCR;
    type CCRX;
    type CNDTRX;
    type CPARX;
    type CMARX;

    fn listen(&mut self, event: Event);
    fn unlisten(&mut self, event: Event);
    fn isr(&self) -> Self::ISR;
    fn ifcr(&self) -> &Self::IFCR;
    fn ccr(&mut self) -> &Self::CCRX;
    fn cndtr(&mut self) -> &Self::CNDTRX;
    fn cpar(&mut self) -> &Self::CPARX;
    fn cmar(&mut self) -> &Self::CMARX;
    fn get_cndtr(&self) -> u32;
    fn set_req_map(&self, bits: u8);
}

#![forbid(unsafe_code)]

use async_std::task::sleep;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use core::time::Duration;
use pasts::{Polling, Loop};

///////////////////////////////////
//// Implement Interval Future ////
///////////////////////////////////

struct Interval(Duration, Pin<Box<dyn Future<Output = ()>>>);

impl Interval {
    fn new(duration: Duration) -> Self {
        Interval(duration, Box::pin(sleep(duration)))
    }
}

impl Future for Interval {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        match self.1.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => {
                self.1 = Box::pin(sleep(self.0));
                Poll::Ready(())
            }
        }
    }
}

///////////////////////
//// Pasts Example ////
///////////////////////

// Exit type for State.
type Exit = ();

// Shared state between tasks on the thread.
struct State {
    begin: bool,
    counter: usize,
    one: Interval,
    two: Interval,
    list: [Interval; 2],
}

impl State {
    fn one(&mut self, _: ()) -> Poll<Exit> {
        println!("One {}", self.counter);
        self.counter += 1;
        if self.counter > 6 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }

    fn two(&mut self, _: ()) -> Poll<Exit> {
        println!("Two {}", self.counter);
        self.counter += 1;
        Poll::Pending
    }
    
    fn dalist(&mut self, (id, ()): (usize, ())) -> Poll<Exit> {
        if self.begin {
            println!("Hi from {}!", id);
            if id == 1 {
                self.begin = false;
            }
        }
        Poll::Pending
    }

    fn event_loop(&mut self, exec: Loop<Self, Exit>) -> impl Future<Output = Poll<Exit>> {
        exec.when(&mut self.one, State::one)
            .when(&mut self.two, State::two)
            .poll(&mut self.list, State::dalist)
    }
}

async fn run() {
    let mut state = State {
        begin: true,
        counter: 0,
        one: Interval::new(Duration::from_secs_f64(1.0)),
        two: Interval::new(Duration::from_secs_f64(2.0)),
        list: [
            Interval::new(Duration::from_secs_f64(0.1)),
            Interval::new(Duration::from_secs_f64(0.55)),
        ],
    };

    pasts::event_loop(&mut state, State::event_loop).await;
}

fn main() {
    pasts::block_on(run())
}

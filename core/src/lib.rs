mod bindings;

pub mod aligned_state_dynamic_timeline;
pub mod backend;
pub mod boundaries;
pub mod event_predicate;
pub mod event_timeline;
pub mod state_dynamic_timeline_point;
pub mod state_dynamics_timeline;
pub mod timeline;
pub mod timeline_op;
pub mod worker_timeline;
pub mod worker_timeline_data;
pub mod zip_result;

use crate::bindings::exports::timeline::core::api::Guest;
use crate::bindings::exports::timeline::core::api::*;
use crate::timeline_op::{TimeLineOp as CoreTimeLineOp, TimeLineOp};
use crate::bindings::timeline::raw_events_stub::stub_raw_events;
use crate::bindings::golem::rpc::types::Uri;



struct Component;

impl Guest for Component {
    fn initialize_timeline(timeline: TimelineOp) {
       let timeline: CoreTimeLineOp = timeline.into();

       let _ = match timeline {
           TimeLineOp::Leaf(server) => {
               let template_id = server.template_id;
               let worker_id = server.worker_id;

               let uri = Uri {
                   value: format!("worker://{template_id}/{}", worker_id.clone()),
               };

               let core = stub_raw_events::Api::new(&uri);

               core.initialize(&stub_raw_events::WorkerId{
                     name: worker_id,
                });

               dbg!("Initialised raw_events");
           },

           TimeLineOp::EqualTo(_, _, _) => {}
           TimeLineOp::GreaterThan(_, _, _) => {}
           TimeLineOp::GreaterThanOrEqual(_, _, _) => {}
           TimeLineOp::LessThan(_, _, _) => {}
           TimeLineOp::LessThanOrEqual(_, _, _) => {}
           TimeLineOp::And(_, _, _) => {}
           TimeLineOp::Or(_, _, _) => {}
           TimeLineOp::Not(_, _) => {}
           TimeLineOp::TlHasExisted(_, _, _) => {}
           TimeLineOp::TlHasExistedWithin(_, _, _, _) => {}
           TimeLineOp::TlLatestEventToState(_, _, _) => {}
           TimeLineOp::TlDurationWhere(_, _, _) => {}
           TimeLineOp::TlDurationInCurState(_, _, _) => {}
       };

        dbg!("Excuted timeline remotely");
    }
}



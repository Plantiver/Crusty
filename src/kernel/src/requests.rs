use limine::BaseRevision;
use limine::mp::RequestFlags;
use limine::request::{MpRequest, RequestsEndMarker, RequestsStartMarker};

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static MP_REQUEST: MpRequest = MpRequest::new().with_flags(RequestFlags::X2APIC);

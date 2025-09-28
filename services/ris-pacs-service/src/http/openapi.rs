use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::http::handlers::health::healthz,
        crate::http::handlers::orders::create::create_imaging_order,
        crate::http::handlers::orders::get::list_imaging_orders,
        crate::http::handlers::orders::get::get_imaging_order,
        crate::http::handlers::studies::schedule::schedule_study,
        crate::http::handlers::studies::start::start_study,
        crate::http::handlers::studies::complete::complete_study,
        crate::http::handlers::reports::get::list_reports,
        crate::http::handlers::reports::get::get_report,
        crate::http::handlers::reports::finalize::create_report,
        crate::http::handlers::reports::finalize::update_report,
        crate::http::handlers::reports::finalize::finalize_report,
    ),
    components(
        schemas(
            crate::domain::entities::imaging_order::ImagingOrder,
            crate::domain::entities::imaging_order::CreateImagingOrderRequest,
            crate::domain::entities::imaging_order::UpdateImagingOrderRequest,
            crate::domain::entities::imaging_order::ImagingOrderQuery,
            crate::domain::entities::imaging_order::ImagingOrderResponse,
            crate::domain::entities::study::ImagingStudy,
            crate::domain::entities::study::ImagingSeries,
            crate::domain::entities::study::ImagingInstance,
            crate::domain::entities::study::CreateStudyRequest,
            crate::domain::entities::study::UpdateStudyRequest,
            crate::domain::entities::study::StudyQuery,
            crate::domain::entities::study::StudyResponse,
            crate::domain::entities::report::ImagingReport,
            crate::domain::entities::report::ReportTemplate,
            crate::domain::entities::report::CreateReportRequest,
            crate::domain::entities::report::UpdateReportRequest,
            crate::domain::entities::report::FinalizeReportRequest,
            crate::domain::entities::report::ReportQuery,
            crate::domain::entities::report::ReportResponse
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "orders", description = "Imaging order management"),
        (name = "studies", description = "DICOM study management"),
        (name = "reports", description = "Imaging report management")
    )
)]
pub struct ApiDoc;
# Pickle Classes Documentation

This document describes all classes found in pickle files from: `runsets/bb/NORM/update-2025-08-04/G1`

Each class is documented once with all paths to reach it across stages 1-4.

---


## AbsoluteBoundingBox
Bounding box with absolute pixel coordinates (xc, yc, w, h)
    1 -> state -> ('center', #) -> object-detections -> detections -> # -> bbox
    1 -> state -> ('center', #) -> object-tracks-hoop -> tracklets -> # -> history -> # -> bbox
    1 -> state -> ('center', #) -> object-tracks-person -> tracklets -> # -> history -> # -> bbox
    2 -> state -> ('center', #) -> object-detections -> detections -> # -> bbox
    2 -> state -> ('center', #) -> object-tracks-hoop -> tracklets -> # -> history -> # -> bbox
    2 -> state -> ('center', #) -> object-tracks-person -> tracklets -> # -> history -> # -> bbox



## ActiveObjectMessage
Active object detection (e.g., active hoop being played on)
    1 -> state -> ('center', #) -> object-active-hoop
    2 -> state -> ('center', #) -> object-active-hoop
- **detection**: None



## BasketballCourtDimensions
Basketball court dimensions in feet (paint, 3-point line, etc.)
    1 -> state -> ('center', #) -> court-dimensions -> court_dimensions -> CourtSideType.LEFT
    2 -> state -> ('center', #) -> court-dimensions -> court_dimensions -> CourtSideType.LEFT
- **center_circle_radius**: float64
- **hoop_center_offset**: float64
- **paint_length**: float64
- **paint_width**: float64
- **restricted_area_circle_radius**: float64
- **three_point_line_radius**: float64



## CourtDimensionsMessage
Container for court dimension information by side
    1 -> state -> ('center', #) -> court-dimensions
    2 -> state -> ('center', #) -> court-dimensions
- **court_dimensions**: dict



## CourtLocalizationMessage
Court localization data including homography and court side
    1 -> state -> ('center', #) -> court-localization
    2 -> state -> ('center', #) -> court-localization
- **court_side**: **CourtSideType**
- **homography**: ndarray



## CourtSideType
Enumeration of court side views (LEFT, RIGHT, FULL)
    1 -> state -> ('center', #) -> court-localization -> court_side
    2 -> state -> ('center', #) -> court-localization -> court_side



## CourtTopViewItemsMessage
Tracklet coordinates in real-world and top-view court space
    1 -> state -> ('center', #) -> court-topview-items
    2 -> state -> ('center', #) -> court-topview-items
- **items**: List[**TopViewItem**] (length=18)



## DetectionMessage
Object detection results for target classes
    1 -> state -> ('center', #) -> object-detections
    2 -> state -> ('center', #) -> object-detections
- **detections**: List[**SVDetection**] (length=22)
- **targets**: List[str] (length=4)



## InstantTrackSample
Single frame snapshot of a tracked object (bbox, state, frame)
    1 -> state -> ('center', #) -> object-tracks-hoop -> tracklets -> # -> history -> #
    1 -> state -> ('center', #) -> object-tracks-person -> tracklets -> # -> history -> #
    2 -> state -> ('center', #) -> object-tracks-hoop -> tracklets -> # -> history -> #
    2 -> state -> ('center', #) -> object-tracks-person -> tracklets -> # -> history -> #
- **bbox**: **AbsoluteBoundingBox**
- **detection_id**: int
- **frame_nbr**: int
- **state**: str



## NumberIdentification
Jersey number identification result with confidence score
    1 -> state -> ('center', #) -> jersey-number -> numbers -> #
    2 -> state -> ('center', #) -> jersey-number -> numbers -> #
- **number**: str
- **score**: float



## NumberIdentificationMessage
Jersey number identifications indexed by track/detection id
    1 -> state -> ('center', #) -> jersey-number
    2 -> state -> ('center', #) -> jersey-number
- **numbers**: dict
- **source**: **SampleSourceEnum**



## PlayerIDAssignmentMessage
Player ID assignments mapping tracklet ids to player identifications
    3 -> state -> ('center', #) -> player-id-assigment
    4 -> state -> ('center', #) -> player-id-assigment
- **end_frame**: int
- **players_ids**: dict
- **start_frame**: int



## PointInt
Integer x,y coordinate point
    1 -> state -> ('center', #) -> pose -> poses -> # -> box_corner
    2 -> state -> ('center', #) -> pose -> poses -> # -> box_corner
- **x**: int
- **y**: int



## Pose
Pose estimation result (17 COCO keypoints relative to box corner)
    1 -> state -> ('center', #) -> pose -> poses -> #
    2 -> state -> ('center', #) -> pose -> poses -> #
- **box_corner**: **PointInt**
- **keypoints_array**: ndarray



## PosesMessage
Pose estimation results indexed by track/detection id
    1 -> state -> ('center', #) -> pose
    2 -> state -> ('center', #) -> pose
- **poses**: dict
- **source**: **SampleSourceEnum**



## RuntimeMetadataMessage
Pipeline runtime metadata (fps, frame range, dimensions)
    1 -> state -> ('center', #) -> runtime-metadata
    2 -> state -> ('center', #) -> runtime-metadata
    3 -> state -> ('center', #) -> runtime-metadata
    4 -> state -> ('center', #) -> runtime-metadata
- **fps**: float
- **frame_end**: int
- **frame_height**: int
- **frame_start**: int
- **frame_width**: int
- **total_frames**: int



## SCCourtLocalizerStatusMessage
Status of sports court localizer (working/not working)
    1 -> state -> ('center', #) -> court-localizer-status
    2 -> state -> ('center', #) -> court-localizer-status
- **working**: bool



## SVDetection
Single object detection with id, class, score, and bounding box
    1 -> state -> ('center', #) -> object-detections -> detections -> #
    2 -> state -> ('center', #) -> object-detections -> detections -> #
- **bbox**: **AbsoluteBoundingBox**
- **class_name**: str
- **id**: int
- **score**: float32



## SVTracklet
Multi-frame track of an object with history and attributes
    1 -> state -> ('center', #) -> object-tracks-hoop -> tracklets -> #
    1 -> state -> ('center', #) -> object-tracks-person -> tracklets -> #
    2 -> state -> ('center', #) -> object-tracks-hoop -> tracklets -> #
    2 -> state -> ('center', #) -> object-tracks-person -> tracklets -> #
- **attributes**: List[empty] (length=0)
- **class_name**: str
- **history**: List[**InstantTrackSample**] (length=1)
- **id**: int
- **score**: float32



## TopViewItem
Mapping between real-world coordinates and top-view court coordinates
    1 -> state -> ('center', #) -> court-topview-items -> items -> #
    2 -> state -> ('center', #) -> court-topview-items -> items -> #
- **item_id**: int
- **proj_x**: float32
- **proj_y**: float32
- **x**: int
- **y**: int



## TrackMessage
Collection of tracked objects (tracklets)
    1 -> state -> ('center', #) -> object-tracks-backboard
    1 -> state -> ('center', #) -> object-tracks-hoop
    1 -> state -> ('center', #) -> object-tracks-person
    2 -> state -> ('center', #) -> object-tracks-backboard
    2 -> state -> ('center', #) -> object-tracks-hoop
    2 -> state -> ('center', #) -> object-tracks-person
- **tracklets**: List[empty] (length=0)



REBOUNDS
repos/algorithms2/src/synch/screbounder.py
calls RebounderDetector()

repos/algorithms2/src/bball_analyze/action_tracking/rebounder_detector.py
RebounderDetector.findRebounder() Returns:
In a window of frames after a shot, find rebounder via
the average distance between the ball and left wrist and right wrist for a given player crops

repos/algorithms2/src/synch/scrbpreboundqualifier.py
if len(tracking_frames) >= self._rbpr.rebound_search_frame_range * self.search_frame_range_tol:
    result_key = (self.view, self.trigger_frame_num, ky.REBOUND_QUALIFIER)

repos/algorithms2/src/bball_analyze/region_based_processing/region_based_processing.py
RBPReboundQualifier.get_rebound_qualifier()
Determine if a rebound is offensive or defensive.
if shot attempted within rebound_search_frame_range then
    rebound was offensive if play switched ends of court, else defensive

repos/algorithms2/src/bball_analyze/region_based_processing/region_based_action_processing.py
RegionBasedActionProcessing.get_rebound_qualifier()
If players are accumulating on the side of the court with a shot then
    the rebound was offensive, else defensive

repos/algorithms2/src/bball_analyze/shots/shot_detector.py
ShotDetector.detect_shot calls _detect_shot()
_detect_shot() calls _validate_shot()
_validate_shot() calls _shot_valid_by_pre_trajectory_analysis()
_shot_valid_by_pre_trajectory_analysis() calls _find_rebounds()
_find_rebounds()
Given a trajectory, find possible rebounds and return the indexes
        on the trajectory where the rebounds happens.
if high_angle_change and speed_change:
    rebound_idxs.append(i + 1)
---------------------------------

ASSISTS

repos/algorithms2/src/synch/scassister.py
repos/algorithms2/src/bball_analyze/action_tracking/assister_detector.py
AssisterDetector.findAssister()
In a window of frames prior to a shot, find assister via
the average distance between the ball and left wrist and right wrist for a given player crops

---------------------------------

TURNOVERS

repos/algorithms2/src/synch/scrbpturnover.py
if play swtiched ends of the court without a shot attempt or
    there was a shot attempt with an offensive rebound then
    a turnover happened
[[seems to me it should be checking for a defensive rebound instead]]

---------------------------------

BLOCKS

We didn't have an algorithm to detect blocked shots because
shots were only detected if the ball approached the hoop.

---------------------------------

STEALS

We didn't have an algorithm to detect steals separate from turnovers.
Turnovers could be due to steals, but also fouls, defensive rebounds, and ball out-of-bounds

---------------------------------

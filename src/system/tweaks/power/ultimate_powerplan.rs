// https://winutil.christitus.com/dev/tweaks/performance-plans/addultperf/

use crate::{AppContext, system::tweaks::templates};

#[allow(dead_code)]
pub fn enable(ctx: &mut AppContext) {
    templates::default(ctx, "Activating Ultimate power plan...", "Successfully activated Ultimate power plan!", "Failed to activate Ultimate power plan....", "
        try {
            $ultimatePlan = powercfg -list | Select-String -Pattern \"Ultimate Performance\"
            $ultimatePlanGUID = (powercfg -list | Select-String -Pattern \"Ultimate Performance\").Line.Split()[3]
            powercfg -setactive $ultimatePlanGUID
            Write-Host \"Ultimate Performance plan is now active.\"
        } catch {
            Write-Warning $psitem.Exception.Message
        }"
    );
}

#[allow(dead_code)]
pub fn disable(ctx: &mut AppContext) {
    templates::default(ctx, "Disabling Ultimate power plan...", "Successfully disabled Ultimate power plan!", "Failed to disable Ultimate power plan...", "
        try {
            $ultimatePlan = powercfg -list | Select-String -Pattern \"Ultimate Performance\"
                if ($ultimatePlan) {
                    $ultimatePlanGUID = $ultimatePlan.Line.Split()[3]
                    $balancedPlanGUID = (powercfg -list | Select-String -Pattern \"Balanced\").Line.Split()[3]
                    powercfg -setactive $balancedPlanGUID
                    powercfg -delete $ultimatePlanGUID
                    Write-Host \"Ultimate Performance plan has been uninstalled.\"
                    Write-Host \"> Balanced plan is now active.\"
                } else {
                    Write-Host \"Ultimate Performance plan is not installed.\"
                }
        } catch {
            Write-Warning $psitem.Exception.Message
        }"
    );
}

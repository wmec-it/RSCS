// DOCS: https://winutil.christitus.com/dev/tweaks/z--advanced-tweaks---caution/disablelms1/

use crate::system::tweaks::templates;

#[allow(dead_code)]
pub fn enable() {
    // You can't...
    // Go to intel.com and install it again.
}

#[allow(dead_code)]
pub fn disable() {
    templates::default(
        "Disabling Intel MM LMS...",
        "Successfully disabled Intel MM LMS!",
        "Failed to disable Intel MM LMS...",
        "
        Write-Host \"Kill LMS\"
        $serviceName = \"LMS\"
        Write-Host \"Stopping and disabling service: $serviceName\"
        Stop-Service -Name $serviceName -Force -ErrorAction SilentlyContinue;
        Set-Service -Name $serviceName -StartupType Disabled -ErrorAction SilentlyContinue;

        Write-Host \"Removing service: $serviceName\";
        sc.exe delete $serviceName;

        Write-Host \"Removing LMS driver packages\";
        $lmsDriverPackages = Get-ChildItem -Path \"C:\\Windows\\System32\\DriverStore\\FileRepository\" -Recurse -Filter \"lms.inf*\";
        foreach ($package in $lmsDriverPackages) {
            Write-Host \"Removing driver package: $($package.Name)\";
            pnputil /delete-driver $($package.Name) /uninstall /force;
        }
        if ($lmsDriverPackages.Count -eq 0) {
            Write-Host \"No LMS driver packages found in the driver store.\";
        } else {
            Write-Host \"All found LMS driver packages have been removed.\";
        }

        Write-Host \"Searching and deleting LMS executable files\";
        $programFilesDirs = @(\"C:\\Program Files\", \"C:\\Program Files (x86)\");
        $lmsFiles = @();
        foreach ($dir in $programFilesDirs) {
            $lmsFiles += Get-ChildItem -Path $dir -Recurse -Filter \"LMS.exe\" -ErrorAction SilentlyContinue;
        }
        foreach ($file in $lmsFiles) {
            Write-Host \"Taking ownership of file: $($file.FullName)\";
            & icacls $($file.FullName) /grant Administrators:F /T /C /Q;
            & takeown /F $($file.FullName) /A /R /D Y;
            Write-Host \"Deleting file: $($file.FullName)\";
            Remove-Item $($file.FullName) -Force -ErrorAction SilentlyContinue;
        }
        if ($lmsFiles.Count -eq 0) {
            Write-Host \"No LMS.exe files found in Program Files directories.\";
        } else {
            Write-Host \"All found LMS.exe files have been deleted.\";
        }
        Write-Host 'Intel LMS vPro service has been disabled, removed, and blocked.';"
    );
}

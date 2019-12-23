use crate::reg_tweak::*;
use crate::reg_tweak::RegPathType::*;
use crate::reg_tweak::RegValData::*;
use crate::tweak_wrapper::*;

pub(crate) const DISABLE_MELTDOWN_SPECTRE: Tweak = Tweak {
  name: "DISABLE MELTDOWN SPECTRE",
  content: &RegTweak {
    path: Regular(r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management"),
    data: &[
      ("FeatureSettingsOverride", RegDword(3)),
      ("FeatureSettingsOverrideMask", RegDword(3))
    ],
  },
  desc: "\
    These tweaks disable the Meltdown and Spectre mitigation from Microsoft
which has been reported to have caused momentary freeze of games.
    See <https://us.battle.net/forums/en/d3/topic/20761026420>.",
};

pub(crate) const DISABLE_NAGLE_S_ALGORITHM: Tweak = Tweak {
  name: "DISABLE NAGLE'S ALGORITHM",
  content: &RegTweak {
    path: Wildcard(r"HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters\Interfaces\#"),
    data: &[
      ("TcpAckFrequency", RegDword(1)),
      ("TcpDelAckTicks", RegDword(0)),
      ("TCPNoDelay", RegDword(1))
    ],
  },
  desc: "\
    Nagle’s algorithm combines several small packets into a single,
larger packet for more efficient transmissions. This is designed to improve throughput
efficiency of data transmission. Disabling “nagling” can help reduce latency/ping in
some games.
    See <https://www.back2gaming.com/guides/how-to-tweak-windows-10-for-gaming>.",
};

pub(crate) const DISABLE_NETWORK_THROTTLING: Tweak = Tweak {
  name: "DISABLE NETWORK THROTTLING",
  content: &RegTweak {
    path: Regular(r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile"),
    data: &[("NetworkThrottlingIndex", RegDword(0xFFFFFFFF))],
  },
  desc: "\
    Windows implements a network throttling mechanism, the idea behind such throttling
is that processing of network packets can be a resource-intensive task. It is beneficial
to turn off such throttling for achieving maximum throughput.
    See <https://www.back2gaming.com/guides/how-to-tweak-windows-10-for-gaming>.",
};

pub(crate) const IMPROVE_SYSTEM_RESPONSIVENESS: Tweak = Tweak {
  name: "IMPROVE SYSTEM RESPONSIVENESS",
  content: &RegTweak {
    path: Regular(r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile"),
    data: &[
      ("NetworkThrottlingIndex", RegDword(0xFFFFFFFF)),
      ("NoLazyMode", RegDword(1)),
      ("SystemResponsiveness", RegDword(0))
    ],
  },
  desc: "\
    WMultimedia streaming and some games that uses “Multimedia Class Scheduler” service
(MMCSS) can only utilize up to 80% of the CPU by default.
    See <https://www.back2gaming.com/guides/how-to-tweak-windows-10-for-gaming>.",
};

pub(crate) const IMPROVE_GPU_AND_PRIORITIES: Tweak = Tweak {
  name: "IMPROVE GPU AND PRIORITIES",
  content: &RegTweak {
    path: Regular(r"HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile\Tasks\Games"),
    data: &[
      ("Affinity", RegDword(0)),
      ("GPU Priority", RegDword(31)),
      ("Priority", RegDword(6)),
      ("Scheduling Category", RegSz("High")),
      ("SFIO Priority", RegSz("High"))
    ],
  },
  desc: "\
    Adjust priorities of CPU and GPU resources for gaming.
    See <https://github.com/CHEF-KOCH/GamingTweaks/issues/5>.",
};
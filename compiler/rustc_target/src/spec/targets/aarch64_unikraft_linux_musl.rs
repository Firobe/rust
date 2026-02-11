use crate::spec::{
    Arch, FramePointer, StackProbeType, Target, TargetMetadata, TargetOptions, base,
};

pub(crate) fn target() -> Target {
    Target {
        llvm_target: "aarch64-unknown-linux-musl".into(),
        metadata: TargetMetadata {
            description: Some("ARM 64-bit Unikraft with musl 1.2.5".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 64,
        arch: Arch::AArch64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-Fn32".into(),
        options: TargetOptions {
            plt_by_default: false,
            max_atomic_width: Some(128),
            supports_xray: true,
            features: "+v8a,+outline-atomics".into(),
            stack_probes: StackProbeType::Inline,
            frame_pointer: FramePointer::NonLeaf,
            mcount: "\u{1}_mcount".into(),
            ..base::unikraft_linux_musl::opts()
        },
    }
}

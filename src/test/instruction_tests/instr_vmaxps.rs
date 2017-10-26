use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmaxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 95, 212], OperandSize::Dword)
}

#[test]
fn vmaxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 95, 20, 78], OperandSize::Dword)
}

#[test]
fn vmaxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 95, 192], OperandSize::Qword)
}

#[test]
fn vmaxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 70278099, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 95, 52, 189, 211, 91, 48, 4], OperandSize::Qword)
}

#[test]
fn vmaxps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 95, 214], OperandSize::Dword)
}

#[test]
fn vmaxps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 95, 32], OperandSize::Dword)
}

#[test]
fn vmaxps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 95, 239], OperandSize::Qword)
}

#[test]
fn vmaxps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 95, 10], OperandSize::Qword)
}

#[test]
fn vmaxps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 142, 95, 194], OperandSize::Dword)
}

#[test]
fn vmaxps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ESI, 869802080, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 142, 95, 158, 96, 32, 216, 51], OperandSize::Dword)
}

#[test]
fn vmaxps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(EDI, 510676782, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 68, 155, 95, 151, 46, 79, 112, 30], OperandSize::Dword)
}

#[test]
fn vmaxps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 33, 100, 133, 95, 241], OperandSize::Qword)
}

#[test]
fn vmaxps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledIndexed(RDI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 4, 130, 95, 60, 215], OperandSize::Qword)
}

#[test]
fn vmaxps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1337733967, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 12, 153, 95, 20, 133, 79, 51, 188, 79], OperandSize::Qword)
}

#[test]
fn vmaxps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 170, 95, 198], OperandSize::Dword)
}

#[test]
fn vmaxps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Four, 979651576, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 68, 169, 95, 132, 154, 248, 75, 100, 58], OperandSize::Dword)
}

#[test]
fn vmaxps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 7803729, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 189, 95, 137, 81, 19, 119, 0], OperandSize::Dword)
}

#[test]
fn vmaxps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM12)), operand3: Some(Direct(YMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 28, 170, 95, 249], OperandSize::Qword)
}

#[test]
fn vmaxps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RCX, 1440859867, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 28, 166, 95, 161, 219, 198, 225, 85], OperandSize::Qword)
}

#[test]
fn vmaxps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 108, 190, 95, 30], OperandSize::Qword)
}

#[test]
fn vmaxps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 158, 95, 255], OperandSize::Dword)
}

#[test]
fn vmaxps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(ECX, 1372230324, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 76, 202, 95, 137, 180, 146, 202, 81], OperandSize::Dword)
}

#[test]
fn vmaxps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 222, 95, 50], OperandSize::Dword)
}

#[test]
fn vmaxps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 44, 150, 95, 245], OperandSize::Qword)
}

#[test]
fn vmaxps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM27)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 36, 199, 95, 25], OperandSize::Qword)
}

#[test]
fn vmaxps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMAXPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RAX, 1820695290, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 4, 210, 95, 136, 250, 154, 133, 108], OperandSize::Qword)
}


use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsubadd213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 167, 219], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 560739735, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 167, 135, 151, 53, 108, 33], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 167, 215], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1719248291, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 167, 44, 181, 163, 165, 121, 102], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 167, 221], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, ECX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 167, 52, 143], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 167, 239], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 167, 52, 207], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 143, 167, 219], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 205, 140, 167, 16], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 157, 167, 18], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 141, 167, 236], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 237, 141, 167, 56], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 155, 167, 52, 185], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 172, 167, 228], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 167, 60, 223], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 1466186136, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 221, 188, 167, 186, 152, 57, 100, 87], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 213, 166, 167, 228], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1558015616, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 221, 164, 167, 36, 189, 128, 110, 221, 92], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RAX, 2068130550, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 157, 181, 167, 184, 246, 42, 69, 123], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 229, 222, 167, 241], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Two, 2118900589, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 167, 140, 65, 109, 219, 75, 126], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Four, 1621808972, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 237, 222, 167, 52, 141, 76, 215, 170, 96], OperandSize::Dword)
}

#[test]
fn vfmsubadd213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 209, 167, 208], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 644795199, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 229, 199, 167, 60, 245, 63, 203, 110, 38], OperandSize::Qword)
}

#[test]
fn vfmsubadd213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUBADD213PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1444273114, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 149, 221, 167, 28, 133, 218, 219, 21, 86], OperandSize::Qword)
}


use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 194], OperandSize::Dword)
}

#[test]
fn vsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 112454328, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 156, 250, 184, 234, 179, 6], OperandSize::Dword)
}

#[test]
fn vsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 214], OperandSize::Qword)
}

#[test]
fn vsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 31], OperandSize::Qword)
}

#[test]
fn vsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 200], OperandSize::Dword)
}

#[test]
fn vsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 44, 66], OperandSize::Dword)
}

#[test]
fn vsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 215], OperandSize::Qword)
}

#[test]
fn vsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RBX, 1415393813, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 139, 21, 50, 93, 84], OperandSize::Qword)
}

#[test]
fn vsqrtps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 137, 81, 238], OperandSize::Dword)
}

#[test]
fn vsqrtps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 91679299, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 81, 12, 157, 67, 234, 118, 5], OperandSize::Dword)
}

#[test]
fn vsqrtps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 157, 81, 34], OperandSize::Dword)
}

#[test]
fn vsqrtps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 124, 140, 81, 194], OperandSize::Qword)
}

#[test]
fn vsqrtps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 124, 140, 81, 36, 203], OperandSize::Qword)
}

#[test]
fn vsqrtps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM27)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 124, 153, 81, 27], OperandSize::Qword)
}

#[test]
fn vsqrtps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 81, 197], OperandSize::Dword)
}

#[test]
fn vsqrtps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1989176152, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 81, 28, 77, 88, 107, 144, 118], OperandSize::Dword)
}

#[test]
fn vsqrtps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 185, 81, 44, 65], OperandSize::Dword)
}

#[test]
fn vsqrtps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 124, 171, 81, 205], OperandSize::Qword)
}

#[test]
fn vsqrtps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 676265203, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 124, 175, 81, 28, 213, 243, 252, 78, 40], OperandSize::Qword)
}

#[test]
fn vsqrtps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Two, 451062345, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 187, 81, 188, 74, 73, 170, 226, 26], OperandSize::Qword)
}

#[test]
fn vsqrtps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 190, 81, 192], OperandSize::Dword)
}

#[test]
fn vsqrtps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 81, 60, 86], OperandSize::Dword)
}

#[test]
fn vsqrtps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 219, 81, 20, 91], OperandSize::Dword)
}

#[test]
fn vsqrtps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 81, 124, 220, 81, 211], OperandSize::Qword)
}

#[test]
fn vsqrtps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectDisplaced(RCX, 896025643, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 124, 206, 81, 177, 43, 68, 104, 53], OperandSize::Qword)
}

#[test]
fn vsqrtps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 223, 81, 12, 222], OperandSize::Qword)
}


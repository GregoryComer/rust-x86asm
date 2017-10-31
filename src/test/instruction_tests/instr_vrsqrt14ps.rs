use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 78, 215], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 78, 39], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 522051346, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 78, 28, 117, 18, 223, 29, 31], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 125, 139, 78, 212], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 78, 16], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 297774216, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 154, 78, 36, 85, 136, 172, 191, 17], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 78, 211], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1315117602, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 78, 140, 143, 34, 26, 99, 78], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 190, 78, 4, 115], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 66, 125, 169, 78, 194], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(RDI, 1635107505, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 169, 78, 151, 177, 194, 117, 97], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM11)), operand2: Some(IndirectDisplaced(RBX, 150033643, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 187, 78, 155, 235, 84, 241, 8], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 78, 253], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 932084809, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 78, 36, 149, 73, 124, 142, 55], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Eight, 618384289, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 218, 78, 164, 210, 161, 203, 219, 36], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 194, 125, 204, 78, 241], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectDisplaced(RSI, 876076180, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 202, 78, 134, 148, 220, 55, 52], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 647687038, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 221, 78, 12, 125, 126, 235, 154, 38], OperandSize::Qword)
}


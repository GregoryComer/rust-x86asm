use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 78, 224], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 799510546, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 78, 60, 157, 18, 144, 167, 47], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1145585878, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 155, 78, 20, 85, 214, 64, 72, 68], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 50, 253, 143, 78, 250], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 77164494, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 253, 143, 78, 44, 213, 206, 111, 153, 4], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM26)), operand2: Some(IndirectDisplaced(RBX, 1115856387, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 157, 78, 147, 3, 158, 130, 66], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 78, 205], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 78, 30], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 185461895, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 191, 78, 4, 117, 135, 236, 13, 11], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 50, 253, 173, 78, 220], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM24)), operand2: Some(IndirectDisplaced(RAX, 152414920, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 173, 78, 128, 200, 170, 21, 9], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM11)), operand2: Some(IndirectScaledDisplaced(RCX, Two, 386624142, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 189, 78, 28, 77, 142, 106, 11, 23], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 78, 244], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 78, 14], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 278802133, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 78, 20, 133, 213, 46, 158, 16], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 253, 203, 78, 220], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM14)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 788754888, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 253, 206, 78, 180, 65, 200, 113, 3, 47], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 2117616365, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 217, 78, 164, 195, 237, 66, 56, 126], OperandSize::Qword)
}


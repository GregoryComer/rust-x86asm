use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 78, 252], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 1021255404, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 78, 163, 236, 30, 223, 60], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 158, 78, 40], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 253, 141, 78, 219], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM26)), operand2: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 253, 139, 78, 18], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 78, 22], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 78, 233], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 78, 40], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 796179360, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 78, 28, 85, 160, 187, 116, 47], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 253, 175, 78, 199], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 78, 2], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM27)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 965949927, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 187, 78, 156, 243, 231, 57, 147, 57], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 78, 238], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1204473625, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 78, 156, 134, 25, 207, 202, 71], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Eight, 1721669164, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 221, 78, 164, 209, 44, 150, 158, 102], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 253, 205, 78, 196], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM12)), operand2: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 201, 78, 35], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 1220065568, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 219, 78, 4, 197, 32, 185, 184, 72], OperandSize::Qword)
}


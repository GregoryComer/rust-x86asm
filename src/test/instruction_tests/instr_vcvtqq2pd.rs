use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 230, 215], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 138, 230, 36, 70], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 209, 254, 142, 230, 218], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(XMM15)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 890249877, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 138, 230, 60, 85, 149, 34, 16, 53], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 172, 230, 197], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1182544517, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 254, 170, 230, 140, 199, 133, 50, 124, 70], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 254, 171, 230, 235], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(YMM9)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 254, 174, 230, 12, 135], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 189, 230, 214], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1253118363, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 230, 12, 253, 155, 17, 177, 74], OperandSize::Dword)
}

#[test]
fn vcvtqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 254, 223, 230, 239], OperandSize::Qword)
}

#[test]
fn vcvtqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTQQ2PD, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1167376707, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 254, 205, 230, 4, 205, 67, 193, 148, 69], OperandSize::Qword)
}


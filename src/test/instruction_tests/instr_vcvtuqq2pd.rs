use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtuqq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 143, 122, 250], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 1030562159, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 254, 139, 122, 163, 111, 33, 109, 61], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 254, 142, 122, 244], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RAX, 1349140701, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 137, 122, 152, 221, 64, 106, 80], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 122, 254], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 346985333, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 172, 122, 164, 193, 117, 147, 174, 20], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 254, 175, 122, 245], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(YMM11)), operand2: Some(IndirectDisplaced(RAX, 1554001005, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 254, 170, 122, 152, 109, 44, 160, 92], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 221, 122, 210], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 1108649471, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 122, 132, 243, 255, 165, 20, 66], OperandSize::Dword)
}

#[test]
fn vcvtuqq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 254, 219, 122, 223], OperandSize::Qword)
}

#[test]
fn vcvtuqq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PD, operand1: Some(Direct(ZMM18)), operand2: Some(IndirectDisplaced(RDI, 93346456, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 254, 203, 122, 151, 152, 90, 144, 5], OperandSize::Qword)
}


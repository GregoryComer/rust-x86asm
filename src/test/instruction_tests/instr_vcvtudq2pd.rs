use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 122, 212], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 122, 8], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 49, 126, 140, 122, 237], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 126, 139, 122, 44, 154], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 122, 201], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 126, 173, 122, 15], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 126, 174, 122, 204], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 126, 172, 122, 28, 198], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 122, 199], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM1)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 126, 202, 122, 12, 176], OperandSize::Dword)
}

#[test]
fn vcvtudq2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 126, 201, 122, 225], OperandSize::Qword)
}

#[test]
fn vcvtudq2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PD, operand1: Some(Direct(ZMM5)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 122, 40], OperandSize::Qword)
}


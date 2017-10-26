use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 137, 122, 207], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(EDX, 1691389377, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 142, 122, 146, 193, 141, 208, 100], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 127, 140, 122, 223], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM25)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 127, 143, 122, 12, 248], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 175, 122, 219], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 170, 122, 27], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 127, 173, 122, 237], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM13)), operand2: Some(IndirectDisplaced(RDX, 1768505464, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 127, 170, 122, 170, 120, 64, 105, 105], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 255, 122, 229], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(EDI, 308496058, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 202, 122, 135, 186, 70, 99, 18], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 49, 127, 255, 122, 221], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM11)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 127, 201, 122, 28, 209], OperandSize::Qword)
}


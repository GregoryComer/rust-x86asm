use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 142, 122, 214], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 769740000, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 143, 122, 28, 125, 224, 76, 225, 45], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 145, 127, 138, 122, 192], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 1623133832, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 127, 142, 122, 20, 69, 136, 14, 191, 96], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 122, 197], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Eight, 907157836, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 172, 122, 60, 245, 76, 33, 18, 54], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 97, 127, 170, 122, 231], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM25)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 127, 172, 122, 12, 87], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 219, 122, 211], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1961935800, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 205, 122, 164, 218, 184, 195, 240, 116], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM11)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 193, 127, 251, 122, 211], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 579251669, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 207, 122, 36, 189, 213, 173, 134, 34], OperandSize::Qword)
}


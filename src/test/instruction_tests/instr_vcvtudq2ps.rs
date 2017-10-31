use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtudq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 138, 122, 251], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 140, 122, 4, 83], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 81, 127, 139, 122, 224], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 1484352179, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 127, 137, 122, 20, 157, 179, 106, 121, 88], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 169, 122, 198], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 127, 173, 122, 42], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 127, 170, 122, 209], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(YMM24)), operand2: Some(IndirectDisplaced(RDX, 598896822, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 127, 175, 122, 130, 182, 112, 178, 35], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 159, 122, 203], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 204, 122, 28, 247], OperandSize::Dword)
}

#[test]
fn vcvtudq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 81, 127, 223, 122, 238], OperandSize::Qword)
}

#[test]
fn vcvtudq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUDQ2PS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 694157530, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 127, 206, 122, 44, 125, 218, 0, 96, 41], OperandSize::Qword)
}


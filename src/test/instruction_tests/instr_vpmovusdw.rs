use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 19, 223], OperandSize::Dword)
}

#[test]
fn vpmovusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(EAX, 171256785, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 19, 168, 209, 43, 53, 10], OperandSize::Dword)
}

#[test]
fn vpmovusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 126, 142, 19, 247], OperandSize::Qword)
}

#[test]
fn vpmovusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1748867477, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 19, 28, 197, 149, 153, 61, 104], OperandSize::Qword)
}

#[test]
fn vpmovusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 19, 238], OperandSize::Dword)
}

#[test]
fn vpmovusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 19, 6], OperandSize::Dword)
}

#[test]
fn vpmovusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM24)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 126, 169, 19, 224], OperandSize::Qword)
}

#[test]
fn vpmovusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(RAX, 1995578149, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 19, 152, 37, 27, 242, 118], OperandSize::Qword)
}

#[test]
fn vpmovusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 19, 250], OperandSize::Dword)
}

#[test]
fn vpmovusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1792593128, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 19, 140, 250, 232, 204, 216, 106], OperandSize::Dword)
}

#[test]
fn vpmovusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 202, 19, 250], OperandSize::Qword)
}

#[test]
fn vpmovusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 19, 43], OperandSize::Qword)
}


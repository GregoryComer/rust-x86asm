use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 32, 223], OperandSize::Dword)
}

#[test]
fn vpmovswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 2123945553, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 32, 180, 131, 81, 214, 152, 126], OperandSize::Dword)
}

#[test]
fn vpmovswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 126, 138, 32, 204], OperandSize::Qword)
}

#[test]
fn vpmovswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 32, 4, 206], OperandSize::Qword)
}

#[test]
fn vpmovswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 169, 32, 218], OperandSize::Dword)
}

#[test]
fn vpmovswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 501715425, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 32, 188, 222, 225, 145, 231, 29], OperandSize::Dword)
}

#[test]
fn vpmovswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 126, 174, 32, 251], OperandSize::Qword)
}

#[test]
fn vpmovswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 32, 41], OperandSize::Qword)
}

#[test]
fn vpmovswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 32, 232], OperandSize::Dword)
}

#[test]
fn vpmovswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectDisplaced(ESI, 1379553966, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 32, 134, 174, 82, 58, 82], OperandSize::Dword)
}

#[test]
fn vpmovswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 126, 205, 32, 205], OperandSize::Qword)
}

#[test]
fn vpmovswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledDisplaced(RAX, Four, 460057536, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 32, 44, 133, 192, 235, 107, 27], OperandSize::Qword)
}


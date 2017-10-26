use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 50, 221], OperandSize::Dword)
}

#[test]
fn vpmovqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectDisplaced(EBX, 1922993015, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 50, 171, 119, 139, 158, 114], OperandSize::Dword)
}

#[test]
fn vpmovqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 126, 140, 50, 195], OperandSize::Qword)
}

#[test]
fn vpmovqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 50, 9], OperandSize::Qword)
}

#[test]
fn vpmovqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 50, 203], OperandSize::Dword)
}

#[test]
fn vpmovqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexed(EBX, ESI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 50, 20, 243], OperandSize::Dword)
}

#[test]
fn vpmovqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM22)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 34, 126, 170, 50, 246], OperandSize::Qword)
}

#[test]
fn vpmovqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 50, 43], OperandSize::Qword)
}

#[test]
fn vpmovqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 50, 237], OperandSize::Dword)
}

#[test]
fn vpmovqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 982457104, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 50, 12, 197, 16, 27, 143, 58], OperandSize::Dword)
}

#[test]
fn vpmovqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 126, 204, 50, 231], OperandSize::Qword)
}

#[test]
fn vpmovqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexed(RBX, RCX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 50, 12, 75], OperandSize::Qword)
}


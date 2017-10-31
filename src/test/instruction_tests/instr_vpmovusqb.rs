use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 18, 209], OperandSize::Dword)
}

#[test]
fn vpmovusqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1974487438, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 18, 180, 184, 142, 73, 176, 117], OperandSize::Dword)
}

#[test]
fn vpmovusqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 126, 138, 18, 239], OperandSize::Qword)
}

#[test]
fn vpmovusqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 18, 20, 81], OperandSize::Qword)
}

#[test]
fn vpmovusqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 18, 226], OperandSize::Dword)
}

#[test]
fn vpmovusqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(ESI, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 18, 52, 206], OperandSize::Dword)
}

#[test]
fn vpmovusqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 126, 169, 18, 233], OperandSize::Qword)
}

#[test]
fn vpmovusqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Four, 243221079, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 18, 132, 145, 87, 66, 127, 14], OperandSize::Qword)
}

#[test]
fn vpmovusqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 18, 200], OperandSize::Dword)
}

#[test]
fn vpmovusqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 18, 20, 246], OperandSize::Dword)
}

#[test]
fn vpmovusqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 126, 207, 18, 241], OperandSize::Qword)
}

#[test]
fn vpmovusqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 18, 12, 155], OperandSize::Qword)
}


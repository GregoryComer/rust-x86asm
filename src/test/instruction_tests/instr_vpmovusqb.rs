use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 18, 247], OperandSize::Dword)
}

#[test]
fn vpmovusqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1984379497, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 18, 140, 79, 105, 58, 71, 118], OperandSize::Dword)
}

#[test]
fn vpmovusqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 126, 142, 18, 245], OperandSize::Qword)
}

#[test]
fn vpmovusqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectDisplaced(RDX, 929518562, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 18, 154, 226, 83, 103, 55], OperandSize::Qword)
}

#[test]
fn vpmovusqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 18, 230], OperandSize::Dword)
}

#[test]
fn vpmovusqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 18, 31], OperandSize::Dword)
}

#[test]
fn vpmovusqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 66, 126, 170, 18, 227], OperandSize::Qword)
}

#[test]
fn vpmovusqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 919908083, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 18, 188, 86, 243, 174, 212, 54], OperandSize::Qword)
}

#[test]
fn vpmovusqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 18, 238], OperandSize::Dword)
}

#[test]
fn vpmovusqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 18, 7], OperandSize::Dword)
}

#[test]
fn vpmovusqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 126, 202, 18, 233], OperandSize::Qword)
}

#[test]
fn vpmovusqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQB, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 72, 18, 49], OperandSize::Qword)
}


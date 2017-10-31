use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovqb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 139, 50, 248], OperandSize::Dword)
}

#[test]
fn vpmovqb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1073827807, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 50, 60, 141, 223, 79, 1, 64], OperandSize::Dword)
}

#[test]
fn vpmovqb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 126, 137, 50, 193], OperandSize::Qword)
}

#[test]
fn vpmovqb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(Direct(XMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 50, 24], OperandSize::Qword)
}

#[test]
fn vpmovqb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 50, 247], OperandSize::Dword)
}

#[test]
fn vpmovqb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectDisplaced(EDI, 2049992844, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 50, 167, 140, 104, 48, 122], OperandSize::Dword)
}

#[test]
fn vpmovqb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM18)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 34, 126, 172, 50, 194], OperandSize::Qword)
}

#[test]
fn vpmovqb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 50, 55], OperandSize::Qword)
}

#[test]
fn vpmovqb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 50, 201], OperandSize::Dword)
}

#[test]
fn vpmovqb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 2137824314, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 50, 140, 206, 58, 156, 108, 127], OperandSize::Dword)
}

#[test]
fn vpmovqb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(Direct(XMM27)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 18, 126, 201, 50, 227], OperandSize::Qword)
}

#[test]
fn vpmovqb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVQB, operand1: Some(IndirectScaledIndexed(RCX, RSI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 50, 28, 177], OperandSize::Qword)
}


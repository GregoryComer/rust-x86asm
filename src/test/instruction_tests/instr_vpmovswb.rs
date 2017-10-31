use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 32, 231], OperandSize::Dword)
}

#[test]
fn vpmovswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 443959449, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 32, 188, 91, 153, 72, 118, 26], OperandSize::Dword)
}

#[test]
fn vpmovswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 126, 142, 32, 225], OperandSize::Qword)
}

#[test]
fn vpmovswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectDisplaced(RDI, 1004304539, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 32, 135, 155, 120, 220, 59], OperandSize::Qword)
}

#[test]
fn vpmovswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 32, 250], OperandSize::Dword)
}

#[test]
fn vpmovswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 1469010917, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 32, 140, 144, 229, 83, 143, 87], OperandSize::Dword)
}

#[test]
fn vpmovswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 126, 169, 32, 252], OperandSize::Qword)
}

#[test]
fn vpmovswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 32, 20, 64], OperandSize::Qword)
}

#[test]
fn vpmovswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 32, 207], OperandSize::Dword)
}

#[test]
fn vpmovswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1921206278, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 32, 28, 245, 6, 72, 131, 114], OperandSize::Dword)
}

#[test]
fn vpmovswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Direct(YMM29)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 126, 207, 32, 245], OperandSize::Qword)
}

#[test]
fn vpmovswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSWB, operand1: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 32, 8], OperandSize::Qword)
}

